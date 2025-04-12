//! This runtime creates a thread to manage OBS, so it can be used across threads

use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::{fmt::Debug, thread::JoinHandle};
use tokio::sync::{oneshot, Mutex};

use crate::{
    bootstrap::ObsBootstrap,
    context::{ObsContext, OBS_THREAD_ID},
    utils::StartupInfo,
};

// Command type for operations to perform on the OBS thread
enum ObsCommand {
    Execute(Box<dyn FnOnce(&mut ObsContext) -> anyhow::Result<()> + Send>),
    ExecuteWithResult(
        Box<dyn FnOnce(&mut ObsContext) -> anyhow::Result<Box<dyn std::any::Any + Send>> + Send>,
        oneshot::Sender<anyhow::Result<Box<dyn std::any::Any + Send>>>,
    ),
    Terminate,
}

#[derive(Debug)]
pub struct ObsRuntimeOptions {
    #[cfg(feature = "bootstrapper")]
    bootstrap_handler: Option<Box<dyn crate::bootstrap::status_handler::ObsBootstrapStatusHandler>>,
    #[cfg(feature = "bootstrapper")]
    bootstrapper_options: crate::bootstrap::ObsBootstrapperOptions,
    startup_info: StartupInfo,
}

impl ObsRuntimeOptions {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "bootstrapper")]
            bootstrap_handler: None,
            #[cfg(feature = "bootstrapper")]
            bootstrapper_options: Default::default(),
            startup_info: StartupInfo::default(),
        }
    }

    #[cfg(feature = "bootstrapper")]
    pub fn enable_bootstrapper<T>(
        mut self,
        handler: T,
        options: crate::bootstrap::ObsBootstrapperOptions,
    ) -> Self
    where
        T: crate::bootstrap::status_handler::ObsBootstrapStatusHandler + 'static,
    {
        self.bootstrap_handler = Some(Box::new(handler));
        self.bootstrapper_options = options;
        self
    }

    pub fn startup_info(mut self, startup_info: StartupInfo) -> Self {
        self.startup_info = startup_info;
        self
    }

    pub async fn start(self) -> anyhow::Result<ObsRuntimeReturn> {
        ObsRuntime::startup(self).await
    }
}

#[cfg(feature = "bootstrapper")]
pub enum ObsRuntimeReturn {
    /// The OBS context is ready to use
    Done(ObsRuntime),

    /// The application must be restarted to apply OBS updates
    Restart,
}

#[cfg(not(feature = "bootstrapper"))]
pub type ObsRuntimeReturn = ObsRuntime;

#[derive(Clone)]
pub struct ObsRuntime {
    handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    command_sender: Arc<Sender<ObsCommand>>,
}

impl ObsRuntime {
    //! This runtime creates a thread to manage OBS, so it can be used across threads.
    //! Will startup OBS when this function is called.

    pub fn new() -> ObsRuntimeOptions {
        ObsRuntimeOptions::new()
    }

    async fn startup(mut options: ObsRuntimeOptions) -> anyhow::Result<ObsRuntimeReturn> {
        let obs_id = OBS_THREAD_ID.lock().await;
        if obs_id.is_some() {
            return Err(anyhow::anyhow!("OBS is already running"));
        }

        drop(obs_id);

        #[cfg(not(feature = "bootstrapper"))]
        return Ok(ObsRuntime::init(options.startup_info).await?);

        #[cfg(feature = "bootstrapper")]
        if options.bootstrap_handler.is_some() {
            use crate::bootstrap::BootstrapStatus;
            use futures_util::{pin_mut, StreamExt};

            log::trace!("Starting bootstrapper");
            let stream = ObsContext::bootstrap(options.bootstrapper_options).await?;
            if let Some(stream) = stream {
                pin_mut!(stream);

                log::trace!("Waiting for bootstrapper to finish");
                while let Some(item) = stream.next().await {
                    match item {
                        BootstrapStatus::Downloading(progress, message) => {
                            if let Some(handler) = &mut options.bootstrap_handler {
                                handler.handle_downloading(progress, message).await?;
                            }
                        }
                        BootstrapStatus::Extracting(progress, message) => {
                            if let Some(handler) = &mut options.bootstrap_handler {
                                handler.handle_extraction(progress, message).await?;
                            }
                        }
                        BootstrapStatus::Error(err) => {
                            return Err(err);
                        }
                        BootstrapStatus::RestartRequired => {
                            return Ok(ObsRuntimeReturn::Restart);
                        }
                    }
                }
            }
        }

        log::trace!("Initializing OBS context");
        return Ok(ObsRuntimeReturn::Done(
            ObsRuntime::init(options.startup_info).await?,
        ));
    }

    async fn init(info: StartupInfo) -> anyhow::Result<Self> {
        let (command_sender, command_receiver) = channel();
        let (init_tx, init_rx) = oneshot::channel();

        let handle = std::thread::spawn(move || {
            log::trace!("Starting OBS thread");
            // Initialize OBS on this thread
            let res = ObsContext::new(info);
            match res {
                Ok(mut context) => {
                    log::trace!("OBS context initialized successfully");
                    let e = init_tx.send(Ok(()));
                    if let Err(err) = e {
                        log::error!("Failed to send initialization signal: {:?}", err);
                    }

                    // Process commands until termination
                    while let Ok(command) = command_receiver.recv() {
                        match command {
                            ObsCommand::Execute(func) => {
                                let _ = func(&mut context);
                            }
                            ObsCommand::ExecuteWithResult(func, result_sender) => {
                                let result = func(&mut context);
                                let _ = result_sender.send(result);
                            }
                            ObsCommand::Terminate => break,
                        }
                    }
                }
                Err(err) => {
                    log::error!("Failed to initialize OBS context: {:?}", err);
                    let _ = init_tx.send(Err(err));
                }
            }
        });

        log::trace!("Waiting for OBS thread to initialize");
        // Wait for initialization to complete
        init_rx.await??;

        Ok(Self {
            handle: Arc::new(Mutex::new(Some(handle))),
            command_sender: Arc::new(command_sender),
        })
    }

    /// Run a function with the ObsContext
    ///
    /// This allows you to execute operations on the OBS thread that don't return a value
    pub fn run_with_obs<F>(&self, operation: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut ObsContext) -> anyhow::Result<()> + Send + 'static,
    {
        self.command_sender
            .send(ObsCommand::Execute(Box::new(operation)))
            .map_err(|_| anyhow::anyhow!("Failed to send command to OBS thread"))?;
        Ok(())
    }

    /// Run a function with the ObsContext and get a result
    ///
    /// This allows you to execute operations on the OBS thread and get a value back
    pub async fn run_with_obs_result<F, T>(&self, operation: F) -> anyhow::Result<T>
    where
        F: FnOnce(&mut ObsContext) -> anyhow::Result<T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        // Create a wrapper closure that boxes the result as Any
        let wrapper = move |ctx: &mut ObsContext| -> anyhow::Result<Box<dyn std::any::Any + Send>> {
            let result = operation(ctx)?;
            Ok(Box::new(result))
        };

        self.command_sender
            .send(ObsCommand::ExecuteWithResult(Box::new(wrapper), tx))
            .map_err(|_| anyhow::anyhow!("Failed to send command to OBS thread"))?;

        let result = rx
            .await
            .map_err(|_| anyhow::anyhow!("OBS thread dropped the response channel"))??;

        // Downcast the Any type back to T
        result
            .downcast::<T>()
            .map(|boxed| *boxed)
            .map_err(|_| anyhow::anyhow!("Failed to downcast result to the expected type"))
    }

    /// Shutdown the OBS runtime and terminate the thread
    pub async fn shutdown(self) -> anyhow::Result<()> {
        self.command_sender
            .send(ObsCommand::Terminate)
            .map_err(|_| anyhow::anyhow!("Failed to send termination command to OBS thread"))?;

        // Wait for the thread to finish
        let mut handle = self.handle.lock().await;
        let handle = handle.take().expect("Handle can not be empty");

        if let Err(err) = handle.join() {
            return Err(anyhow::anyhow!("OBS thread panicked: {:?}", err));
        }
        Ok(())
    }
}

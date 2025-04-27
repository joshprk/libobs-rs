use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;
use tokio::sync::RwLock;

use crate::{data::output::ObsOutputRef, enums::ObsOutputStopSignal};

pub type OutputSignalType = (String, ObsOutputStopSignal);
lazy_static! {
    pub static ref OUTPUT_SIGNALS: RwLock<(Sender<OutputSignalType>, Receiver<OutputSignalType>)> =
        RwLock::new(unbounded());
    static ref SIGNALS: RwLock<Vec<OutputSignalType>> = RwLock::new(vec![]);
}

pub async fn rec_output_signal(output: &ObsOutputRef) -> Result<ObsOutputStopSignal> {
    let receiver = &OUTPUT_SIGNALS.read().await.1;
    let mut s = SIGNALS
        .write()
        .await;

    while let Some(e) = receiver.try_recv().ok() {
        s.push(e);
    }

    for i in 0..s.len() {
        if s[i].0 == output.name().to_string() {
            let s = s.remove(i);
            return Ok(s.1);
        }
    }

    Ok(receiver.recv()?.1)
}

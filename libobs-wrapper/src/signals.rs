
use std::sync::{Mutex, RwLock};

use anyhow::{anyhow, Result};
use crossbeam_channel::{unbounded, Receiver, Sender};
use lazy_static::lazy_static;

use crate::{data::output::ObsOutput, enums::ObsOutputSignal};

pub type OutputSignalType = (String, ObsOutputSignal);
lazy_static! {
    pub static ref OUTPUT_SIGNALS: RwLock<(Sender<OutputSignalType>, Receiver<OutputSignalType>)> =
        RwLock::new(unbounded());
}

static SIGNALS: Mutex<Vec<OutputSignalType>> = Mutex::new(vec![]);

pub fn rec_output_signal(output: &ObsOutput) -> Result<ObsOutputSignal> {
    let receiver = &OUTPUT_SIGNALS.read().unwrap().1;

    let s = &mut SIGNALS
        .lock()
        .map_err(|e| anyhow!("Failed to lock SIGNALS: {}", e))?;
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
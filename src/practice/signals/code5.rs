use anyhow::Result;
use crossbeam_channel::{bounded, select, tick, Receiver};
use std::time::{Duration, Instant};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(1);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<()> {
    let ctrl_c_events: Receiver<()> = ctrl_channel()?;
    let ticks: Receiver<Instant> = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrl_c_events) -> _ => {
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}

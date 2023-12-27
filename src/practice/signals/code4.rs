use libc::SIGINT;
use std::error::Error;
use std::{thread, time::Duration};
use signal_hook::iterator::{Signals, SignalsInfo};

fn main() -> Result<(), Box<dyn Error>> {
    let mut signal: SignalsInfo = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signal.forever() {
            println!("Received signal {:?}", sig);
        }
    });

    // Following code does the actual work, and can be interrupted by pressing
    // Ctrl-C. As an example: Let's wait a few seconds.
    thread::sleep(Duration::from_secs(2));

    Ok(())
}

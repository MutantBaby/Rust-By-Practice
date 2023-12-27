use tokio::signal::{
    self,
    unix::{signal, Signal, SignalKind},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sigint: Signal = signal(SignalKind::interrupt())?;

    match sigint.recv().await {
        Some(()) => println!("Received SIGINT signal"),
        None => eprintln!("Stream terminated before receiving SIGINT signal"),
    }

    for num in 0..100 {
        println!("{}", num)
    }

    Ok(())
}

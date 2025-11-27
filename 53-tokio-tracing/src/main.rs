use tracing::instrument;

use tokio::sync::mpsc;

use tokio::{time::Duration, time::sleep};

#[tokio::main]
async fn main() {
    console_subscriber::init(); // This is what the instrumentation

    let (tx, mut rx) = mpsc::channel::<i32>(1);
    let prod = tokio::spawn(producer(tx));
    let con = tokio::spawn(async move { consumer(&mut rx).await });

    let _ = tokio::join!(prod, con);
}

#[instrument(name="producer-task",skip(tx))] // in creates a span automatically
async fn producer(tx: mpsc::Sender<i32>) {
    let mut i = 0;
    loop {
        tx.send(i).await.unwrap();
       // sleep(Duration::from_millis(5)).await;
        i += 1;
    }
}

#[instrument(name="consumer-task",skip(rx))]
async fn consumer(rx: &mut mpsc::Receiver<i32>) {
    let mut i = 0;
    while let Some(val) = rx.recv().await {
        println!("received:{}", val);
       // sleep(Duration::from_millis(20)).await;
    }
}

// is there any multiple consumers for the same producer 

// cargo install tokio-console 

// RUSTFLAGS="--cfg tokio_unstable" cargo run 
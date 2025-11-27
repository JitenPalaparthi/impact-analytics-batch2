use std::sync::Arc;

use tracing::instrument;

use tokio::sync::mpsc;

use tokio::{time::Duration, time::sleep};

use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    console_subscriber::init(); // This is what the instrumentation

    let (tx, mut rx) = mpsc::channel::<i32>(2);
    let prod = tokio::spawn(producer(tx));

    let shared_rx = Arc::new(Mutex::new(rx));
    let mut workers = Vec::new();

    for i in 1..=5 {
        let rx_clone: Arc<Mutex<mpsc::Receiver<i32>>> = shared_rx.clone();
        workers.push(tokio::spawn(async move { consumer(i, rx_clone).await }));
    }

   let _ = tokio::join!(
    async {
        prod.await.unwrap();
    },
    async {
        for w in workers {
            let _ = w.await;
        }
    }
);
    
}

#[instrument(name = "producer-task", skip(tx))] // in creates a span automatically
async fn producer(tx: mpsc::Sender<i32>) {
    let mut i = 0;
    loop {
        tx.send(i).await.unwrap();
       // sleep(Duration::from_millis(10)).await;
        i += 1;
    }
}

#[instrument(name = "consumer-task", skip(rx))]
async fn consumer(i: i32, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let mut guard = rx.lock().await;
        if let Some(r) = guard.recv().await {
            drop(guard);
            println!("received data from worker-->{} data:{}", i, r);
            //sleep(Duration::from_millis(20)).await;
        }else{
            println!("some data")
        }
    }
}

// is there any multiple consumers for the same producer

// cargo install tokio-console

// RUSTFLAGS="--cfg tokio_unstable" cargo run

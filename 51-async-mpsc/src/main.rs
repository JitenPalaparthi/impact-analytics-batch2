use std::time::Duration;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx) = mpsc::channel(10);

    let tx2 = tx1.clone();

    let tx3 = tx2.clone();

    let h1 = tokio::spawn(async move {
        for i in 1..=10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Sending the value from publisher-1 {}", i);
            tx1.send(i * i).await.unwrap();
        }
    });

    let h2 = tokio::spawn(async move {
        for i in 1..=10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Sending the value from publisher-2 {}", i);
            tx2.send(i * i).await.unwrap();
        }
    });

    let h3 = tokio::spawn(async move {
        while let Some(val) = rx.recv().await {
            println!("Received by receiver-1:{}", val);
        }
    });

    let h4 = tokio::spawn(async move {
        for i in 1..=10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Sending the value from publisher-3 {}", i);
            tx3.send(i * i).await.unwrap();
        }
    });

    let result = tokio::join!(h1, h2, h3,h4);
}

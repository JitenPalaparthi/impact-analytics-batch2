use std::time::Duration;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);

    let tx1_clone = tx1.clone();
   
    let h1 = tokio::spawn(async move {
        for i in 1..=10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Sending the value from publisher-1 {}", i);
            tx1.send(i * i).await.unwrap();
            // tx2.send(i*1).await.unwrap();
        }
    });

    let h2 = tokio::spawn(async move {
        while let Some(val) = rx1.recv().await {
            println!("Received by receiver-1:{}", val);
           // tx2.send(val/2*val).await.unwrap();
             tx2.send(val).await.unwrap();
        }
    });


    let h3 = tokio::spawn(async move {
        while let Some(val) = rx2.recv().await {
            println!("Received by receiver-2:{}", val);
        }
    });

   
    let h4 = tokio::spawn(async move {
        for i in 1..=10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("Sending the value from publisher-2 {}", i);
            tx1_clone.send(i * i).await.unwrap();
            // tx2.send(i*1).await.unwrap();
        }
    });

    let result = tokio::join!(h1, h2,h3,h4);
}

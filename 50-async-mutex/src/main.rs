use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers=vec![];
    for _ in 0..5{
        let counter_cloune = Arc::clone(&counter);

        let h = tokio::spawn(async move{
            for _ in 0..10{
                let mut  num =counter_cloune.lock().await;
                *num+=1;
            }
        });
        handlers.push(h);
    }

     for h in handlers{
            h.await.unwrap();
        }
    println!("Counter:{}",*counter.lock().await);
}

use std::time::Duration;
use tokio::task::spawn_blocking;
#[tokio::main]

async fn main() {
    let b = async {
        for i in 1..=10 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            println!("async spawn-1 {}", i);
        }
    };

    let task1 = tokio::spawn(b);

    let mut num = 1;

    let task2 = tokio::spawn(async move {
        loop {
            if num > 10 {
                break;
            }
            println!("async spawn-2{}", num);
            num += 1;
        }
    });

    //    let mut a = 0;
    //    let mut b = 1;

    let fib_task = |mut a: i32, mut b: i32| async move {
        for _ in 0..10 {
            println!("fib --> {}", a);
            tokio::time::sleep(Duration::from_millis(400)).await;
            let t = a;
            a = b;
            b = t + b;
        }
    };

      let fib_task_block = |mut a: i32, mut b: i32| {
        for _ in 0..10 {
            println!("fib spawn block-2 --> {}", a);
            //tokio::time::sleep(Duration::from_millis(400)).await;
            let t = a;
            a = b;
            b = t + b;
        }
    };

    let task3 = tokio::spawn(fib_task(0, 1));

    let task4 = spawn_blocking(move || {
        let mut a =0;
        let mut b =1;
        for _ in 0..10 {
            println!("fib spawn blocking2 --> {}", a);
            let t = a;
            a = b;
            b = t + b;
        }
    });

    let task5 = spawn_blocking(move || fib_task_block(0,1));
    // not using await but using spawm so .. tokio automatically run tasks concurrently if you spawn them
    let r1 = task1.await.unwrap();
    let r2 = task2.await.unwrap();
    let r3 = task3.await.unwrap();
    let r4 = task4.await.unwrap();
    let r5 = task5.await.unwrap();
}

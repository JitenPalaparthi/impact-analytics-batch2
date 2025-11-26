use std::time::Duration;

#[tokio::main]
async fn main() {
    let b1 = async {
        let mut num = 1;
        let mut sum = 0;

        loop {
            if num > 100 {
                break sum;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
            sum += num;
            num += 1
        }
    };

    let fn1 = async |name: String| {
        let mut a = 0;
        let mut b = 1;
        for _ in 1..=10 {
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("fib from {} {} ", name, a);
            let t = a;
            a = b;
            b = t + b;
        }
    };

     let r = b1.await; // waiting here, until it is ready
     println!("result:{}",r);
     fn1("fib-1".to_string()).await;
     fib("fib-2".to_string()).await;

    //let (r1, fn1r, fn2r) = tokio::join!(b1, fn1("fib-1".to_string()), fib("fib-2".to_string()));

    //println!("result:{}", r1);
}

async fn fib(name: String) {
    let mut a = 0;
    let mut b = 1;
    for _ in 1..=10 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        println!("fib from {} {} ", name, a);
        let t = a;
        a = b;
        b = t + b;
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let task1 = runtime.spawn(async {
        for i in 0..10 {
            println!("i = {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    let task2 = runtime.spawn(async {
        for i in 0..10 {
            println!("j = {}", i);
            tokio::time::sleep(tokio::time::Duration::from_millis(60)).await;
        }
    });

    let task = async {
        tokio::try_join!(task1, task2).expect("TODO: panic message");
    };

    runtime.block_on(task);
}
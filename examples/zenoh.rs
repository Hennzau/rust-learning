use zenoh::prelude::*;

use zenoh::prelude::r#async::AsyncResolve;

async fn run() {
    let session = zenoh::open(config::default()).res().await.unwrap();
    let subscriber = session.declare_subscriber("key/expression").res().await.unwrap();
    while let Ok(sample) = subscriber.recv_async().await {
        println!("Received: {}", sample);
    };
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(run());
}
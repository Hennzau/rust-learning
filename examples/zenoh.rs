use zenoh::prelude::*;

use zenoh::prelude::r#async::AsyncResolve;

async fn run() {
    let mut config = config::peer();
    config.connect.endpoints.push("udp/10.0.0.17:7447".parse().unwrap());

    let session = zenoh::open(config).res().await.unwrap();
    let subscriber = session.declare_subscriber("key/expression")
        .callback(|sample| {
            println!("Received: {:?}", sample.value.payload);
        }).res().await.unwrap();

    loop {}
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(run());
}
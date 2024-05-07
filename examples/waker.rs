use zenoh::prelude::*;

use zenoh::prelude::r#async::AsyncResolve;

async fn run() {
    let session = zenoh::open(config::default()).res().await.unwrap();
    session.put("key/expression", "value").res().await.unwrap();
    session.close().res().await.unwrap();
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(run());
}
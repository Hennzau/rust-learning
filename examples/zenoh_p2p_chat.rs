use clap::{arg, Command};

use zenoh::{
    prelude::r#async::AsyncResolve,
    config,
    SessionDeclarations
};

async fn run() {
    let matches = Command::new("Electrode").version("1.0").about("Dora node for communication between dataflow").arg(arg!(--address <VALUE>).required(true)).arg(arg!(--listener <VALUE>).required(true)).arg(arg!(--sender <VALUE>).required(true)).get_matches();

    let (address, listener, sender) = (matches.get_one::<String>("address").expect("required").clone(), matches.get_one::<String>("listener").expect("required").clone(), matches.get_one::<String>("sender").expect("required").clone());
    let address = format!("udp/{}", address);

    let mut config = config::peer();
    config.connect.endpoints.push(address.parse().unwrap());

    let session = zenoh::open(config).res().await.unwrap();

    let pub_text_chat = sender.clone() + "/text_chat";
    let sub_text_chat = listener.clone() + "/text_chat";

    println!("Publisher: {}", pub_text_chat);
    println!("Subscriber: {}", sub_text_chat);

    let subscriber = session.declare_subscriber(sub_text_chat).callback_mut(|sample| {
        println!("Received: {:?}", sample.key_expr);
    }).res().await.unwrap();

    loop {
        let data: Vec<u8> = Vec::from ([0; 525000]);

        session.put(pub_text_chat.clone(), data).res().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(run());
}
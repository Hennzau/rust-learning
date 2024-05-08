use clap::{arg, Command};

use zenoh::{
    prelude::r#async::AsyncResolve,
    config,
    SessionDeclarations,
};

async fn run() {
    let matches = Command::new("Electrode").version("1.0").about("Dora node for communication between dataflow").arg(arg!(--port <VALUE>).required(true)).arg(arg!(--protocol <VALUE>).required(true)).arg(arg!(--listener <VALUE>).required(true)).arg(arg!(--sender <VALUE>).required(true)).get_matches();

    let (listener, sender) = (matches.get_one::<String>("listener").expect("required").clone(), matches.get_one::<String>("sender").expect("required").clone());
    let (protocol, port) = (matches.get_one::<String>("protocol").expect("required").clone(), matches.get_one::<String>("port").expect("required").clone());

    let listener_session = format!("{}/{}:{}", protocol, listener, port);
    let sender_session = format!("{}/{}:{}", protocol, sender, port);

    let mut config = config::peer();
    config.listen.endpoints.push(listener_session.parse().unwrap());
    config.connect.endpoints.push(sender_session.parse().unwrap());

    let session = zenoh::open(config).res().await.unwrap();

    let pub_text_chat = sender.clone() + "/text_chat";
    let sub_text_chat = listener.clone() + "/text_chat";

    println!("Publisher: {}", pub_text_chat);
    println!("Subscriber: {}", sub_text_chat);

    let subscriber = session.declare_subscriber(sub_text_chat).callback_mut(|sample| {
        println!("Received: {:?}", sample.key_expr);
    }).res().await.unwrap();

    let mut data: Vec<u8> = Vec::new();
    for i in 0..1024 * 1024 { data.push(0); }

    loop {
        session.put(pub_text_chat.clone(), data.clone()).res().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(run());
}
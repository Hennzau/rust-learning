use std::{
    collections::VecDeque,
    sync::{
        Arc,
        Mutex
    }
};

use clap::{arg, Command};

struct Packet {
    topic: String,
    data: Vec<u8>
}

struct Shared {
    running: bool,
    packets: VecDeque<Packet>
}

async fn run_listener(listener: String, shared: Arc<Mutex<Shared>>) {
    println!("Listener: {}", listener);

    let socket = tokio::net::UdpSocket::bind(listener.clone()).await.unwrap();

    loop {
        // Receive a packet

        let mut buf = [0; 1024];

        let status = socket.try_recv(&mut buf);
        if let Ok(len) = status {
            let data = buf[..len].to_vec();
            let data = String::from_utf8(data).unwrap();

            println!("Listener {} received data from sender: {}", listener, data);
        }

        let shared = shared.lock().unwrap();

        if !shared.running {
            break;
        }
    }
}


async fn run(host: String, sender: String, shared: Arc<Mutex<Shared>>) {
    println!("Sender: {}", sender);

    let socket = tokio::net::UdpSocket::bind(host.clone()).await.unwrap();
    println!("Electrode node running at : {}", socket.local_addr().unwrap());

    socket.connect(sender.clone()).await.unwrap();

    let data = format!("Data sent from {}", socket.local_addr().unwrap());

    let mut i = 0;
    loop {
        let status = socket.send(data.as_bytes()).await;

        if let Err(e) = status {
            eprintln!("Error: {}", e);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        i += 1;

        if i >= 100 {
            break;
        }
    }

    let mut shared = shared.lock().unwrap();
    shared.running = false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Electrode").version("1.0").about("Dora node for communication between dataflow").arg(arg!(--address <VALUE>).required(true)).arg(arg!(--listener <VALUE>).required(true)).arg(arg!(--sender <VALUE>).required(true)).get_matches();

    let (address, listener, sender) = (matches.get_one::<String>("address").expect("required").clone(), matches.get_one::<String>("listener").expect("required").clone(), matches.get_one::<String>("sender").expect("required").clone());

    let listener = address.clone() + ":" + &listener;
    let sender = address.clone() + ":" + &sender;
    let host = address.clone() + ":0";

    let shared = Arc::new(Mutex::new(Shared {
        running: true,
        packets: VecDeque::new()
    }));

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let listener_task = runtime.spawn(run_listener(sender, shared.clone()));
    let main_task = runtime.spawn(run(host, listener, shared.clone()));

    let listener = runtime.block_on(listener_task);
    let main = runtime.block_on(main_task);

    return match (listener, main) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), _) | (_, Err(e)) => Err(e.into()),
    };
}

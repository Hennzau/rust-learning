use std::{
    collections::VecDeque,
    sync::{
        Arc,
        Mutex
    }
};

struct Packet {
    topic: String,
    data: Vec<u8>
}

struct Shared {
    running: bool,
    packets: VecDeque<Packet>
}

async fn run_listener(shared: Arc<Mutex<Shared>>) {
    let args: Vec<String> = std::env::args().collect();
    let listener = args.get(2).unwrap().clone();

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


async fn run(shared: Arc<Mutex<Shared>>) {
    let args: Vec<String> = std::env::args().collect();
    let sender = args.get(4).unwrap().clone();
    let listener = args.get(2).unwrap().clone();
    println!("Sender: {}", sender);
    println!("Listener: {}", listener);

    let socket = tokio::net::UdpSocket::bind(listener.clone()).await.unwrap();
    socket.connect(sender.clone()).await.unwrap();

    let data = format!("Data sent from {}", sender);

    let mut i = 0;
    loop {
        let status = socket.send(data.as_bytes()).await;

        if let Err(e) = status {
            eprintln!("Error: {}", e);
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        i += 1;

        if i == 10 {
            break;
        }

        println!("{}", i);
    }

    let mut shared = shared.lock().unwrap();
    shared.running = false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let shared = Arc::new(Mutex::new(Shared {
        running: true,
        packets: VecDeque::new()
    }));

    let runtime = tokio::runtime::Runtime::new().unwrap();

    let listener_task = runtime.spawn(run_listener(shared.clone()));
    let main_task = runtime.spawn(run(shared.clone()));

    let listener = runtime.block_on(listener_task);
    let main = runtime.block_on(main_task);

    return match (listener, main) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(e), _) | (_, Err(e)) => Err(e.into()),
    };
}

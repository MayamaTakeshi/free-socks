//! Outbound socket example

use free_socks::EventSocket;
//use free_socks::EventReader;

//#![warn(rust_2018_idioms)]

use tokio::net::TcpListener;

use tokio::io::split;

use tokio::io::AsyncWriteExt;

use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream, ToSocketAddrs,
    }
};

use tokio::time::sleep;

use std::time::{Duration, Instant};



use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8084".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("connection accepted");

        tokio::spawn(async move {
            let event_socket = EventSocket::new(socket, Duration::from_secs(5));

            let client = event_socket.client();

            println!("Sending connect");
            let mut reply = client.command("connect\n\n").await;
            println!("{:?}", reply);

            println!("Sending myevents");
            let mut reply = client.command("myevents\n\n").await;
            println!("{:?}", reply);

            println!("Sending answer");
            reply = client.command("sendmsg\ncall-command: execute\nexecute-app-name: answer\n\n").await;
            println!("{:?}", reply);
     
            println!("Sending send_dtmf");
            reply = client.command("sendmsg\ncall-command: execute\nexecute-app-name: send_dtmf\nexecute-app-data: 1234\n\n").await;
            println!("{:?}", reply);

            let duration = Duration::from_secs(3);
            sleep(duration).await;

            println!("Sending hangup");
            reply = client.command("sendmsg\ncall-command: execute\nexecute-app-name: hangup\n\n").await;
            println!("{:?}", reply);
     
        /*

            let mut disconnected = false;

            while !disconnected {
                match event_socket.event_reader.read_event().await {
                    Err(e) => {
                        disconnected = true;
                        println!("{:?}", e);
                    },
                    Ok(event) => {
                        println!("{:?}", event);
                    }
                };
            }
        */
        });
    }
}

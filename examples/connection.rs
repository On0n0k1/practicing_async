// use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{task, pin};
use std::error::Error;
use std::net::Ipv4Addr;

use tokio_stream::{self as stream, StreamExt};

enum Frame{
    Value(i32),
    Exit,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    // Connection will be refused because DNS not enabled(somehow)
    // let mut stream = TcpStream::connect("127.0.0.1:0").await?;
    let mut listener = TcpListener::bind((Ipv4Addr::new(127, 0, 0, 1), 0)).await?;

    println!("New Address is: {}", listener.local_addr()?);

    let join = task::spawn(async {
        println!("Sending values.");
        let mut stream = stream::iter(vec![0..10]);

        


    });

    pin!(join);
    (&mut join).await?;

    loop {
        let (_socket, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);

    }


    // Write some data.
    // stream.write_all(b"hello world!").await?;

    Ok(())
}
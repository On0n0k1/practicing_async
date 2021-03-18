use tokio::sync::mpsc;

/// Creates 2 async tasks that fill their own channels
/// 
/// Read each channel with the main task and print the sum of each value.
#[tokio::main]
async fn main(){
    let (tx, mut rx) = mpsc::channel(11);
    let (tx2, mut rx2) = mpsc::channel(11);

    let first = tokio::spawn(async move {
        // tx.send("sending from first handle").await.unwrap();
        for i in 0..10{
            tx.send(i as usize).await.unwrap();
        }
    });
    let second = tokio::spawn(async move {
        // tx2.send("sending from second handle").await.unwrap();
        for i in 0..10{
            tx2.send((10 - i) as usize).await.unwrap();
        }
    });
    first.await.unwrap();
    second.await.unwrap();

    // results expected to be 10 10 10 10 10...
    // while let (Some(message), Some(message2)) = (rx.recv().await, rx2.recv().await) {
    //     println!("Got = {}", message + message2);
    // }
    while let Some(message) = rx.recv().await{
        if let Some(message2) = rx2.recv().await{
            println!("Got = {} + {} = {}", message, message2, message + message2);
        }
        else{
            println!("Got = {}", message);
        }
    }
    // while let Some(message2) = rx2.recv().await{
    //     println!("Got = {}", message2);
    // }
}

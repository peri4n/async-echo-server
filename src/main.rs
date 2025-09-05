use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection at {}", addr);

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            print!("Starting to echo data for {}\n", addr);

            io::copy(&mut reader, &mut writer).await.unwrap();
            println!("Connection at {} closed", addr);
        });
    }
}

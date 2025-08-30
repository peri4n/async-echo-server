use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection at {}", addr);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut buf = String::with_capacity(1000);
            let mut reader = BufReader::new(reader);

            while let Ok(_) = reader.read_line(&mut buf).await {
                if buf.trim() == "quit" {
                    break;
                }
                writer.write_all(buf.as_bytes()).await.unwrap();
                buf.clear();
            }
        });
    }
}

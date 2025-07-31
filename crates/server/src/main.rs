use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:25565").await?;
    println!("Server listening on port 25565");

    loop {
        let (_socket, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);
    }
}

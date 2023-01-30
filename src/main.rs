use public_ip_addr::get_public_ip;

#[tokio::main]
async fn main() {
    let ip = get_public_ip().await.unwrap();
    println!("Your public IP is: {}", ip);
}

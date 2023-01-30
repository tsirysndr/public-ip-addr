<h1>public-ip-addr</h1>
<p>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
  <a href="https://crates.io/crates/public-ip-addr" target="_blank">
    <img src="https://img.shields.io/crates/v/public-ip-addr.svg" />
  </a>
  
  <a href="https://crates.io/crates/public-ip-addr" target="_blank">
    <img src="https://img.shields.io/crates/dr/public-ip-addr" />
  </a>
  
  <a href="https://docs.rs/public-ip-addr" target="_blank">
    <img src="https://docs.rs/public-ip-addr/badge.svg" />
  </a>
</p>

A simple library to get your public IP address.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
public-ip-addr = "0.1"
```

## Example

```rust
use public_ip_addr::get_public_ip;

#[tokio::main]
async fn main() {
    let ip = get_public_ip().await.unwrap();
    println!("Your public IP is: {}", ip);
}
```

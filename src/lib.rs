use std::time::Duration;

use serde::Deserialize;
use surf::{Client, Config};
#[derive(Deserialize)]
pub struct IpInfo {
    #[serde(rename = "Answer")]
    pub answer: String,
}

pub async fn get_public_ip() -> Result<String, surf::Error> {
    let client: Client = Config::new()
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()
        .unwrap();
    let res: IpInfo = client
        .get("https://api.duckduckgo.com/?q=ip&format=json")
        .recv_json()
        .await?;
    let ip = res.answer.split(" ").collect::<Vec<&str>>()[4];
    Ok(ip.to_string())
}

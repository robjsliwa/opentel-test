mod name;

use name::name_client::NameClient;
use name::NameRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:60000")
        .connect()
        .await?;

    let mut client = NameClient::new(channel);

    let request = tonic::Request::new(NameRequest{});

    let response = client.get_name(request).await?.into_inner();
    println!("Sidekick name:{:?}", response);

    Ok(())
}
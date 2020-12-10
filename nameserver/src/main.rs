mod name;

use tonic::{transport::Server, Request, Response, Status};
use name::name_server::{Name, NameServer};
use name::{NameResponse, NameRequest};

#[derive(Default)]
pub struct SidekickNames {}

#[tonic::async_trait]
impl Name for SidekickNames {
    async fn get_name(&self, _: Request<NameRequest>) -> Result<Response<NameResponse>, Status> {
        Ok(Response::new(NameResponse{
            name: String::from("Test name"),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:60000".parse().unwrap();

    let sidekick_name_server = SidekickNames::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(NameServer::new(sidekick_name_server))
        .serve(addr)
        .await?;

    Ok(())
}

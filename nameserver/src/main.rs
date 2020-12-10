mod name;

use tonic::{transport::Server, Request, Response, Status};
use name::name_server::{Name, NameServer};
use name::{NameResponse, NameRequest};
use std::collections::HashMap;

#[derive(Default)]
pub struct SidekickNames {}

#[tonic::async_trait]
impl Name for SidekickNames {
    async fn get_name(&self, _: Request<NameRequest>) -> Result<Response<NameResponse>, Status> {
        let resp = reqwest::get("http://localhost:8000/name")
            .await
            .map_err(|e| Status::unknown(format!("{}", e)))?
            .json::<HashMap<String, String>>()
            .await
            .map_err(|e| Status::unknown(format!("{}", e)))?;
        Ok(Response::new(NameResponse{
            name: resp.get(&"name".to_string()).unwrap().clone(),
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

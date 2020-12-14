mod name;

use tonic::{transport::Server, Request, Response, Status};
use name::name_server::{Name, NameServer};
use name::{NameResponse, NameRequest};
use std::collections::HashMap;
use hyper::{Method, Client, Body};
use opentelemetry::global;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::{
    trace::{Span, TraceContextExt, Tracer},
    Context, KeyValue,
};

#[derive(Default)]
pub struct SidekickNames {}

#[tonic::async_trait]
impl Name for SidekickNames {
    async fn get_name(&self, request: Request<NameRequest>) -> Result<Response<NameResponse>, Status> {
        let parent_cx = global::get_text_map_propagator(|prop| prop.extract(request.metadata()));
        let span = global::tracer("get_name").start_from_context("get_name", &parent_cx);
        let cx = Context::current_with_span(span);

        let mut headers = reqwest::header::HeaderMap::new();
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut headers)
        });

        println!("TEST {:?}", headers);
        let client = reqwest::Client::new();
        let resp = client.get("http://localhost:8000/name")
            .headers(headers)
            .send()
            .await
            .map_err(|e| Status::unknown(format!("{}", e)))?
            .json::<HashMap<String, String>>()
            .await
            .map_err(|e| Status::unknown(format!("{}", e)))?;

        // span.set_attribute(KeyValue::new("request", format!("{:?}", resp.get(&"name".to_string()).unwrap().clone())));

        Ok(Response::new(NameResponse{
            name: resp.get(&"name".to_string()).unwrap().clone(),
        }))
    }
}

fn tracing_init() -> Result<(impl Tracer, opentelemetry_jaeger::Uninstall), Box<dyn std::error::Error + Send + Sync + 'static>> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("nameserver")
        .install()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _uninstall = tracing_init()?;
    let addr = "[::1]:60000".parse().unwrap();

    let sidekick_name_server = SidekickNames::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(NameServer::new(sidekick_name_server))
        .serve(addr)
        .await?;

    Ok(())
}

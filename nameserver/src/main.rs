mod name;

use tonic::{transport::Server, Request, Response, Status};
use name::name_server::{Name, NameServer};
use name::{NameResponse, NameRequest};
use std::collections::HashMap;
use std::env::var;
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
        let sidekick_host = match var("SIDEKICK_HOST") {
            Ok(val) => val,
            Err(_) => "localhost".to_string(),
        };
        let sidekick_url = format!("http://{}:8000/name", sidekick_host);
        let parent_cx = global::get_text_map_propagator(|prop| prop.extract(request.metadata()));
        let span = global::tracer("").start_from_context("get_name", &parent_cx);
        span.set_attribute(KeyValue::new("note", "calling node server"));
        let cx = Context::current_with_span(span);

        let mut headers = reqwest::header::HeaderMap::new();
        global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&cx, &mut headers)
        });

        let client = reqwest::Client::new();
        let resp = client.get(&sidekick_url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| Status::unknown(format!("{}", e)))?
            .json::<HashMap<String, String>>()
            .await
            .map_err(|e| Status::unknown(format!("{}", e)))?;

        // span.set_attribute(KeyValue::new("received_nickname", format!("{:?}", resp.get(&"name".to_string()).unwrap().clone())));

        Ok(Response::new(NameResponse{
            name: resp.get(&"name".to_string()).unwrap().clone(),
        }))
    }
}

fn tracing_init() -> Result<(impl Tracer, opentelemetry_jaeger::Uninstall), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let trace_host = match var("TRACE_HOST") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(),
    };
    global::set_text_map_propagator(TraceContextPropagator::new());
    opentelemetry_jaeger::new_pipeline()
        .with_collector_endpoint(format!("http://{}:14268/api/traces", trace_host))
        .with_service_name("nameserver")
        .install()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let _uninstall = tracing_init()?;
    let addr = "0.0.0.0:60000".parse()?;

    let sidekick_name_server = SidekickNames::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(NameServer::new(sidekick_name_server))
        .serve(addr)
        .await?;

    Ok(())
}

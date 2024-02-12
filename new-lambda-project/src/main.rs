use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

// based on diabetes dataset 
#[derive(Deserialize)]
struct Request {
    glucose: i32,
    units: i32,
    outcome: i32,
}

#[derive(Serialize)]
struct Response {
    req_id: String,
    percent: i32,
    product: i32,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let glucose = event.payload.glucose;
    let units= event.payload.units;

    let percent = (glucose / units) * 100;
    let product = glucose * units;

    let resp = Response {
        req_id: event.context.request_id.clone(),
        percent,
        product,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

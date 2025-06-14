use unit_conversion::{init_units, Interpreter};

use axum::{
    http::{StatusCode, HeaderValue, Method},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use http::header;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    init_units();


    let app = Router::new()
        .route("/api/check_unit", post(check_unit))
        .route("/api/check_conv", post(check_conv))
        .layer(
        //     // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
        //     // for more details
        //     //
        //     // pay attention that for some request types like posting content-type: application/json
        //     // it is required to add ".allow_headers([http::header::CONTENT_TYPE])"
        //     // or see this issue https://github.com/tokio-rs/axum/issues/849
            CorsLayer::new()
                .allow_origin("http://localhost:8000".parse::<HeaderValue>().unwrap())
                // .allow_origin("Any")
                .allow_methods([Method::GET])
        );

    println!("booting up server");

    axum::Server::bind(&"0.0.0.0:8282".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct CheckUnitRequest {
    unit: String,
}

#[derive(Deserialize)]
struct CheckConvRequest {
    src: String,
    tgt: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}


#[derive(Serialize)]
pub struct ConvResponse {
    pub coherent: bool,
    pub result: f64
}


async fn check_unit(
    Json(payload): Json<CheckUnitRequest>,
) -> Result<impl IntoResponse, StatusCode> {

    let mut ii = Interpreter::new();

    match ii.deco(&payload.unit) {
        Ok(v) => Ok(Json(v).into_response()),
        Err(e) => {
            let error_response = ErrorResponse { error: e.to_string() };
            Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response())
        }
    }
}

async fn check_conv(
    Json(payload): Json<CheckConvRequest>,
) -> Result<impl IntoResponse, StatusCode> {

    let mut ii = Interpreter::new();
    let req = format!("1 {}=>{}", payload.src, payload.tgt);
    match ii.conv(&req) {
        Ok(v) => {
            let resp = ConvResponse {coherent: true, result: v };
            Ok(Json(resp).into_response())
        },
        Err(_) => {
            let resp = ConvResponse {coherent: false, result: 0.0 };
            // let error_response = ErrorResponse { error: e.to_string() };
            Ok((StatusCode::BAD_REQUEST, Json(resp)).into_response())
        }
    }
}
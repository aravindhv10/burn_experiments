mod model;
mod mylib;

use tokio;

use futures_util::TryStreamExt;

// use std::net::IpAddr;
// use std::net::Ipv4Addr;
// use std::net::SocketAddr;
// use std::sync::Arc;

// use actix_multipart::Multipart;
// use actix_web::App;
// use actix_web::Error;
// use actix_web::HttpResponse;
// use actix_web::HttpServer;
// use actix_web::web;

async fn infer_handler(
    mut payload: actix_multipart::Multipart,
    infer_slave: actix_web::web::Data<std::sync::Arc<model::model_client>>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {

    let mut data = Vec::new();

    while let Some(mut field) = payload.try_next().await? {
        while let Some(chunk) = field.try_next().await? {
            data.extend_from_slice(&chunk);
        }
    }

    if data.is_empty() {
        return Ok(actix_web::HttpResponse::BadRequest().body("No image data"));
    }

    match infer_slave.do_infer_data(data).await {
        Ok(pred) => {
            return Ok(actix_web::HttpResponse::Ok().json(model::prediction_probabilities_reply::from(pred)));
        },
        Err(e) => {
            return Ok(actix_web::HttpResponse::InternalServerError().body(e));
        },
    }
}

pub mod infer {
    tonic::include_proto!("infer"); // The string specified here must match the proto package name
}

pub struct MyInferer {
    slave_client: std::sync::Arc<model::model_client>
}

#[tonic::async_trait]
impl infer::infer_server::Infer for MyInferer {
    async fn do_infer(&self, request: tonic::Request<infer::Image>) -> Result<tonic::Response<infer::Prediction>, tonic::Status> {
        println!("Received gRPC request");
        let image_data = request.into_inner().image_data;
        match self.slave_client.do_infer_data(image_data).await {
            Ok(pred) => {
                let reply = infer::Prediction {
                    ps1: pred.val[0],
                    ps2: pred.val[1],
                    ps3: pred.val[2],
                };
                return Ok(tonic::Response::new(reply));
            },
            Err(e) => {
                Err(tonic::Status::internal(e))
            },
        }
    }
}

#[actix_web::main]
async fn main() -> () {
    let (mut slave_server, slave_client) = crate::model::get_inference_tuple();
    let slave_client_1 = std::sync::Arc::new(slave_client);
    let slave_client_2 = std::sync::Arc::clone(&slave_client_1);
    let future_infer = slave_server.infer_loop();
    match actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(std::sync::Arc::clone(&slave_client_1)))
            .route("/infer", actix_web::web::post().to(infer_handler))
    })
    .bind(("0.0.0.0", 8000))
    {
        Ok(ret) => {
            let future_rest_server = ret.run();
            let ip_v4 = std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0));
            let addr = std::net::SocketAddr::new(ip_v4, 8001);
            let inferer_service = MyInferer{slave_client: slave_client_2};
            let future_grpc = tonic::transport::Server::builder().add_service(infer::infer_server::InferServer::new(inferer_service)).serve(addr);
            let (first, second, third) = tokio::join!(future_infer, future_rest_server, future_grpc);
            match second {
                Ok(_) => {
                    println!("REST server executed and stopped successfully");
                }
                Err(e) => {
                    println!("Encountered error in starting the server due to {}.", e);
                }
            }
            match third {
                Ok(_) => {
                    println!("GRPC server executed and stopped successfully");
                }
                Err(e) => {
                    println!("Encountered error in starting the server due to {}.", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to bind to port");
        }
    }
}

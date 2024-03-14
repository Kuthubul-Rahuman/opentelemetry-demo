// Copyright The OpenTelemetry Authors
// SPDX-License-Identifier: Apache-2.0

use tonic::transport::Server;

use log::*;

use std::env;

mod shipping_service;
use shipping_service::shop::shipping_service_server::ShippingServiceServer;
use shipping_service::ShippingServer;

mod telemetry;
use telemetry::configure_global_logger;
use telemetry::init_logger;
use telemetry::init_metrics;

use telemetry::init_tracer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<ShippingServiceServer<ShippingServer>>()
        .await;

    let tracer = init_tracer()?;
    let _ = init_metrics()?;

    let _ = init_logger();
    configure_global_logger(tracer);

    info!("OTel pipeline created");
    let port = env::var("SHIPPING_SERVICE_PORT").expect("$SHIPPING_SERVICE_PORT is not set");
    let addr = format!("0.0.0.0:{}", port).parse()?;
    info!("listening on {}", addr);
    let shipper = ShippingServer::default();

    Server::builder()
        .add_service(ShippingServiceServer::new(shipper))
        .add_service(health_service)
        .serve(addr)
        .await?;

    Ok(())
}

use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

mod db;

#[path = "../../src/protos/invoice_grpc.rs"]
mod invoice_grpc;

#[path = "../../src/protos/invoice.rs"]
mod invoice;

mod service;
use service::AnalysisService;

fn main() {
    let minutes = Duration::new(15 * 60, 0);
    let env = Arc::new(Environment::new(1));
    let store = db::DetectDuplicateStore::new(minutes);
    let service = invoice_grpc::create_analysis(AnalysisService(store));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_052)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}

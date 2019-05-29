use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, ServerBuilder};

mod db;

#[path = "../protos/invoice.rs"]
mod invoice;

#[path = "../protos/invoice_grpc.rs"]
mod invoice_grpc;

mod service;
use service::RatingService;

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = invoice_grpc::create_invoices(RatingService());
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_053)
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

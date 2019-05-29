use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

#[path = "../protos/invoice_grpc.rs"]
mod invoice_grpc;
use invoice_grpc::Analysis;

#[path = "../protos/invoice.rs"]
mod invoice;
use invoice::{DetectDuplicateReply, DetectDuplicateRequest};

#[derive(Clone)]
struct AnalysisService(db::DetectDuplicateStore);

mod db;

impl Analysis for AnalysisService {
    fn detect_duplicate(
        &mut self,
        ctx: RpcContext<'_>,
        req: DetectDuplicateRequest,
        sink: UnarySink<DetectDuplicateReply>,
    ) {
        let dup_res = self.0.detect_duplicate(req.get_invoice());
        let mut resp = DetectDuplicateReply::new();
        resp.set_result(dup_res);
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

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

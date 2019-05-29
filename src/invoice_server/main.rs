use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use protobuf::RepeatedField;

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

#[path = "../protos/invoice_grpc.rs"]
mod invoice_grpc;
use invoice_grpc::Invoices;

#[path = "../protos/invoice.rs"]
mod invoice;
use invoice::{
    CreateInvoiceReply, CreateInvoiceRequest, ListInvoiceReply, ListInvoiceRequest,
    RemoveInvoiceReply, RemoveInvoiceRequest,
};

mod db;

#[derive(Clone)]
struct InvoiceService(db::Database);

impl Invoices for InvoiceService {
    fn create(
        &mut self,
        ctx: RpcContext<'_>,
        req: CreateInvoiceRequest,
        sink: UnarySink<CreateInvoiceReply>,
    ) {
        let invoice_number = self.0.create(req.get_invoice());
        let mut resp = CreateInvoiceReply::new();
        resp.set_invoice_number(invoice_number);
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn list(
        &mut self,
        ctx: RpcContext<'_>,
        req: ListInvoiceRequest,
        sink: UnarySink<ListInvoiceReply>,
    ) {
        let invoice_numbers = self.0.list();
        let mut resp = ListInvoiceReply::new();
        let f = RepeatedField::from_vec(invoice_numbers);
        resp.set_invoice_numbers(f);
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }

    fn remove(
        &mut self,
        ctx: RpcContext<'_>,
        req: RemoveInvoiceRequest,
        sink: UnarySink<RemoveInvoiceReply>,
    ) {
        let mut resp = RemoveInvoiceReply::new();
        match self.0.remove(req.get_invoice_number()) {
            Some(invoice_number) => resp.set_invoice_number(invoice_number.to_string()),
            None => {}
        };
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = invoice_grpc::create_invoices(InvoiceService(db::Database::new()));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50_051)
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

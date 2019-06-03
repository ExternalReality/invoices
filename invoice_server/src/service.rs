use futures::Future;
use grpcio::{RpcContext, UnarySink};
use protobuf::RepeatedField;

use crate::db::Database;
use crate::invoice::{
    ByCompanyReply, ByCompanyRequest, CreateInvoiceReply, CreateInvoiceRequest, ListInvoiceReply,
    ListInvoiceRequest, RemoveInvoiceReply, RemoveInvoiceRequest,
};
use crate::invoice_grpc::Invoices;

#[derive(Clone)]
pub struct InvoiceService(pub Database);

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

    fn by_company(
        &mut self,
        ctx: RpcContext<'_>,
        req: ByCompanyRequest,
        sink: UnarySink<ByCompanyReply>,
    ) {
        let invoices = self.0.read_by_company(req.get_company_name());
        let mut resp = ByCompanyReply::new();
        let f = RepeatedField::from_vec(invoices);
        resp.set_invoices(f);
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

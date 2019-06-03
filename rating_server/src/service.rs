use futures::Future;

use crate::invoice::{ByCompanyRequest, Invoice, RatingReply, RatingRequest};
use crate::invoice_grpc::{InvoicesClient, Rating};
use grpcio::{RpcContext, UnarySink};

#[derive(Clone)]
pub struct RatingService(pub InvoicesClient);

impl Rating for RatingService {
    fn generate_bill(
        &mut self,
        ctx: RpcContext<'_>,
        req: RatingRequest,
        sink: UnarySink<RatingReply>,
    ) {
        let mut company_invoices_req = ByCompanyRequest::new();
        company_invoices_req.set_company_name(req.get_company_name().to_string());
        let invoices = self
            .0
            .by_company(&company_invoices_req)
            .unwrap()
            .take_invoices();
        let bill_amount = calculate_bill(&invoices);
        let mut resp = RatingReply::new();
        resp.set_bill_amount(bill_amount);
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn calculate_bill(invoices: &[Invoice]) -> f64 {
    invoices.iter().fold(0.0, |acc, x| acc + x.total_price)
}

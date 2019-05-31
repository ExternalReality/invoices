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

#[cfg(test)]
mod tests {
    use crate::invoice::Invoice;

    #[test]
    fn add_bill_values() {
        let mut inv1 = Invoice::new();
        let mut inv2 = Invoice::new();

        inv1.set_total_price(100.21);
        inv2.set_total_price(230.33);

        assert_eq!(
            super::calculate_bill(&[inv1.clone(), inv2.clone()]),
            inv1.total_price + inv2.total_price
        )
    }
}

use futures::Future;

use crate::invoice::{RatingReply, RatingRequest};
use crate::invoice_grpc::Rating;
use grpcio::{RpcContext, UnarySink};

#[derive(Clone)]
pub struct RatingService();

impl Rating for RatingService {
    fn generate_bill(
        &mut self,
        ctx: RpcContext<'_>,
        req: RatingRequest,
        sink: UnarySink<RatingReply>,
    ) {
        let resp = RatingReply::new();
        let f = sink
            .success(resp)
            .map_err(move |e| panic!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

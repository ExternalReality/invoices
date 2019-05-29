use futures::Future;

use crate::db::DetectDuplicateStore;
use crate::invoice::{DetectDuplicateReply, DetectDuplicateRequest};
use crate::invoice_grpc::Analysis;
use crate::{RpcContext, UnarySink};

#[derive(Clone)]
pub struct AnalysisService(pub DetectDuplicateStore);

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

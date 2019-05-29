// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_INVOICES_CREATE: ::grpcio::Method<super::invoice::CreateInvoiceRequest, super::invoice::CreateInvoiceReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Invoices/Create",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INVOICES_LIST: ::grpcio::Method<super::invoice::ListInvoiceRequest, super::invoice::ListInvoiceReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Invoices/List",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_INVOICES_REMOVE: ::grpcio::Method<super::invoice::RemoveInvoiceRequest, super::invoice::RemoveInvoiceReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Invoices/Remove",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct InvoicesClient {
    client: ::grpcio::Client,
}

impl InvoicesClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        InvoicesClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_opt(&self, req: &super::invoice::CreateInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoice::CreateInvoiceReply> {
        self.client.unary_call(&METHOD_INVOICES_CREATE, req, opt)
    }

    pub fn create(&self, req: &super::invoice::CreateInvoiceRequest) -> ::grpcio::Result<super::invoice::CreateInvoiceReply> {
        self.create_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_async_opt(&self, req: &super::invoice::CreateInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::CreateInvoiceReply>> {
        self.client.unary_call_async(&METHOD_INVOICES_CREATE, req, opt)
    }

    pub fn create_async(&self, req: &super::invoice::CreateInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::CreateInvoiceReply>> {
        self.create_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_opt(&self, req: &super::invoice::ListInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoice::ListInvoiceReply> {
        self.client.unary_call(&METHOD_INVOICES_LIST, req, opt)
    }

    pub fn list(&self, req: &super::invoice::ListInvoiceRequest) -> ::grpcio::Result<super::invoice::ListInvoiceReply> {
        self.list_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_async_opt(&self, req: &super::invoice::ListInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::ListInvoiceReply>> {
        self.client.unary_call_async(&METHOD_INVOICES_LIST, req, opt)
    }

    pub fn list_async(&self, req: &super::invoice::ListInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::ListInvoiceReply>> {
        self.list_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn remove_opt(&self, req: &super::invoice::RemoveInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoice::RemoveInvoiceReply> {
        self.client.unary_call(&METHOD_INVOICES_REMOVE, req, opt)
    }

    pub fn remove(&self, req: &super::invoice::RemoveInvoiceRequest) -> ::grpcio::Result<super::invoice::RemoveInvoiceReply> {
        self.remove_opt(req, ::grpcio::CallOption::default())
    }

    pub fn remove_async_opt(&self, req: &super::invoice::RemoveInvoiceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::RemoveInvoiceReply>> {
        self.client.unary_call_async(&METHOD_INVOICES_REMOVE, req, opt)
    }

    pub fn remove_async(&self, req: &super::invoice::RemoveInvoiceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::RemoveInvoiceReply>> {
        self.remove_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Invoices {
    fn create(&mut self, ctx: ::grpcio::RpcContext, req: super::invoice::CreateInvoiceRequest, sink: ::grpcio::UnarySink<super::invoice::CreateInvoiceReply>);
    fn list(&mut self, ctx: ::grpcio::RpcContext, req: super::invoice::ListInvoiceRequest, sink: ::grpcio::UnarySink<super::invoice::ListInvoiceReply>);
    fn remove(&mut self, ctx: ::grpcio::RpcContext, req: super::invoice::RemoveInvoiceRequest, sink: ::grpcio::UnarySink<super::invoice::RemoveInvoiceReply>);
}

pub fn create_invoices<S: Invoices + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INVOICES_CREATE, move |ctx, req, resp| {
        instance.create(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INVOICES_LIST, move |ctx, req, resp| {
        instance.list(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_INVOICES_REMOVE, move |ctx, req, resp| {
        instance.remove(ctx, req, resp)
    });
    builder.build()
}

const METHOD_ANALYSIS_DETECT_DUPLICATE: ::grpcio::Method<super::invoice::DetectDuplicateRequest, super::invoice::DetectDuplicateReply> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Analysis/DetectDuplicate",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AnalysisClient {
    client: ::grpcio::Client,
}

impl AnalysisClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AnalysisClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn detect_duplicate_opt(&self, req: &super::invoice::DetectDuplicateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::invoice::DetectDuplicateReply> {
        self.client.unary_call(&METHOD_ANALYSIS_DETECT_DUPLICATE, req, opt)
    }

    pub fn detect_duplicate(&self, req: &super::invoice::DetectDuplicateRequest) -> ::grpcio::Result<super::invoice::DetectDuplicateReply> {
        self.detect_duplicate_opt(req, ::grpcio::CallOption::default())
    }

    pub fn detect_duplicate_async_opt(&self, req: &super::invoice::DetectDuplicateRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::DetectDuplicateReply>> {
        self.client.unary_call_async(&METHOD_ANALYSIS_DETECT_DUPLICATE, req, opt)
    }

    pub fn detect_duplicate_async(&self, req: &super::invoice::DetectDuplicateRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::invoice::DetectDuplicateReply>> {
        self.detect_duplicate_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Analysis {
    fn detect_duplicate(&mut self, ctx: ::grpcio::RpcContext, req: super::invoice::DetectDuplicateRequest, sink: ::grpcio::UnarySink<super::invoice::DetectDuplicateReply>);
}

pub fn create_analysis<S: Analysis + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ANALYSIS_DETECT_DUPLICATE, move |ctx, req, resp| {
        instance.detect_duplicate(ctx, req, resp)
    });
    builder.build()
}

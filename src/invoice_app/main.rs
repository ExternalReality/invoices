use std::fs;
use std::sync::Arc;

#[macro_use]
extern crate clap;
use clap::App;

use grpcio::{ChannelBuilder, EnvBuilder};
use toml;

#[path = "../protos/invoice_grpc.rs"]
mod invoice_grpc;
use invoice_grpc::{AnalysisClient, InvoicesClient, RatingClient};

#[path = "../protos/invoice.rs"]
mod invoice;
use invoice::DetectDuplicateReply_Result;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let invoice_client = invoice_grpc::InvoicesClient::new(ch);

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50052");
    let analysis_client = invoice_grpc::AnalysisClient::new(ch);

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50053");
    let rating_client = invoice_grpc::RatingClient::new(ch);

    match matches.subcommand() {
        ("submit", Some(sc)) => handle_submit(sc, invoice_client, analysis_client),
        ("list", Some(sc)) => handle_list(sc, invoice_client),
        ("remove", Some(sc)) => handle_remove(sc, invoice_client),
        ("bill", Some(sc)) => handle_bill(sc, rating_client),
        _ => {}
    }
}

fn handle_submit(sc: &clap::ArgMatches<'_>, inc: InvoicesClient, anc: AnalysisClient) {
    let filename = sc.value_of("input").unwrap();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let inv: invoice::Invoice = toml::from_str(&contents).unwrap();

    if !sc.is_present("force") {
        let mut req = invoice::DetectDuplicateRequest::new();
        req.set_invoice(inv.clone());
        let res = anc.detect_duplicate(&req).unwrap();
        match res.result {
            DetectDuplicateReply_Result::OK => {}
            DetectDuplicateReply_Result::DUPLICATE => {
                let warning = "
WARNING: The submitted invoice is very similar\n\
to a recent submission and may be a duplicate.\n\
Use the '--force' option to submit anyway.\n";

                println!("{}", warning);
                return;
            }
        }
    }

    let mut req = invoice::CreateInvoiceRequest::new();
    req.set_invoice(inv);
    let res = inc.create(&req).unwrap();
    println!("invoice number: {}", res.invoice_number);
}

fn handle_list(_: &clap::ArgMatches<'_>, client: InvoicesClient) {
    let req = invoice::ListInvoiceRequest::new();
    let res = client.list(&req).unwrap();
    println!("invoice numbers: {:?}", res.invoice_numbers.into_vec());
}

fn handle_remove(sc: &clap::ArgMatches<'_>, client: InvoicesClient) {
    let mut req = invoice::RemoveInvoiceRequest::new();
    let invoice_number = sc.value_of("invoice").unwrap();
    req.set_invoice_number(invoice_number.to_string());
    let res = client.remove(&req).unwrap();
    println!("invoice number {} removed", res.invoice_number);
}

fn handle_bill(sc: &clap::ArgMatches<'_>, client: RatingClient) {
    let mut req = invoice::RatingRequest::new();
    let company_name = sc.value_of("company").unwrap();
    req.set_company_name(company_name.to_string());
    let rep = client.generate_bill(&req).unwrap();
    println!("bill amount: {}", rep.get_bill_amount());
}

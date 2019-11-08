/// AMF - Associated Methods and Functions
extern crate pretty_env_logger;
extern crate structopt;

use rust_method_count::*;

use std::fs::File;
use std::io::Read;

// Command line
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "AMF - Associated Methods and Functions")]
/// Simple Audio Video Encoding tool
struct Opt {
    /// Input file
    #[structopt(short = "i", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();

    let mut file = File::open(&opt.input).expect("Error on file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    let res = amf_count(syntax);

    if !res.is_empty() {
        println!("Item\t\t\tAMF");
        for (item, amf) in res {
            println!("{}\t\t\t{}", item, amf);
        }
    }
}

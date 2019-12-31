/// AMF - Associated Methods and Functions
extern crate pretty_env_logger;
extern crate serde;
extern crate structopt;

use code_metrics::associated_method::*;
use code_metrics::cognitive_complexity::*;

use std::fs::File;
use std::io::Read;

// Command line
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Rust Code Metrics")]
/// Extract metricts from rust source code
struct Opt {
    /// Input file
    #[structopt(short = "i", parse(from_os_str))]
    input: PathBuf,
    /// Json out
    #[structopt(short = "j", long = "json out format")]
    is_json: bool,
}

fn main() {
    pretty_env_logger::init();

    let opt = Opt::from_args();

    let mut file = File::open(&opt.input).expect("Error on file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let syntax = syn::parse_file(&src).expect("Unable to parse file");

    let mut counter = AMF::from_path(opt.input.parent().unwrap().to_path_buf());
    counter.visit_file(&syntax);

    let mut complexity = CognitiveComplexity::new();
    complexity.visit_file(&syntax);

    if opt.is_json {
        let serialized = serde_json::to_string(&counter.tree).unwrap();

        println!("{}", serialized);
    } else if !counter.tree.is_empty() || !complexity.tree.is_empty() {
        println!("Item\t\t\tCimplexity\t\t\tAMF");
        for (item, amf) in counter.tree {
            let cc = match complexity.tree.get(&item) {
                Some(v) => v.to_string(),
                None => "undefined".to_owned()
            };
            let cc = cc.to_string();
            println!("{}\t\t\t{}\t\t\t{:?}", item, cc, amf.total());
        }
    }
}

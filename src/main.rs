/// AMF - Associated Methods and Functions
extern crate pretty_env_logger;
extern crate serde;
extern crate structopt;

use code_metrics::*;

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
        let metrics = AggregatedMetrics {
            cc: complexity.tree,
            amf: counter.tree,
        };
        let s_metrics = serde_json::to_string(&metrics).unwrap();

        println!("{}", s_metrics);
    } else if !counter.tree.is_empty() || !complexity.tree.is_empty() {
        println!("Item\t\t\tAMF");
        for (item, amf) in counter.tree {
            println!("{}\t\t\t{:?}", item, amf.total());
        }

        println!("==========================");

        println!("Function\t\tComplexity");
        for (f, cc) in complexity.tree {
            println!("{}\t\t\t{}", f, cc);
        }
    }
}

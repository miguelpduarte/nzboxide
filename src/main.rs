use clap::Parser;
use std::fs;

/// Downloads data using NZB files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_hint = clap::ValueHint::FilePath)]
    nzb_location: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);

    let nzb_file_data =
        fs::read_to_string(args.nzb_location).expect("Could not find provided NZB file");

    let parsed_nzb = quick_xml::de::from_str::<nzboxide::parser::NZBData>(&nzb_file_data)
        .expect("Could not parse NZB file");
    println!("parsed nzb: {:#?}", &parsed_nzb);
}

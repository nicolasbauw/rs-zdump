
extern crate tzparse;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "My Rust version of zdump")]
struct Opt {
    // List transitions verbosely
    #[structopt(short = "v")]
    verbose: bool,

    // Timezone
    timezone: String,

    #[structopt(short = "c")]
    year: Option<i32>,
}

fn main() {
    let opt = Opt::from_args();
    //println!("Verbose={}, timezone={}, year={:?}", opt.verbose, opt.timezone, opt.year);
    match tzparse::get(opt.timezone, opt.year) {
        Some(tz) => println!("{:?}", tzparse::worldtime(tz).unwrap()),
        None => println!("Timezone not found")
    };*/
}

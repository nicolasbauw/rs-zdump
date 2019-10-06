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

    // Year selection
    #[structopt(short = "y")]
    year: Option<i32>,
}

fn main() {
    // Getting cmdline args
    let opt = Opt::from_args();

    // Parsing timezone's TZfile
    let timechanges = match tzparse::get(&opt.timezone, opt.year) {
        Some(tc) => tc,
        None => return
    };
    let tzdata = match tzparse::worldtime(&timechanges) {
        Some(tz) => tz,
        None => return
    };

    //No verbose, no year provided
    if opt.verbose == false && opt.year == None {
        println!("{} {} {}", &opt.timezone, tzdata.datetime.to_rfc2822(), tzdata.abbreviation);
    } else {
            println!("Timechanges for {} in {} (UTC):", &opt.timezone, opt.year.unwrap());
            for i in &timechanges {
                println!("{}", i.time.format("%a %e %b %T %Y").to_string());
            }
        }
}

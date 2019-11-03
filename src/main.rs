extern crate tzparse;
use structopt::StructOpt;
use chrono::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(about = "My Rust version of zdump")]
struct Opt {
    // Timezone
    #[structopt(default_value = "Europe/Paris")]
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
        None => { println!("No data for requested timezone"); return }
    };
    let tzdata = match tzparse::worldtime(&timechanges) {
        Some(tz) => tz,
        None => return
    };

    let year: i32 = match opt.year {
        Some(y) => y,
        None => {
            let d = Utc::now();
            d.format("%Y").to_string().parse().unwrap()
                }
    };
    
    println!("{} {} {}, week number: {}\n", &opt.timezone, tzdata.datetime.to_rfc2822(), tzdata.abbreviation, tzdata.datetime.format("%W").to_string());
    println!("Timechanges (or last timechange) for {} in {} (UTC):", &opt.timezone, year);
    for i in &timechanges {
        println!("{}", i.time.format("%a %e %b %T %Y").to_string());
    }
}

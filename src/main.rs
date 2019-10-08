extern crate tzparse;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "My Rust version of zdump")]
struct Opt {
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

    match opt.year {
        None => println!("{} {} {}, week number: {}", &opt.timezone, tzdata.datetime.to_rfc2822(), tzdata.abbreviation, tzdata.datetime.format("%W").to_string()),
        Some(y) => {
            println!("Timechanges for {} in {} (UTC):", &opt.timezone, y);
            for i in &timechanges {
                println!("{}", i.time.format("%a %e %b %T %Y").to_string());
            }
        }
    }
}


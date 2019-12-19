extern crate tzparse;
use structopt::StructOpt;

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
    let timechanges = match tzparse::get_timechanges(&opt.timezone, opt.year) {
        Some(tc) => tc,
        None => return
    };
    let tzdata = match tzparse::get_zoneinfo(&timechanges) {
        Some(tz) => tz,
        None => return
    };

    match opt.year {
        None => println!("{} {} {}, week number: {}", &opt.timezone, tzdata.datetime.to_rfc2822(), tzdata.abbreviation, tzdata.datetime.format("%W").to_string()),
        Some(y) => {
            for i in &timechanges {
                // Timechange's year
                let cy: i32=  i.time.format("%Y").to_string().parse().unwrap();
                // Timechange's year does not match selected year ? we do not display it
                if cy == y {
                    println!("{} {} UT -> {} gmtoff={} DST: {}", &opt.timezone, i.time.format("%a %e %b %T %Y").to_string(), i.abbreviation, i.gmtoff, i.isdst);
                }
            }
        }
    }
}


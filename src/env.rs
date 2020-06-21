use std::env;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static HELP: &str = "An alternative version of zdump

USAGE:
    zdump [zonename] [OPTIONS]

OPTIONS:
    -a               View all zone's timechanges
    -y <year>        View year's timechanges
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <zonename>    The timezone to query [default: Europe/Paris]";

pub struct Args {
    // The timezone to query
    pub zonename: Option<String>,
    // View all zone's timechanges
    pub a: bool,
    // Specify a year
    pub y: bool,
    // View year's timechanges
    pub year: Option<i32>,
}

pub fn get_cargs() -> Option<Args> {
    let mut a: Vec<String> = env::args().collect();
    let b = a.clone();
    let mut parsed_args: Vec<usize> = Vec::new();
    let mut comparator: Vec<usize> = Vec::new();
    // Zonename, -a, -y, -h, year
    let mut args: (&str, bool, bool, Option<i32>) = ("", false, false, None);

    for i in 1..a.len() {
        match a[i].as_ref() {
            "-a" => {
                args.1 = true;
                a[i].truncate(1);
            }
            "-y" => {
                args.2 = true;
                match a[i + 1].parse::<i32>() {
                    Ok(y) => {
                        args.3 = Some(y);
                    }
                    Err(_) => println!("Invalid year"),
                };
                parsed_args.push(i + 1);
                a[i].truncate(1);
            }
            "-h" => {
                println!("{}", HELP);
                return None;
            }
            "-V" => {
                println!("{}", VERSION);
                return None;
            }
            &_ => {
                a[i].truncate(1);
            }
        }
        match a[i].as_ref() {
            "-" => {
                parsed_args.push(i);
            }
            &_ => {}
        }
    }

    parsed_args.sort();
    for _ in 0..((a.len()) - 1) - parsed_args.len() {
        parsed_args.push(0)
    }
    for i in 1..a.len() {
        comparator.push(i);
    }
    for i in 0..parsed_args.len() {
        if parsed_args[i] != comparator[i] && a[comparator[i]].parse::<u32>().is_err() == true {
            args.0 = &b[comparator[i]];
            break;
        }
    }

    #[cfg(debug_assertions)]
    {
        println!("Cmdline args : {:?}", b);
        println!("Parsed args : {:?}", parsed_args);
        println!("Comparator : {:?}", comparator);
        println!(
            "Zone : {:?}, -a : {}, -y : {}, year : {:?}",
            args.0, args.1, args.2, args.3
        );
    }

    Some(Args {
        zonename: if args.0 == "" {
            None
        } else {
            Some((args.0).to_string())
        },
        a: args.1,
        y: args.2,
        year: args.3,
    })
}

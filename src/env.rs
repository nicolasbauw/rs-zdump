use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const HELP: &str = "USAGE:
    zdump [OPTIONS] [ZONENAME]

OPTIONS:
    -a               Prints all transitions
    -y <year>        Prints year's transitions
    -w               Prints week number
    -r               Prints TZFile raw contents
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    [ZONENAME]       The timezone to query";

pub struct Args {
    // The timezone to query
    pub zonename: Option<String>,
    // View all zone's timechanges
    pub a: bool,
    // Display week number
    pub w: bool,
    // View year's timechanges
    pub year: Option<i32>,
}

pub fn get_cargs() -> Option<Args> {
    let mut a: Vec<String> = env::args().collect();
    let b = a.clone();
    let mut parsed_args: Vec<usize> = Vec::new();
    let mut comparator: Vec<usize> = Vec::new();
    // Zonename, -a, -y, year, -w, -r
    let mut args: (&str, bool, bool, Option<i32>, bool, bool) = ("", false, false, None, false, false);

    for i in 1..a.len() {
        match a[i].as_ref() {
            "-a" => {
                args.1 = true;
                a[i].truncate(1);
            }
            "-w" => {
                args.4 = true;
                a[i].truncate(1);
            }
            "-y" => {
                args.2 = true;
                match a[i + 1].parse::<i32>() {
                    Ok(y) => {
                        args.3 = Some(y);
                    }
                    Err(_) => {
                        println!("Invalid year");
                        return None;
                    }
                };
                parsed_args.push(i + 1);
                a[i].truncate(1);
            }
            "-r" => {
                args.5 = true;
                a[i].truncate(1);
            }
            "-h" | "--help" => {
                println!("{}", HELP);
                return None;
            }
            "-V" | "--version" => {
                println!("zdump (Rust) {}", VERSION);
                return None;
            }
            &_ => {
                a[i].truncate(1);
            }
        }
        if let "-" =  a[i].as_ref() {
            parsed_args.push(i);
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
            "Zone : {:?}, -a : {}, -y : {}, -w : {}, -r : {},  year : {:?}",
            args.0, args.1, args.2, args.4, args.5, args.3
        );
    }

    Some(Args {
        zonename: if args.0 == "" {
            None
        } else {
            Some((args.0).to_string())
        },
        a: args.1,
        w: args.4,
        year: args.3,
    })
}

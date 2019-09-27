extern crate tzparse;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let requested_timezone = &args[1];
    let year = &args[2].parse().unwrap();
    match tzparse::get(&requested_timezone, *year) {
        Some(tz) => println!("{:?}", tzparse::worldtime(tz).unwrap()),
        None => println!("Timezone not found")
    };
}

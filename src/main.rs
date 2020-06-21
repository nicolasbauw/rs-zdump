//! An alternative version of the zdump utility.
//!
//! Without arguments, shows current time and data about the Europe/Paris zone:
//! ```text
//! zdump
//! Europe/Paris Thu, 19 Dec 2019 13:48:47 +0100 CET, week number: 51
//! ````
//! 
//! Same thing when specifying a zonename: 
//! ```text
//! zdump America/Phoenix
//! America/Phoenix Thu, 19 Dec 2019 05:52:04 -0700 MST, week number: 51
//! ````
//!
//! When specifying a zonename and a year, outputs timechanges for that year: 
//! ```text
//! zdump Europe/Paris -y 2020
//! Europe/Paris Sun 29 Mar 01:00:00 2020 UT -> CEST gmtoff=7200 DST: true
//! Europe/Paris Sun 25 Oct 01:00:00 2020 UT -> CET gmtoff=3600 DST: false 
//! ```
//! 
//! To display all zone's timechanges: 
//! ```text
//! zdump America/Phoenix -a     
//! America/Phoenix Sun 18 Nov 19:00:00 1883 UT -> MST gmtoff=-25200 DST: false
//! America/Phoenix Sun 31 Mar 09:00:00 1918 UT -> MDT gmtoff=-21600 DST: true
//! America/Phoenix Sun 27 Oct 08:00:00 1918 UT -> MST gmtoff=-25200 DST: false
//! America/Phoenix Sun 30 Mar 09:00:00 1919 UT -> MDT gmtoff=-21600 DST: true
//! America/Phoenix Sun 26 Oct 08:00:00 1919 UT -> MST gmtoff=-25200 DST: false
//! America/Phoenix Mon  9 Feb 09:00:00 1942 UT -> MWT gmtoff=-21600 DST: true
//! America/Phoenix Sat  1 Jan 06:01:00 1944 UT -> MST gmtoff=-25200 DST: false
//! America/Phoenix Sat  1 Apr 07:01:00 1944 UT -> MWT gmtoff=-21600 DST: true
//! America/Phoenix Sun  1 Oct 06:01:00 1944 UT -> MST gmtoff=-25200 DST: false
//! America/Phoenix Sun 30 Apr 09:00:00 1967 UT -> MDT gmtoff=-21600 DST: true
//! America/Phoenix Sun 29 Oct 08:00:00 1967 UT -> MST gmtoff=-25200 DST: false
//! ```
//! 
//! It uses system TZfiles (default location on Linux and Macos /usr/share/zoneinfo). On Windows, default expected location is HOME/.zoneinfo. You can override the TZfiles default location with the TZFILES_DIR environment variable. Example for Windows:
//!
//! $env:TZFILES_DIR="C:\Users\nbauw\Dev\rs-tzfile\zoneinfo\"; zdump

mod env;
use env::get_cargs;
use tzparse::TzError;

fn main() -> Result<(), TzError> {
    // Getting cmdline args
    let opt = get_cargs();
    let z = match opt.zonename {
        Some(s) => String::from(s),
        None => String::from("Europe/Paris")
    };

    let tzdata = tzparse::get_zoneinfo(&z)?;

    // Provided year (or current year by default)
    let year: i32 = match opt.year {
        Some(y) => y,
        None => {
            tzdata.datetime.format("%Y").to_string().parse()?
                }
            };

    // Parsing timechanges
    let timechanges = tzparse::get_timechanges(&z, if !opt.a { Some(year) } else { None })?;

    if opt.a {
        for i in &timechanges {
                println!("{} {} UT -> {} gmtoff={} DST: {}", z, i.time.format("%a %e %b %T %Y").to_string(), i.abbreviation, i.gmtoff, i.isdst);
        }
        return Ok(())
    }

    match opt.year {
        None => println!("{} {} {}, week number: {}", z, tzdata.datetime.to_rfc2822(), tzdata.abbreviation, tzdata.week_number.to_string()),
        Some(y) => {
            for i in &timechanges {
                // Timechange's year
                let cy: i32=  i.time.format("%Y").to_string().parse()?;
                // Timechange's year does not match selected year ? we do not display it
                if cy == y {
                    println!("{} {} UT -> {} gmtoff={} DST: {}", z, i.time.format("%a %e %b %T %Y").to_string(), i.abbreviation, i.gmtoff, i.isdst);
                }
            }
        }
    }
    Ok(())
}


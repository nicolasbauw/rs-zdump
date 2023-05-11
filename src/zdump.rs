use crate::env::get_cargs;
use libtzfile::{Tz, TzError};
use std::error::Error;

pub fn zdump() -> Result<(), Box<dyn Error>> {
    // Getting cmdline args
    let Some(opt) = get_cargs() else { return Ok(()) };
    let Some(z) = opt.zonename else { return Ok(()) };

    let tz = Tz::new(&z)?;
    let tzdata = tz.zoneinfo()?;

    // Provided year (or current year by default)
    let year: i32 = match opt.year {
        Some(y) => y,
        None => tzdata.datetime.format("%Y").to_string().parse()?,
    };

    // Parsing timechanges
    let timechanges = match tz.transition_times(if !opt.a { Some(year) } else { None }) {
        Ok(tt) => tt,
        Err(TzError::NoData) => Vec::new(),
        Err(e) => return Err(Box::new(e)),
    };

    if opt.r {
        println!("{:?}\n", tz);
        println!("{:?}", tzdata);
        return Ok(());
    }

    if opt.a {
        for i in &timechanges {
            // we do not display the transition time if timestamp = 0x7FFFFFFF (Tue 19 Jan 03:14:07 2038)
            if i.time.timestamp() != 2147483647 {
                println!(
                    "{} {} UT -> {}, utc_offset={}, DST: {}",
                    tzdata.timezone,
                    i.time.format("%a, %d %b %Y %T"),
                    i.abbreviation,
                    i.utc_offset,
                    i.isdst
                );
            }
        }
        return Ok(());
    }

    match opt.year {
        None => {
            if opt.w {
                println!(
                    "{} {} {}, week number: {}",
                    tzdata.timezone,
                    tzdata.datetime.to_rfc2822(),
                    tzdata.abbreviation,
                    tzdata.week_number
                )
            } else {
                println!(
                    "{} {} {}",
                    tzdata.timezone,
                    tzdata.datetime.to_rfc2822(),
                    tzdata.abbreviation
                )
            }
        }
        Some(y) => {
            for i in &timechanges {
                // Timechange's year
                let cy: i32 = i.time.format("%Y").to_string().parse()?;
                // Timechange's year does not match selected year ? we do not display it
                if cy == y {
                    println!(
                        "{} {} UT -> {}, utc_offset={}, DST: {}",
                        tzdata.timezone,
                        i.time.format("%a, %d %b %Y %T"),
                        i.abbreviation,
                        i.utc_offset,
                        i.isdst
                    );
                }
            }
        }
    }
    Ok(())
}

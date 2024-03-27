//! A Rust version of the zdump utility.
//!
//! Output example when specifying only a zonename:
//! ```text
//! zdump /usr/share/zoneinfo/Europe/Paris
//! Europe/Paris Tue, 04 Aug 2020 23:26:09 +0200 CEST
//! ````
//!
//! To display week number:
//! ```text
//! zdump /usr/share/zoneinfo/Europe/Paris -w
//! Europe/Paris Tue, 04 Aug 2020 23:26:09 +0200 CEST, week number: 32
//! ````
//!
//! When specifying a zonename and a year, outputs transition times for that year:
//! ```text
//! zdump /usr/share/zoneinfo/Europe/Paris -y 2020
//! Europe/Paris Sun, 29 Mar 2020 01:00:00 UT -> CEST, utc_offset=7200, DST: true
//! Europe/Paris Sun, 25 Oct 2020 01:00:00 UT -> CET, utc_offset=3600, DST: false
//! ```
//!
//! To display all zone's transition times:
//! ```text
//! zdump /usr/share/zoneinfo/America/Phoenix -a
//! America/Phoenix Sun, 18 Nov 1883 19:00:00 UT -> MST, utc_offset=-25200, DST: false
//! America/Phoenix Sun, 31 Mar 1918 09:00:00 UT -> MDT, utc_offset=-21600, DST: true
//! America/Phoenix Sun, 27 Oct 1918 08:00:00 UT -> MST, utc_offset=-25200, DST: false
//! America/Phoenix Sun, 30 Mar 1919 09:00:00 UT -> MDT, utc_offset=-21600, DST: true
//! America/Phoenix Sun, 26 Oct 1919 08:00:00 UT -> MST, utc_offset=-25200, DST: false
//! America/Phoenix Mon, 09 Feb 1942 09:00:00 UT -> MWT, utc_offset=-21600, DST: true
//! America/Phoenix Sat, 01 Jan 1944 06:01:00 UT -> MST, utc_offset=-25200, DST: false
//! America/Phoenix Sat, 01 Apr 1944 07:01:00 UT -> MWT, utc_offset=-21600, DST: true
//! America/Phoenix Sun, 01 Oct 1944 06:01:00 UT -> MST, utc_offset=-25200, DST: false
//! America/Phoenix Sun, 30 Apr 1967 09:00:00 UT -> MDT, utc_offset=-21600, DST: true
//! America/Phoenix Sun, 29 Oct 1967 08:00:00 UT -> MST, utc_offset=-25200, DST: false
//! ```
//!
//! To display raw and debug data:
//! ```text
//! zdump /usr/share/zoneinfo/EST -r
//! Tz { tzh_timecnt_data: [], tzh_timecnt_indices: [], tzh_typecnt: [Ttinfo { tt_gmtoff: -18000, tt_isdst: 0, tt_abbrind: 0 }], tz_abbr: ["EST"], name: "EST" }
//! Tzinfo { timezone: "EST", utc_datetime: 2021-07-09T20:57:50.343062Z, datetime: 2021-07-09T15:57:50.343062-05:00, dst_from: None, dst_until: None, dst_period: false, raw_offset: -18000, dst_offset: 0, utc_offset: -05:00, abbreviation: "EST", week_number: 27 }
//! ```
//!
//! -h and -V prints help and version information, respectively.
//!

mod args;
mod zdump;
use std::process;

fn main() {
    if let Err(e) = zdump::zdump() {
        println!("{}", e);
        process::exit(1);
    }
}

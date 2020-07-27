# zdump

[![Current Crates.io Version](https://img.shields.io/crates/v/zdump.svg)](https://crates.io/crates/zdump)
[![Downloads badge](https://img.shields.io/crates/d/zdump.svg)](https://crates.io/crates/zdump)

A Rust version of the zdump utility.

Output example when specifying only a zonename:
```
zdump /usr/share/zoneinfo/America/Phoenix
America/Phoenix Thu, 19 Dec 2019 05:52:04 -0700 MST, week number: 51
````

When specifying a zonename and a year, outputs timechanges for that year:
```text
zdump /usr/share/zoneinfo/Europe/Paris -y 2020
Europe/Paris Sun 29 Mar 01:00:00 2020 UT -> CEST gmtoff=7200 DST: true
Europe/Paris Sun 25 Oct 01:00:00 2020 UT -> CET gmtoff=3600 DST: false
```

To display all zone's transition times:
```
zdump /usr/share/zoneinfo/America/Phoenix -a
America/Phoenix Sun 18 Nov 19:00:00 1883 UT -> MST gmtoff=-25200 DST: false
America/Phoenix Sun 31 Mar 09:00:00 1918 UT -> MDT gmtoff=-21600 DST: true
America/Phoenix Sun 27 Oct 08:00:00 1918 UT -> MST gmtoff=-25200 DST: false
America/Phoenix Sun 30 Mar 09:00:00 1919 UT -> MDT gmtoff=-21600 DST: true
America/Phoenix Sun 26 Oct 08:00:00 1919 UT -> MST gmtoff=-25200 DST: false
America/Phoenix Mon  9 Feb 09:00:00 1942 UT -> MWT gmtoff=-21600 DST: true
America/Phoenix Sat  1 Jan 06:01:00 1944 UT -> MST gmtoff=-25200 DST: false
America/Phoenix Sat  1 Apr 07:01:00 1944 UT -> MWT gmtoff=-21600 DST: true
America/Phoenix Sun  1 Oct 06:01:00 1944 UT -> MST gmtoff=-25200 DST: false
America/Phoenix Sun 30 Apr 09:00:00 1967 UT -> MDT gmtoff=-21600 DST: true
America/Phoenix Sun 29 Oct 08:00:00 1967 UT -> MST gmtoff=-25200 DST: false
```
-h and -V prints help and version information, respectively.

License: GPL-3.0

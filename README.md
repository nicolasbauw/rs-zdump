# zdump

An alternative version of the zdump utility.

Without arguments, shows actual time and data about the Europe/Paris zone:
```
zdump
Europe/Paris Thu, 19 Dec 2019 13:48:47 +0100 CET, week number: 51
````

Same thing when specifying a zonename:
```text
zdump America/Phoenix
America/Phoenix Thu, 19 Dec 2019 05:52:04 -0700 MST, week number: 51
````

When specifying a zonename and a year, outputs timechanges for that year:
```text
zdump Europe/Paris -y 2020
Europe/Paris Sun 29 Mar 01:00:00 2020 UT -> CEST gmtoff=7200 DST: true
Europe/Paris Sun 25 Oct 01:00:00 2020 UT -> CET gmtoff=3600 DST: false
```

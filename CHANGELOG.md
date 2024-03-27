# Changelog

### v1.3.5 (2024-03-27)

- [Fixed] Crash when -y was not followed by anything
- [Changed] updated to libtzfile v3.0.0

### v1.3.0 (2021-07-10)

- [Fixed] Incorrect display of timezones without transition times
- [Added] -r commutator for raw/debug data display

### v1.2.1 (2020-08-04)

- [Fixed] Date formatting differences between -a, -y and no option

### v1.2.0 (2020-07-29)

- [Changed] week number is no longer displayed by default
- [Added] week number display commutator (-w)
- [Changed] better formatting of error messages

### v1.1.0 (2020-06-26)

- [Changed] Removed structopt dependency (3 times lighter executable !)
- [Changed] Tzfiles location can no longer be customized by ENV
- [Changed] Zonename file must be entered in absolute path, removed tzfiles default location

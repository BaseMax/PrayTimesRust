# praytimes

Prayer Times Calculator cli for Rust Based on
[Praytimes.org](https://praytimes.org)

Dont forget to give [Praytimes.org](https://praytimes.org) based on their
license :

```txt
TERMS OF USE:
	Permission is granted to use this code, with or
	without modification, in any website or application
	provided that credit is given to the original work
	with a link back to PrayTimes.org.

This program is distributed in the hope that it will
be useful, but WITHOUT ANY WARRANTY.
```

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY.

## installation

```bash
cargo install praytimes-cli
```
## usage

```
cli Prayertime calculator

Usage: praytimes-cli [OPTIONS] --method <METHOD> --latitude <LATITUDE> --longitude <LONGITUDE>

Options:
  -m, --method <METHOD>        calculation methods [possible values: mwl, isna, egypt, makkah, karachi, tehran, jafari]
      --latitude <LATITUDE>    location's latitude
      --longitude <LONGITUDE>  location's longitude
  -e, --elevation <ELEVATION>  elevation from surrounding terrain [default: 0]
  -d, --date <DATE>            date for calculation ( default is today ) [default: 2023-08-18]
  -f, --format <FORMAT>        strftime compatible format [default: %H:%M:%S]
  -j, --json
  -h, --help                   Print help
  -V, --version                Print version
```

## Credits

PrayTimes.js is based on [PrayTimes](http://praytimes.org). Cities dataset from
[countries-states-cities-database](https://github.com/dr5hn/countries-states-cities-database).

## License

GNU GPL v3.0 - see LICENSE

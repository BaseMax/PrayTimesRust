# praytimes

Prayer Times Calculator for Rust Based on [Praytimes.org](https://praytimes.org)

Dont forget to give [Praytimes.org](https://praytimes.org) based on their license :

```txt
TERMS OF USE:
	Permission is granted to use this code, with or
	without modification, in any website or application
	provided that credit is given to the original work
	with a link back to PrayTimes.org.

This program is distributed in the hope that it will
be useful, but WITHOUT ANY WARRANTY.
```

This program is distributed in the hope that it will
be useful, but WITHOUT ANY WARRANTY.
## Installation

```
cargo add praytimes
```

## Example

```rs
let calculator = &Calculator::new(
    methods::ISLAMIC_SOCIETY_OF_NORTH_AMERICA, // calculation method or parameters
    TuneOffsets {
        fajr: Some(7.0), // time for precaution
        ..Default::default()
    },
);
let output: PraytimesOutput = calculator
.calculate(
    &Location {
        longitude: 43.0,
        latitude: 30.0,
        elevation: 0.0, // elevation of that point in meters
    },
    &NaiveDate::from_ymd_opt(2022, 11, 11),
);
```

## Docs

see [lib.rs docs](https://lib.rs/praytimes)

## Credits

PrayTimes.js is based on [PrayTimes](http://praytimes.org). Cities dataset from
[countries-states-cities-database](https://github.com/dr5hn/countries-states-cities-database).

## License

GNU GPL v3.0 - see LICENSE

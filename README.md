# Praytimes

PrayTimesRust is a powerful and versatile Muslim prayer time calculator
implemented in Rust. This project is built upon the PrayTimes.org library, a
widely recognized resource for accurate prayer time calculations. Whether you're
developing a website, application, or any other digital platform, PrayTimesRust
offers a seamless way to integrate accurate prayer time calculations for Muslims
worldwide.

> Prayer Times Calculator for Rust Based on
> [Praytimes.org](https://praytimes.org). Dont forget to give
> [Praytimes.org](https://praytimes.org) based on their license :

TERMS OF USE: Permission is granted to use this code, with or without
modification, in any website or application provided that credit is given to the
original work with a link back to PrayTimes.org.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY.

## Projects

- [praytimes lib](#library) - praytime calculator library
- [praytimes-kit](#praytimes-kit) - praytime kit for calculation, webservice and
  notification daemon

## Library

see documentation in [docs.rs](https://docs.rs/praytimes)

## PrayTimes Kit

PrayTimesKit is a versatile toolkit for calculating Muslim prayer times and
scheduling notifications around them. It is built on top of
[PrayTimes.org](https://praytimes.org), a widely used prayer time calculation
library.

### Features

PrayTimesKit provides the following major features:

#### Accurate Prayer Time Calculations

- Calculates prayer times for any location using coordinates
- Supports several calculation methods like Muslim World League, ISNA, Egypt,
  Makkah, Karachi, Tehran, Jafari
- Highly configurable with customizable calculation parameters
- Support for Shia calculation methods

#### Prayer Time daemon

- Scheduled notifications for prayer times using system notifications
- Configure notifications to be triggered before/after any prayer time by a
  specified duration
- Notification text can contain dynamic info like prayer name, prayertime itself
  (which is different with the time that program runs because of the time diff),
  time difference

#### Prayer Time API

- HTTP API to get prayer times for a location and date
- Request JSON payload can configure calculation parameters, timezone,
  formatting
- Returns prayer times formatted as JSON or plain text

### Usage

#### Configuration

The configuration file is a JSON file that specifies the location, calculation
parameters, notification commands, and time formats.

##### Format

The `format` field specifies how prayer times should be formatted. It uses
[strftime](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html)
syntax.

Default is `%T` which formats the time as `HH:MM:SS`.

##### Location

The `location` field specifies the location coordinates.

```json
"location": {
  "longitude": 83.62,
  "latitude": 43.35
}
```

##### Parameters

The `parameters` field configures the calculation parameters.

There are two possible formats:

###### Specify Calculation Method

Use `method` to pick from a predefined calculation method:

```json
"parameters": {
  "method": "Tehran"
}
```

Supported methods are:

- `MWL` - Muslim World League
- `ISNA` - Islamic Society of North America
- `Egypt` - Egyptian General Authority of Survey
- `Makkah` - Umm al-Qura, Makkah
- `Karachi` - University of Islamic Sciences, Karachi
- `Tehran` - Institute of Geophysics, University of Tehran
- `Jafari` - Shia Ithna-Ashari, Leva Institute, Qum

###### Customize Parameters

Specify the `imsak`, `fajr`, `dhuhr` etc parameters directly:

```json
"parameters": {
  "imsak": 10,
  "fajr": 15,
  "dhuhr": 0,
  "asr": 1,
  "maghrib": 0,
  "isha": 10,
  "midnight": "Standard",
  "highLats": "NightMiddle"
}
```

##### Commands (configuration field)

The `commands` field contains the notification commands to run. Each command
specifies:

- `praytime` - which prayer time to trigger on
- `time_diff` - minutes to add/subtract from the pray time
- `cmd` - the command to run

The command text can contain variables like `$TYPE`, `$TIME`, `$DIFF` which will
be interpolated.

##### Tune

The `tune` field allows adjusting prayer times.and is a key-value pair of
praytime names with a number the number is either negative or positive and is
represented in minutes

##### Cli sub commands

PrayTimesKit provides the following commands:

#### `serve`


Starts the HTTP server for the prayer time API.

```
praytimes-kit serve
```

You can configure it using the `PORT` and `HOST` environment variables to run on a specific port or listen on specific ip.

You can also use Docker to run it:

```sh
docker run -p 3535:3535 -it basemax/praytimes:1.0
```

Or with Docker Compose:

```yml
version: "3"

services:

  praytimes:
    image: basemax/praytimes:1.0
    ports:
      - "3535:3535"
    environment:
      HOST: "0.0.0.0"
```

This runs the `basemax/praytimes:1.0` Docker image, exposes port 3535, and sets the `HOST` env var so it listens on all interfaces.

Some additional Compose examples:

- Set a custom port:

```
ports:
  - "8080:3535"
```

#### `daemon`

Runs the prayer daemon based on a configuration file.

```
praytimes-kit daemon path/to/config.json
```

The configuration file specifies the location, calculation parameters, commands
to run, and time formats.

Here is an example configuration:

```json
{
  "location": {
    "longitude": 83.62,
    "latitude": 43.35
  },

  "parameters": {
    "method": "Tehran"
  },

  "format": "%T",

  "commands": [
    {
      "praytime": "dhuhr",
      "time_diff": 180,
      "cmd": "notify-send 'Adhan: Its $TYPE at $TIME + $DIFF'"
    },

    {
      "praytime": "maghrib",
      "time_diff": 180,
      "cmd": "notify-send 'Adhan: Its $TYPE at $TIME + $DIFF, the darkness is coming'"
    }
  ]
}
```

##### systemd service

You can define a systemd service for the praytimes daemon to control it using
systemd

```txt
[Unit]
Description=praytimes daemon

[Service]
Environment="PRAYTIMES_LOG=info"
ExecStart=/path/to/praytimes-kit daemon /etc/praytimes/praytimes.json
Restart=always
```

then you can run

```sh
sudo systemctl enable praytimes;
sudo systemctl start praytimes;
```

#### `calculate`

Simple CLI to get prayer times for a location on a specific date. Supports text
and JSON output formats

Gets prayer times for a location on a specific date.

```
praytimes-kit calculate --config path/to/config.json --date 2021-03-15
```

The configuration file contains the location details and calculation parameters.
Note that you can use one configuration file for both daemon and calculate
commands the unnecessary fields won't effect calculate command

##### help

```txt
simple cli to show praytimes for specific day

Usage: praytimes-kit calculate [OPTIONS] --config <CONFIG>

Options:
  -c, --config <CONFIG>  configuration file
  -d, --date <DATE>      date for calculation ( default is today ) [default: 2023-09-04]
  -f, --format <FORMAT>  strftime compatible format [default: %H:%M:%S]
  -j, --json             whether to output as json format or not
  -h, --help             Print help
  -V, --version          Print version
```

### Integration

PrayTimesKit provides building blocks that can be easily integrated into other
apps and services:

- Use the **praytimes** crate in your Rust project to calculate prayer times
- Call the **HTTP API** from any language to get prayer times
- Run the **CLI** on your desktop to get praytimes

### Credits

PrayTimesKit is built on top of [PrayTimes.org](https://praytimes.org). The time
calculation code is adapted from the original JavaScript library.

### License

GNU GPL v3.0

Let me know if you would like me to explain or expand on any section! I tried to
provide an overview of the key capabilities and usage.

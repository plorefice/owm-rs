# owm-rs
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An OpenWeatherMap API client written in Rust.

## Overview

`owm` provides access to OpenWeatherMap API to fetch current weather and forecasts. It supports querying by city name or city ID.
At the moment, only the JSON response API is supported, but contributions for other formats are welcome!

This crate relies on [hyper](https://crates.io/crates/hyper) to handle all the HTTP stuff, so you'll need that.
You will also need to provide an API key, which can obtained at the following link: http://openweathermap.org/appid.

## Example

```rust
extern crate hyper;
extern crate owm;

use owm::{WeatherHub, Error};

fn main() {
    let hub = WeatherHub::new(hyper::Client::new(), "YOUR_API_KEY".to_string());
    let res = hub.current().by_name("London");

    match res {
        Err(e) => match e {
              Error::HttpError(_)
            | Error::BadRequest(_)
            | Error::JsonDecodeError(_, _)
            | Error::Failure(_) => println!("{:?}", e),
        },
        Ok(res) => println!("{:?}", res),
    }
}
```

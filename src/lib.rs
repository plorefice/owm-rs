//! This crate provides access to OpenWeatherMap's API.
//!
//! At the moment, JSON is the only supported data format. To use the API, you
//! also need to provide an API key, which can obtained at the following link:
//! http://openweathermap.org/appid.
//!
//! # Features
//!
//! Right now, the crate supports:
//!
//! * Querying the current weather via [CurrentWeatherQuery](struct.CurrentWeatherQuery.html)
//!  * By city name
//!  * By city ID
//!
//! # Example
//!
//! ```test_harness,no_run
//! extern crate hyper;
//! extern crate owm;
//!
//! use owm::{WeatherHub, Error};
//!
//! # #[test] fn eval() {
//! let hub = WeatherHub::new(hyper::Client::new(), "YOUR_API_KEY".to_string());
//! let res = hub.current().by_name("London", Some("UK"));
//!
//! match res {
//!     Err(e) => match e {
//!           Error::HttpError(_)
//!         | Error::BadRequest(_)
//!         | Error::JsonDecodeError(_, _)
//!         | Error::Failure(_) => println!("{:?}", e),
//!     },
//!     Ok(res) => println!("{:?}", res),
//! }
//! # }
//! ```

#[macro_use]
extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json as json;
extern crate url;

use std::mem;
use std::io::Read;
use std::cell::RefCell;
use std::borrow::BorrowMut;

#[derive(Debug)]
pub enum Error {
    /// An error occurred while performing the HTTP request.
    HttpError(hyper::Error),

    /// The request was not correctly understood by the server. Details included.
    BadRequest(ErrorResponse),

    /// Invalid JSON received from the server, likely caused by an API change.
    JsonDecodeError(String, json::Error),

    /// Indicates an HTTP repsonse with a non-success status code.
    Failure(hyper::client::Response),
}

/// A universal result type used as return for all calls.
pub type Result<T> = std::result::Result<T, Error>;

/// Central hub to access all weather-related facilities.
pub struct WeatherHub<C> {
    client: RefCell<C>,
    key: String,
}

impl<'a, C> WeatherHub<C>
    where C: BorrowMut<hyper::Client>
{
    /// Creates a new WeatherHub which will use the provided client to perform
    /// its requests. It also requires an OWM API key.
    pub fn new<S: Into<String>>(client: C, key: S) -> WeatherHub<C> {
        WeatherHub {
            client: RefCell::new(client),
            key: key.into(),
        }
    }

    /// Provides access to the current-weather facilities.
    pub fn current(&'a self) -> CurrentWeatherQuery<'a, C> {
        CurrentWeatherQuery {
            hub: &self,
            _builder: QueryBuilder::new().param("appid", self.key.clone()),
        }
    }
}

/// Rectangle specified by geographic coordinates (latitude and longitude).
#[derive(Debug)]
pub struct BoundingBox {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

/// Query builder for the Current Weather API.
pub struct CurrentWeatherQuery<'a, C>
    where C: 'a
{
    hub: &'a WeatherHub<C>,
    _builder: QueryBuilder<'a>,
}

impl<'a, C> CurrentWeatherQuery<'a, C>
    where C: BorrowMut<hyper::Client>
{
    /// Query current weather by passing a city name and an optional country code.
    pub fn by_name<S: Into<String>>(mut self,
                                    city: S,
                                    country: Option<S>)
                                    -> Result<(hyper::client::Response, WeatherInfo)> {
        let q = match country {
            None => city.into(),
            Some(code) => format!("{},{}", city.into(), code.into()),
        };

        let query = {
            let b = mem::replace(&mut self._builder, QueryBuilder::new());
            b.method("weather").param("q", q).build()
        };
        self.run_query(query)
    }

    /// Query current weather by passing a city ID. API responds with exact result.
    /// See http://bulk.openweathermap.org/sample/ for a list of city IDs.
    pub fn by_id(mut self, id: i32) -> Result<(hyper::client::Response, WeatherInfo)> {
        let query = {
            let b = mem::replace(&mut self._builder, QueryBuilder::new());
            b.method("weather").param("id", id.to_string()).build()
        };
        self.run_query(query)
    }

    /// Query current weather by passing a ZIP code and an optional country code.
    pub fn by_zip_code<S: Into<String>>(mut self,
                                        zip: i32,
                                        country: Option<S>)
                                        -> Result<(hyper::client::Response, WeatherInfo)> {
        let q = match country {
            None => zip.to_string(),
            Some(code) => format!("{},{}", zip.to_string(), code.into()),
        };

        let query = {
            let b = mem::replace(&mut self._builder, QueryBuilder::new());
            b.method("weather").param("zip", q).build()
        };
        self.run_query(query)
    }

    /// Query current weather by passing geographic coordinates.
    pub fn by_coords(mut self,
                     lat: f32,
                     lon: f32)
                     -> Result<(hyper::client::Response, WeatherInfo)> {
        let query = {
            let b = mem::replace(&mut self._builder, QueryBuilder::new());
            b.method("weather")
                .param("lat", lat.to_string())
                .param("lon", lon.to_string())
                .build()
        };
        self.run_query(query)
    }

    /// Query current weather for cities within the defined rectangle specified
    /// by the bounding box using the given zoom. Server clustering of points
    /// can also be used.
    pub fn by_bounds(mut self,
                     bbox: &BoundingBox,
                     zoom: i32,
                     cluster: bool)
                     -> Result<(hyper::client::Response, WeatherBoxAggregate)> {
        let q = format!("{},{},{},{},{}",
                        bbox.left,
                        bbox.bottom,
                        bbox.right,
                        bbox.top,
                        zoom);

        let query = {
            let b = mem::replace(&mut self._builder, QueryBuilder::new());
            b.method("box/city")
                .param("bbox", q)
                .param("cluster", if cluster { "yes" } else { "no" })
                .build()
        };
        self.run_query(query)
    }

    /// Query current weather for cities laid inside a circle specified by
    /// center point (lan, lot) and expected number of cities withing.
    pub fn by_circle(mut self,
                     lat: f32,
                     lon: f32,
                     count: i32,
                     cluster: bool)
                     -> Result<(hyper::client::Response, WeatherAggregate)> {
        let query = {
            let b = mem::replace(&mut self._builder, QueryBuilder::new());
            b.method("find")
                .param("lat", lat.to_string())
                .param("lon", lon.to_string())
                .param("cnt", count.to_string())
                .param("cluster", if cluster { "yes" } else { "no" })
                .build()
        };
        self.run_query(query)
    }

    /// Does the actual API call, parses the response and handles any errors.
    fn run_query<D>(&self, url: String) -> Result<(hyper::client::Response, D)>
        where D: serde::Deserialize
    {
        let req_result = ((*self.hub.client.borrow_mut()).borrow_mut())
            .request(hyper::method::Method::Get, &url)
            .send();

        match req_result {
            Err(err) => return Err(Error::HttpError(err)),
            Ok(mut res) => {
                if !res.status.is_success() {
                    let mut json_err = String::new();
                    res.read_to_string(&mut json_err).unwrap();
                    return match json::from_str::<ErrorResponse>(&json_err) {
                               Ok(serr) => Err(Error::BadRequest(serr)),
                               Err(_) => Err(Error::Failure(res)),
                           };
                }
                let mut json_resp = String::new();
                res.read_to_string(&mut json_resp).unwrap();
                return match json::from_str(&json_resp) {
                           Ok(decoded) => Ok((res, decoded)),
                           Err(err) => Err(Error::JsonDecodeError(json_resp, err)),
                       };
            }
        }
    }
}

/// Generic query builder that handles all URI-related stuff.
struct QueryBuilder<'a> {
    _api_ver: &'a str,
    _method: &'a str,
    _params: Vec<(&'a str, String)>,
}

impl<'a> QueryBuilder<'a> {
    fn new() -> Self {
        QueryBuilder {
            _api_ver: "2.5",
            _method: "",
            _params: Vec::with_capacity(2),
        }
    }

    fn method(mut self, method: &'a str) -> Self {
        self._method = method;
        self
    }

    fn param<S: Into<String>>(mut self, key: &'a str, val: S) -> Self {
        self._params.push((key, val.into()));
        self
    }

    fn build(self) -> String {
        let base = format!("http://api.openweathermap.org/data/{api}/{method}",
                           api = self._api_ver,
                           method = self._method);
        let mut ser = url::form_urlencoded::Serializer::new(String::new());

        match self._params.len() {
            0 => base,
            _ => {
                for p in self._params {
                    ser.append_pair(p.0, p.1.as_str());
                }
                base + "?" + ser.finish().as_str()
            }            
        }
    }
}

/// Contains the result of an aggregate query.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherAggregate {
    /// Search accuracy. Possible values: "accurate", "like".
    pub message: Option<String>,
    /// HTTP status code for the request
    pub cod: Option<i32>,
    /// Number of items in the list
    pub count: Option<i32>,
    /// List of weather info
    pub list: Option<Vec<WeatherInfo>>,
}

/// Contains the result of a bounding-box query.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherBoxAggregate {
    /// HTTP status code for the request
    pub cod: Option<i32>,
    /// Time elapsed server-side to handle the request
    pub calctime: Option<f32>,
    /// Number of items in the list
    pub cnt: Option<i32>,
    /// List of weather info
    pub list: Option<Vec<WeatherInfo>>,
}

/// Contains all the weather-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct WeatherInfo {
    /// City geographic coordinates
    pub coord: Option<Coordinates>,
    /// Weather conditions
    pub weather: Option<Vec<Weather>>,
    /// Internal parameter
    pub base: Option<String>,
    /// General weather parameters
    pub main: Option<Main>,
    /// Wind-related information
    pub wind: Option<Wind>,
    /// Cloud-related information
    pub clouds: Option<Clouds>,
    /// Rain-related information
    pub rain: Option<Rain>,
    /// Snow-related information
    pub snow: Option<Snow>,
    /// Time of data calculation, Unix, UTC
    pub dt: Option<i64>,
    /// Internal parameter
    pub sys: Option<Sys>,
    /// City ID
    pub id: Option<i64>,
    /// City name
    pub name: Option<String>,
    /// Internal parameter
    pub cod: Option<i32>,
}

/// Contains the geographic coordinates of the location.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Coordinates {
    /// Longitude
    pub lon: Option<f32>,
    /// Latitude
    pub lat: Option<f32>,
}

/// Represents OpenWeatherMap's weather condition codes.
/// See http://openweathermap.org/weather-conditions for details.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Weather {
    /// Weather condition ID
    pub id: Option<i32>,
    /// Group of weather parameters
    pub main: Option<String>,
    /// Weather condition within the group
    pub description: Option<String>,
    /// Weather icon ID
    pub icon: Option<String>,
}

/// Contains weather information not tied to particular weather conditions.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Main {
    /// Current temperature. Unit Default: [K], Metric: [°C], Imperial: [°F]
    pub temp: Option<f32>,
    /// Atmospheric pressure [hPa] (at sea level, if there is no sea_level or grnd_level data)
    pub pressure: Option<i32>,
    /// Humidity [%]
    pub humidity: Option<i32>,
    /// Minimum temperature at the moment (deviation from current temperature, significant for large areas)
    pub temp_min: Option<f32>,
    /// Maximum temperature at the moment (deviation from current temperature, significant for large areas)
    pub temp_max: Option<f32>,
    /// Atmospheric pressure on the sea level [hPa]
    pub sea_level: Option<i32>,
    /// Atmospheric pressure on the ground level [hPa]
    pub grnd_level: Option<i32>,
}

/// Contains wind-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Wind {
    /// Wind speed. Unit default: [m/s], Metric: [m/s], Imperial: [miles/h]
    pub speed: Option<f32>,
    /// Wind direction [deg] (meteorological)
    pub deg: Option<i32>,
    /// Wind gust. Same units as speed.AsMut
    pub gust: Option<f32>,
}

/// Contains cloud-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Clouds {
    /// Cloudiness [%]
    pub all: Option<i32>,
}

/// Contains rain-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Rain {
    /// Rain volume for the last 3 hours
    #[serde(rename="3h")]
    pub three_hours: Option<i32>,
}

/// Contains snow-related information.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Snow {
    /// Snow volume for the last 3 hours
    #[serde(rename="3h")]
    pub three_hours: Option<i32>,
}

/// Contains internal API parameters.
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Sys {
    /// Internal parameter
    #[serde(rename="type")]
    pub type_: Option<i32>,
    /// Internal parameter
    pub id: Option<i32>,
    /// Internal parameter
    pub message: Option<f32>,
    /// Country code
    pub country: Option<String>,
    /// Sunrise time, Unix, UTC
    pub sunrise: Option<i64>,
    /// Sunset time, Unix, UTC
    pub sunset: Option<i64>,
}

/// Represents an error message sent by the API server in response to a bad request.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// HTTP status code for the requested resource
    pub cod: Option<i32>,
    /// Human-readable error message
    pub message: Option<String>,
}

extern crate hyper;
extern crate owm;

use std::env;
use self::owm::{WeatherHub, BoundingBox, Units, FormatResponse};

#[test]
fn current_by_name() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let resp = hub.current().by_name("Pisa", Some("IT"));

    match resp {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok((_, info)) => {
            assert_eq!(Some(10.4),
                       info.coord
                           .clone()
                           .unwrap()
                           .lon);
            assert_eq!(Some(43.72),
                       info.coord
                           .clone()
                           .unwrap()
                           .lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_id() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let resp = hub.current().by_id(6542122); // Pisa

    match resp {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok((_, info)) => {
            assert_eq!(Some(10.41),
                       info.coord
                           .clone()
                           .unwrap()
                           .lon);
            assert_eq!(Some(43.71),
                       info.coord
                           .clone()
                           .unwrap()
                           .lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_coords() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let resp = hub.current().by_coords(43.71, 10.41); // Pisa

    match resp {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok((_, info)) => {
            assert_eq!(Some(10.41),
                       info.coord
                           .clone()
                           .unwrap()
                           .lon);
            assert_eq!(Some(43.71),
                       info.coord
                           .clone()
                           .unwrap()
                           .lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
#[ignore]
fn current_by_zip() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let resp = hub.current().by_zip_code(56124, Some("IT")); // Pisa

    match resp {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok((_, info)) => {
            assert_eq!(Some(10.41),
                       info.coord
                           .clone()
                           .unwrap()
                           .lon);
            assert_eq!(Some(43.71),
                       info.coord
                           .clone()
                           .unwrap()
                           .lat);
            assert_eq!(Some("Pisa".to_string()), info.name);
        }
    }
}

#[test]
fn current_by_bounds() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let resp = hub.current().by_bounds(&BoundingBox {
                                            top: 43.73,
                                            left: 10.38,
                                            bottom: 43.7,
                                            right: 10.42,
                                        },
                                       10,
                                       false);

    match resp {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok((_, info)) => {
            assert_eq!(Some(1), info.cnt);
            assert_eq!(Some("Pisa".to_string()), info.list.clone().unwrap()[0].name);
        }
    }
}

#[test]
fn current_by_circle() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let resp = hub.current().by_circle(43.71, 10.41, 10, false);

    match resp {
        Err(e) => {
            println!("{:#?}", e);
            assert!(false);
        }
        Ok((_, info)) => {
            assert_eq!(Some(10), info.count);
            assert_eq!(Some("Pisa".to_string()), info.list.clone().unwrap()[0].name);
        }
    }
}

#[test]
fn current_with_units() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let no_units = hub.current().by_id(6542122);
    let units = hub.current().units(Units::Metric).by_id(6542122);

    match (no_units, units) {
        (_, Err(e)) | (Err(e), _) => {
            println!("{:#?}", e);
            assert!(false);
        }
        (Ok((_, i1)), Ok((_, i2))) => {
            assert_eq!(i1.name, i2.name);
            assert!(i1.main.unwrap().temp != i2.main.unwrap().temp);
        }
    }
}

#[test]
fn current_with_language() {
    let hub = WeatherHub::new(hyper::Client::new(), &env::var("OWM_API_KEY").unwrap());
    let no_lang = hub.current().by_id(6542122);
    let lang = hub.current().lang("IT").by_id(6542122);

    match (no_lang, lang) {
        (_, Err(e)) | (Err(e), _) => {
            println!("{:#?}", e);
            assert!(false);
        }
        (Ok((_, i1)), Ok((_, i2))) => {
            assert_eq!(i1.name, i2.name);
            assert!(i1.weather.unwrap()[0].description != i2.weather.unwrap()[0].description);
        }
    }
}
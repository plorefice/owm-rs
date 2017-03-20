extern crate hyper;
extern crate owm;

#[cfg(test)]
mod tests {
    use std::env;
    use owm::{WeatherHub, BoundingBox};
    use hyper;

    #[test]
    fn current_by_name() {
        let hub = WeatherHub::new(hyper::Client::new(), env::var("OWM_API_KEY").unwrap());
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
        let hub = WeatherHub::new(hyper::Client::new(), env::var("OWM_API_KEY").unwrap());
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
        let hub = WeatherHub::new(hyper::Client::new(), env::var("OWM_API_KEY").unwrap());
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
        let hub = WeatherHub::new(hyper::Client::new(), env::var("OWM_API_KEY").unwrap());
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
        let hub = WeatherHub::new(hyper::Client::new(), env::var("OWM_API_KEY").unwrap());
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
        let hub = WeatherHub::new(hyper::Client::new(), env::var("OWM_API_KEY").unwrap());
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
}

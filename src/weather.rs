use std::string::String;
use std::time::{Duration, Instant};
use std::error::Error;
use weathererror::*;

// https://home.openweathermap.org
// https://api.openweathermap.org/data/2.5/weather?q={CITY_ID}&appid={API_KEY}

#[derive(Debug, PartialEq, Clone)]
pub struct Weather {
    description: Option<String>,
    degrees: Option<i8>,
    url: String,
    five_min: Duration,
    last_update: Option<Instant>,
}

impl Weather {
    pub fn init(url: String) -> Weather {
        Weather {
            description: None,
            degrees: None,
            url,
            five_min: Duration::from_secs(300),
            last_update: None,
        }
    }

    pub fn update(&mut self) {
        if let Some(t) = self.last_update {
            if t.elapsed() >= self.five_min {
                Weather::update_vals(self);
            }
        } else {
            Weather::update_vals(self);
        }
    }

    fn update_vals(&mut self) {
        match get_weather(&self.url) {
            Ok(t) => {
                self.description = Some(t.0);
                self.degrees = Some(t.1);
            },
            Err(e) => {
                self.description = None;
                self.degrees = None;
                eprintln!("Error: {}", e)
            },
        }
        self.last_update = Some(Instant::now());
    }

    pub fn output(&self) -> Option<String> {
        match (&self.description, &self.degrees) {
            (Some(descript), Some(degree)) => Some(format!("\u{e01d}{} {}°C", descript, degree)),
            _ => None,
        }
    }
}

fn get_weather(url: &str) -> Result<(String, i8), Box<dyn Error>> {
    /* JSON FORMAT
    {
        "base":"stations",
        "clouds":{"all":75},
        "cod":200,
        "coord":{"lat":59.27,"lon":15.21},
        "dt":1549862400,
        "id":2686657,
        "main":{"humidity":96,"pressure":992,"temp":274.15,"temp_max":274.15,"temp_min":274.15},
        "name":"Orebro",
        "sys":{"country":"SE","id":1777,"message":0.0036,"sunrise":1549867523,"sunset":1549899741,"type":1},
        "visibility":6000,
        "weather":[{"description":"mist","icon":"50n","id":701,"main":"Mist"}],
        "wind":{"deg":320,"speed":1.5}
    }
    */

    let json: serde_json::Value = reqwest::get(url)?.json()?;

    let degrees_cel = json
        .pointer("/main/temp")
        .ok_or_else(|| WeatherError::NoTempVal)?
        .as_f64()
        .ok_or_else(|| WeatherError::F64Error)?
        .round() as i8;

    let description = json.pointer("/weather/0/description")
        .ok_or_else(|| WeatherError::NoDescriptionVal)?
        .as_str()
        .ok_or_else(|| WeatherError::StrError)?
        .trim_matches('"')
        .capitalize_words();

    Ok((description, degrees_cel))
}

trait Capitalize {
    fn capitalize_words(&self) -> String;
}

impl Capitalize for str {
    fn capitalize_words(&self) -> String {
        let mut new = String::new();
        let words: Vec<_> = self.split_whitespace().collect();
        let len = words.len() - 1;

        for (num, word) in words.iter().enumerate() {
            let mut chars = word.chars();
            if let Some(c) = chars.next() {
                new.push_str(&(c.to_uppercase().collect::<String>() + chars.as_str()));
                if num != len {
                    new.push_str(" ");
                }
            }
        }
        new
    }
}

mod weathererror {
    use std::fmt;

    #[derive(Debug, PartialEq, Clone)]
    pub enum WeatherError {
        NoTempVal,
        F64Error,
        NoDescriptionVal,
        StrError,
    }

    impl std::error::Error for WeatherError {
        fn description(&self) -> &str {
            match *self {
                WeatherError::NoTempVal => "could not find `/main/temp` in json",
                WeatherError::F64Error => "could not cast `/main/temp` value into f64",
                WeatherError::NoDescriptionVal => "could not find /weather/0/description in json",
                WeatherError::StrError => "could not cast /weather/0/description value into str",
            }
        }
    }

    impl fmt::Display for WeatherError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                WeatherError::NoTempVal => f.write_str("could not find `/main/temp` in json"),
                WeatherError::F64Error => f.write_str("could not cast `/main/temp` value into f64"),
                WeatherError::NoDescriptionVal => f.write_str("could not find /weather/0/description in json"),
                WeatherError::StrError => f.write_str("could not cast /weather/0/description value into str"),
            }
        }
    }


}

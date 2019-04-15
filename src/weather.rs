use std::string::String;
use std::time::{Duration, Instant};

// https://home.openweathermap.org
// https://api.openweathermap.org/data/2.5/weather?q={CITY_ID}&appid={API_KEY}

#[derive(Debug)]
pub struct Weather {
    val: String,
    url: String,
    five_min: Duration,
    last_update: Option<Instant>,
}

impl Weather {
    pub fn init(url: String) -> Weather {
        Weather {
            url,
            val: String::new(),
            five_min: Duration::from_secs(300),
            last_update: None,
        }
    }

    pub fn update(&mut self) {
        if let Some(lupdate) = self.last_update {
            if lupdate.elapsed() >= self.five_min {
                self.val = get_weather(&self.url).unwrap_or_default();
                self.last_update = Some(Instant::now());
            }
        } else {
            self.val = get_weather(&self.url).unwrap_or_default();
            self.last_update = Some(Instant::now());
        }
    }

    pub fn output(&self) -> String {
        self.val.to_string()
    }
}

fn get_weather(url: &str) -> Option<String> {
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

    let mut req = match reqwest::get(url) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };

    let json: serde_json::Value = match req.json() {
        Ok(j) => j,
        Err(e) => {
            eprintln!("{}", e);
            return None;
        }
    };

    let degrees_cel: Option<f64> = json
        .pointer("/main/temp")
        .and_then(|n| n.as_f64().and_then(|f| Some(f.round())));

    let weather: Option<String> = json.pointer("/weather/0/description").and_then(|s| {
        s.as_str().and_then(|ss| {
            let mut x = ss.trim_matches('"').chars();
            x.next()
                .and_then(|f| Some(f.to_uppercase().collect::<String>() + x.as_str()))
        })
    });

    let mut weather_str = String::new();
    if let (Some(x), Some(y)) = (weather, degrees_cel) {
        weather_str.push_str(&format!("\u{e01d}{} {}°C", x, y));
    }

    Some(weather_str)
}

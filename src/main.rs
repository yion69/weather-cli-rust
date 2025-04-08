use std::io::{stdin, stdout, Write};
use dotenv::dotenv;
use serde::Deserialize;
use colored::{ColoredString, Colorize};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    coord: Cords,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i64,
    wind: Wind,
    clouds: Clouds,
    dt: i128,
    sys: Sys,
    timezone: i32,
    name: String,
    cod: i32  
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Cords {
    lon: f64,
    lat: f64
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Weather {
    id: i32, 
    main: String,
    description: String,
    icon: String
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32,
    sea_level: i32,
    grnd_level: i32
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: i32
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Clouds {
    all: i32
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Sys {
    #[serde(rename = "type")]
    type_field: Option<i32>,
    id: Option<i64>,
    country: String,
    sunrise: i64,
    sunset: i64
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct WeatherMain {
    city: String,
    country: String,
    weather_status: String,
    weather_description: String,
    wind_speed: f32,
    temp: f32,
    temp_min: f32,
    temp_max: f32,
    humidity: i32
}

async fn fetch_weather(
    city_name: &str,
    country_name: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
   
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&APPID={}&units=metric",
        city_name, country_name, api_key
    );
 
    let response = reqwest::get(url)
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(response)
}

fn print_map() {
    let ascii_text = r#"
___________________________________________________
.   *   .   .  .       .   *   .     .    .   *   . 
.   .  .   .  * ┓ ┏     ┓       ┏┓┓ ┳    .  .
.    .    *  .  ┃┃┃┏┓┏┓╋┣┓┏┓┏┓  ┃ ┃ ┃  *  .    .  * 
*  .    .    *  ┗┻┛┗ ┗┻┗┛┗┗ ┛   ┗┛┗┛┻  .  .  *    .                      
. *      .   .    .  .     .  * .  . .  *  .    . *  
___________________________________________________          
    "#;
    println!("{:30}",ascii_text.white().bold());
}

fn get_emoji (weather: &str) -> &str {

    let emoji;
    
    match weather {
        "clear sky" => emoji = "☀️",
        "few clouds" | "scattered clouds" | "broken clouds" => emoji = "🌤️",
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" => emoji = "☁️",
        "shower rain" | "rain" => emoji = "🌧️",
        "thunderstorm" => emoji = "⛈️",
        "snow" => emoji = "🌨️",
        _ => emoji = "⚠️error"
    }

    return emoji
}

fn print_response (res: &WeatherResponse) {

    let city: &String = &res.name;
    let country: &String = &res.sys.country;
    let weather_status: &String = &res.weather[0].main;
    let weather_description: &String = &res.weather[0].description;
    let wind_speed: &f32 = &res.wind.speed;
    let temperature: &f32 = &res.main.temp;
    let temp_min: &f32 = &res.main.temp_min;
    let temp_max: &f32 = &res.main.temp_max;
    let humidity: &i32 = &res.main.humidity;
    let weather_emoji = get_emoji(&weather_description);

    let formatted_text: ColoredString = format!(
        "Weather in {},{} | {} ({}) 
            >Temperature: {}°C
                > Temperature-Min: {}°C
                > Temperature-Max: {}°C
            >Wind: {} km/h
            >Humidity: {} % 
___________________________________________________
        ", city, country, weather_status, weather_emoji, temperature, temp_min, temp_max, wind_speed, humidity
    ).white().bold();

    println!("{:30}",formatted_text);
}

fn print_text (text:&str) {

    let formatted_text: ColoredString = format!("{}",text).white().bold();

    println!("{:30}",formatted_text);
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let mut user_response = String::new();
    let mut city = String::new();          
    let mut country = String::new();        
      
    let api_key = std::env::var("API_KEY").expect("key not found");
    
    let text_user_request = "Check weather in your city? <yes|no>";
    let text_user_city = "Enter your city name | e.g. <Yangon, Bangkok>";
    let text_user_country = "Enter your country code | e.g. <MM, TH>";
    
    print_map();
    
    loop {
        print_text(text_user_request);
        let _ = stdout().flush();
        stdin().read_line(&mut user_response).unwrap();
        
        user_response = user_response.trim().to_lowercase();
        
        if user_response == "no"{ break };

        print_text(text_user_city);
        stdin().read_line(&mut city).unwrap();

        let city_name:Vec<&str> = city.trim().split_whitespace().collect();   
        if city_name.len() > 1 { 
            city = city_name.join("+");
        } else {
            city = city_name[0].to_string();
        }

        print_text(text_user_country);
        stdin().read_line(&mut country).unwrap();

        country = country.trim().to_lowercase();       
        
        let result = fetch_weather(&city, &country, &api_key).await;
        match result {
            Ok(v) => print_response(&v),
            Err(e) => eprintln!("{}", e)
        };

        user_response.clear();
        city.clear();
        country.clear();
    }

}
extern crate chrono;

use chrono::Local;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
enum StatusBarError {
    IoError(std::io::Error),
    TypeConvertError(std::num::ParseFloatError),
}
impl From<std::io::Error> for StatusBarError {
    fn from(error: std::io::Error) -> Self {
        StatusBarError::IoError(error)
    }
}
impl From<std::num::ParseFloatError> for StatusBarError {
    fn from(error: std::num::ParseFloatError) -> Self {
        StatusBarError::TypeConvertError(error)
    }
}

fn read_file_content(path: &str) -> Result<String, StatusBarError> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    contents.truncate(contents.len() - 1);
    Ok(contents)
}

fn read_file_to_num(path: &str) -> Result<f64, StatusBarError> {
    let contents = read_file_content(path)?;
    let num_cont = contents.parse::<f64>()?;
    Ok(num_cont)
}

fn get_bat_percent() -> Result<f64, StatusBarError> {
    let bat_max = read_file_to_num("/sys/class/power_supply/BAT0/energy_full")
        .or_else(|_| read_file_to_num("/sys/class/power_supply/BAT0/charge_full"))?;

    let bat_now = read_file_to_num("/sys/class/power_supply/BAT0/energy_now")
        .or_else(|_| read_file_to_num("/sys/class/power_supply/BAT0/charge_now"))?;
    Ok((bat_now / bat_max) * 100.0)
}

fn get_bat_status() -> Result<String, StatusBarError> {
    let cont = read_file_content("/sys/class/power_supply/BAT0/status")?;
    match cont.as_ref() {
        "Discharging" => Ok("BAT".to_string()),
        "Charging" => Ok("CHR".to_string()),
        _ => Ok("PWR".to_string()),
    }
}

fn get_cpu_temperature() -> Result<f64, StatusBarError> {
    let raw = read_file_to_num("/sys/class/hwmon/hwmon2/temp5_input")?;
    Ok(raw / 1000.0)
}

fn fmt_cpu() -> Option<String> {
    let temp = match get_cpu_temperature() {
        Ok(t) => t,
        Err(_) => -1.0,
    };
    if temp < 0.0 {
        None
    } else {
        Some(format!("CPU: {:.*}C", 0, temp))
    }
}

fn fmt_bat() -> Option<String> {
    let bat_status = match get_bat_status() {
        Ok(s) => s,
        Err(_) => "NA".to_string(),
    };
    let bat_percent = match get_bat_percent() {
        Ok(p) => p,
        Err(_) => -1.0,
    };
    Some(format!("{}: {:.*}%", bat_status, 2, bat_percent))
}

fn fmt_time() -> Option<String> {
    Some(Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string())
}

fn fmt_bar() -> String {
    let functions: Vec<fn() -> Option<String>> = vec![fmt_cpu, fmt_bat, fmt_time];
    let mut formatted: Vec<String> = vec![];
    for output in functions {
        match output() {
            Some(val) => formatted.push(val),
            None => continue,
        };
    }
    formatted.join(" | ")
}

fn main() {
    println!(" {} ", fmt_bar());
}

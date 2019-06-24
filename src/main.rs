extern crate chrono;

use chrono::Local;
use std::fs::File;
use std::io::prelude::*;

fn read_file_content(path: &str) -> String {
    let mut contents = String::new();
    let mut file = File::open(path).expect("Unable to open the file");
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    let len = contents.len();
    contents.truncate(len - 1);
    contents
}

fn read_file_to_num(path: &str) -> f64 {
    let contents = read_file_content(path);
    let num_cont = contents.parse::<f64>().unwrap();
    num_cont
}

fn get_bat_percent() -> f64 {
    let bat_max = read_file_to_num("/sys/class/power_supply/BAT0/energy_full");
    let bat_now = read_file_to_num("/sys/class/power_supply/BAT0/energy_now");
    (bat_now / bat_max) * 100.0
}

fn get_bat_status() -> String {
    let cont = read_file_content("/sys/class/power_supply/BAT0/status");
    match cont.as_ref() {
        "Discharging" => "BAT".to_string(),
        "Charging" => "CHR".to_string(),
        _ => "PWR".to_string(),
    }
}

fn get_cpu_temperature() -> f64 {
    read_file_to_num("/sys/class/hwmon/hwmon2/temp5_input") / 1000.0
}

fn main() {
    println!(
        " {} | {}: {} | {}",
        format!("CPU: {:.*}C", 0, get_cpu_temperature(),),
        get_bat_status(),
        format!("{:.*}%", 2, get_bat_percent()),
        Local::now().format("%Y-%m-%d %H:%M:%S %z")
    );
}

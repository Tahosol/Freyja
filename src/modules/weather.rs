use regex::Regex;
use std::process::Command;
pub fn get() -> String {
    let curl = Command::new("curl")
        .arg("https://wttr.in/?format=4")
        .output();
    if let Some(output) = curl.ok() {
        let re = Regex::new(r"\d+(\.\d+)?").unwrap(); // Regex to match integers and decimals
        let string_output = String::from_utf8_lossy(&output.stdout).to_string();
        let slipt_from_cloud_icon: Vec<&str> = string_output.split("🌬️").collect();
        let wind_speed = slipt_from_cloud_icon.last().unwrap_or(&"NA").trim();
        let other_half = slipt_from_cloud_icon[0];
        let location_temp: Vec<String> = other_half.split(':').map(|e| e.to_string()).collect();
        let temps = location_temp
            .last()
            .cloned()
            .unwrap_or_else(|| "NA".to_string());
        let temp: Vec<&str> = re.find_iter(&temps).map(|mat| mat.as_str()).collect();
        format!(
            "According to my calculations, you can expect a temperature of {} °C and a wind speed of {}. This information is based on {} which represents your estimated location.",
            temp[0], wind_speed, location_temp[0]
        )
    } else {
        "I can't seem to find the weather now.".to_string()
    }
}

// use bk_tree::{BKTree, metrics};

pub fn detect_weather_ask(que: &str) -> bool {
    if que.contains("weather") {
        return true;
    } else {
        return false;
    }
    // let tree = geta();
    // let find = tree.find(que, 5).collect::<Vec<_>>();
    // if find.is_empty() { false } else { true }
}

// use std::fs;
// fn geta() -> BKTree<String> {
//     let mut tree: BKTree<String> = BKTree::new(metrics::Levenshtein);
//     match fs::read_to_string("default/weather.txt") {
//         Ok(list) => {
//             for line in list.lines() {
//                 tree.add(line.to_string());
//             }
//         }
//         Err(e) => {
//             println!("Can't find default {e}");
//         }
//     }
//     tree
// }

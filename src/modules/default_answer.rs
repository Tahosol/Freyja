use std::fs;

#[allow(dead_code)]
pub fn get() -> Vec<String> {
    match fs::read_to_string("default/default.txt") {
        Ok(list) => {
            let vec = list.lines().map(|e| e.to_string()).collect();
            vec
        }
        Err(e) => {
            println!("Can't find default {e}");
            Vec::new()
        }
    }
}

pub fn check(que: &str) -> (bool, String) {
    if (que.contains("cava") && que.contains("open"))
        || (que.contains("run") && que.contains("cava"))
        || (que.contains("open")
            && (que.contains("cover") || que.contains("carver") || que.contains("call vote")))
    {
        (true, command_cava())
    } else if (que.contains("open")
        || que.contains("run")
        || que.contains("play")
        || que.contains("plays"))
        && (que.contains("music") || que.contains("song") || que.contains("songs"))
    {
        (true, command_elisa())
    } else {
        (false, String::new())
    }
}

use std::process::Command;

fn command_cava() -> String {
    let _ = Command::new("kitty").arg("--hold").arg("cava").spawn();
    "Okay, I will run cava. Enjoy your music!".to_string()
}

use rand::random_range;
use std::fs;

fn command_elisa() -> String {
    let music_dir = dirs::audio_dir().unwrap();

    match fs::read_dir(music_dir) {
        Ok(read_entry) => {
            let entries: Vec<_> = read_entry.map(|e| e.ok()).collect();
            let random_choice = random_range(0..entries.len());
            let folder_entry = &entries[random_choice];
            if let Some(e) = folder_entry {
                let path = e.path();
                if path.is_file() {
                    let _ = Command::new("setsid").arg("elisa").arg(path).spawn();
                    return "Sure, I will play a random song now".to_string();
                } else {
                    let read = fs::read_dir(path);
                    if let Some(e) = read.ok() {
                        let entries: Vec<_> = e.map(|e| e.ok()).collect();
                        let random_choice = random_range(0..entries.len());
                        let folder_entry = &entries[random_choice];
                        if let Some(e) = folder_entry {
                            let _ = Command::new("setsid").arg("elisa").arg(e.path()).spawn();
                            return "Sure, I will play a random song now".to_string();
                        } else {
                            return "Sorry, I fail to open the second entry file".to_string();
                        }
                    } else {
                        return "Sorry, I fail to open the second entry folder".to_string();
                    }
                }
            } else {
                return "Sorry, I fail to read the folder_entry".to_string();
            }
        }
        Err(_) => "Sorry, I fail to read the music directory.".to_string(),
    }
}

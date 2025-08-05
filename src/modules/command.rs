pub fn check(que: &str) -> (bool, String) {
    if (que.contains("open")
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

use std::path::{Path, PathBuf};
use std::{error::Error, process::Command};

use rand;
use rand::seq::SliceRandom;
use std::fs;
fn command_elisa() -> String {
    let music_dir = dirs::audio_dir().unwrap();
    let mut folders: Vec<PathBuf> = Vec::new();
    let mut music_files: Vec<PathBuf> = Vec::new();

    let mut first_read = find_music(vec![music_dir]);
    if !first_read.0.is_empty() {
        music_files.append(&mut first_read.0);
    }
    if !first_read.1.is_empty() {
        folders = first_read.1;
    }
    loop {
        let mut read = find_music(folders.clone());
        if !read.0.is_empty() {
            music_files.append(&mut read.0);
        }
        if !read.1.is_empty() {
            folders = read.1;
        } else {
            break;
        }
    }
    let mut random = rand::rng();
    music_files.shuffle(&mut random);
    let mut command = Command::new("setsid");
    command.arg("elisa");

    for song in music_files {
        command.arg(song);
    }
    let _ = command.spawn();
    return "Sure, I will play some random songs now".to_string();
}

use std::fs::DirEntry;

fn read_dir_return_entries(dir: &Path) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let vec_of_direntry: Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
    Ok(vec_of_direntry)
}

fn detect_music(dir: &Path) -> bool {
    let match_ext = vec![
        "mp3", "flac", "wav", "ogg", "m4a", "aac", "wma", "opus", "aiff", "alac",
    ];
    if match_ext.contains(&dir.extension().and_then(|dir| dir.to_str()).unwrap()) {
        true
    } else {
        false
    }
}

fn find_music(dir: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut musics = Vec::new();
    let mut paths = Vec::new();
    for es in dir {
        match read_dir_return_entries(&es) {
            Ok(e) => {
                for s in e {
                    if s.path().is_dir() {
                        paths.push(s.path());
                    } else if detect_music(&s.path()) {
                        musics.push(s.path());
                    }
                }
            }
            Err(_) => {}
        }
    }
    (musics, paths)
}

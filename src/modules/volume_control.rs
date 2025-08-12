use std::{error::Error, process::Command};

pub fn pause() -> Result<(), Box<dyn Error>> {
    Command::new("playerctl")
        .args(&["pause", "--all-players"])
        .output()?;
    Ok(())
}
pub fn play() -> Result<(), Box<dyn Error>> {
    Command::new("playerctl").arg("play").output()?;
    Ok(())
}

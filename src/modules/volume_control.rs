use std::{error::Error, process::Command};

pub fn mute() -> Result<(), Box<dyn Error>> {
    Command::new("wpctl")
        .args(&["set-mute", "@DEFAULT_AUDIO_SINK@", "1"])
        .output()?;
    Ok(())
}

pub fn unmute() -> Result<(), Box<dyn Error>> {
    Command::new("wpctl")
        .args(&["set-mute", "@DEFAULT_AUDIO_SINK@", "0"])
        .output()?;
    Ok(())
}

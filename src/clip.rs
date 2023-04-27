use anyhow::{anyhow, Result};
use std::env;
use std::env::consts::OS;
use std::process::Command;

pub struct Clip {
    pub text: Option<String>,
    pub is_wayland: Option<bool>,
    pub is_x11: Option<bool>,
    pub is_windows: Option<bool>,
    pub is_mac: Option<bool>,
}

impl Clip {
    pub fn new() -> Result<Self> {
        // check os type
        let mut clip = Self {
            text: None,
            is_wayland: None,
            is_x11: None,
            is_windows: None,
            is_mac: None,
        };

        match OS {
            "linux" => {
                let wayland = env::var("WAYLAND_DISPLAY").is_ok();
                let x11 = env::var("DISPLAY").is_ok();

                if wayland {
                    clip.is_wayland = Some(true);
                } else if x11 {
                    clip.is_x11 = Some(true);
                }
            }
            "windows" => {
                clip.is_windows = Some(true);
            }
            "macos" => {
                clip.is_mac = Some(true);
            }
            _ => {
                return Err(anyhow!("Unsupported OS"));
            }
        }

        Ok(clip)
    }

    pub fn copy(&mut self, text: &str) -> Result<()> {
        match self {
            Clip {
                is_wayland: Some(true),
                ..
            } => {
                self.copy_wayland(text)?;
            }
            Clip {
                is_x11: Some(true), ..
            } => {
                self.copy_x11(text)?;
            }
            Clip {
                is_windows: Some(true),
                ..
            } => {
                self.copy_windows(text)?;
            }
            Clip {
                is_mac: Some(true), ..
            } => {
                self.copy_macos(text)?;
            }
            _ => {
                return Err(anyhow!("Unsupported OS"));
            }
        }

        Ok(())
    }

    pub fn copy_x11(&mut self, text: &str) -> Result<()> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("echo -n {} | xclip -selection clipboard", text))
            .spawn()
            .expect("failed to execute process");
        child.wait().expect("failed to wait on child");

        Ok(())
    }

    pub fn copy_wayland(&mut self, text: &str) -> Result<()> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("wl-copy {}", text))
            .spawn()
            .expect("failed to execute process");

        child.wait().expect("failed to wait on child");

        Ok(())
    }

    pub fn copy_windows(&mut self, text: &str) -> Result<()> {
        let mut child = Command::new("powershell.exe")
            .arg("-c")
            .arg(format!("Set-Clipboard -Value {}", text))
            .spawn()
            .expect("failed to execute process");

        child.wait().expect("failed to wait on child");

        Ok(())
    }

    pub fn copy_macos(&mut self, text: &str) -> Result<()> {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "echo -n {} | osascript -e 'set the clipboard to (read stdin)'",
                text
            ))
            .spawn()
            .expect("failed to execute process");

        child.wait().expect("failed to wait on child");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_new() -> Result<()> {
        let clip = Clip::new()?;
        assert!(
            clip.is_wayland.is_some()
                || clip.is_x11.is_some()
                || clip.is_windows.is_some()
                || clip.is_mac.is_some()
        );

        Ok(())
    }

    #[test]
    fn test_clip_copy() -> Result<()> {
        let mut clip = Clip::new()?;
        clip.copy("test").unwrap();

        Ok(())
    }

    #[test]
    fn test_clip_unsupported() -> Result<()> {
        let mut clip = Clip {
            text: None,
            is_wayland: None,
            is_x11: None,
            is_windows: None,
            is_mac: None,
        };

        assert!(clip.copy("test").is_err());

        Ok(())
    }
}

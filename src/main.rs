use std::{collections::HashMap, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let cwd = std::env::current_dir()?;
    let mut configs = HashMap::new();
    configs.insert(
        "/Users/snipextt/flobiz",
        "/Users/snipextt/.ssh/flobiz-config",
    );
    configs.insert(
        "/Users/snipextt/code",
        "/Users/snipextt/.ssh/personal-config",
    );
    const CONFIG_FILE: &str = "/Users/snipextt/.ssh/config";
    match cwd.canonicalize()?.to_str() {
        Some(path) => {
            for (key, value) in configs {
                if path.starts_with(key) {
                    fs::copy(value, CONFIG_FILE)?;
                }
            }
        }
        None => println!("Current directory: {:?}", cwd),
    }
    Ok(())
}

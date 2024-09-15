use std::env;
use std::fs::{self, File};
use std::io::Result;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local, Timelike};
use std::time::SystemTime;

fn create_afterhours_directory() -> Result<PathBuf> {
    let afterhours_dir = env::current_dir()?.join("afterhours");
    if !afterhours_dir.exists() {
        fs::create_dir(&afterhours_dir)?;
    }
    Ok(afterhours_dir)
}

fn is_afterhours(file_time: SystemTime) -> bool {
    let datetime: DateTime<Local> = DateTime::from(file_time);
    let hour = datetime.hour();

    // Check if the file was modified between 8 PM and midnight or midnight and 9 AM
    (hour >= 20 && hour < 24) || (hour >= 0 && hour < 9)
}

fn move_file_to_afterhours(file_path: &Path, afterhours_dir: &Path) -> Result<()> {
    let file_name = file_path.file_name().unwrap();
    let dest_path = afterhours_dir.join(file_name);
    fs::rename(file_path, dest_path)?;
    println!("Moved file: {:?}", file_name);
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    let afterhours_dir = create_afterhours_directory()?;

    for entry in fs::read_dir(&directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let modified_time = metadata.modified()?;

            if is_afterhours(modified_time) {
                move_file_to_afterhours(&path, &afterhours_dir)?;
            }
        }
    }

    Ok(())
}


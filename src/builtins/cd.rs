use std::env;
use std::path::{Path, PathBuf};

pub fn cd(current_dir: &mut PathBuf, args: &[String]) -> Result<(), String> {
    let target = if args.is_empty() {
        env::var("HOME").unwrap_or_else(|_| current_dir.to_string_lossy().to_string())
    } else if args.len() == 1 {
        args[0].clone()
    } else {
        return Err("cd: too many arguments".to_string());
    };

    let next_path = if Path::new(&target).is_absolute() {
        PathBuf::from(&target)
    } else {
        current_dir.join(&target)
    };

    if !next_path.exists() {
        return Err(format!("cd: {}: No such file or directory", target));
    }

    if !next_path.is_dir() {
        return Err(format!("cd: {}: Not a directory", target));
    }

    env::set_current_dir(&next_path).map_err(|err| format!("cd: {}: {}", target, err))?;

    *current_dir =
        env::current_dir().map_err(|err| format!("Failed to get current directory: {}", err))?;

    Ok(())
}

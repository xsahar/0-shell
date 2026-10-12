use std::fs;
use std::path::Path;

pub fn rm(args: &[String]) -> Result<(), String> {
    let mut recursive = false;
    let mut paths = Vec::new();

    for arg in args {
        if arg == "-r" || arg == "-R" {
            recursive = true;
        } else {
            paths.push(arg.clone());
        }
    }

    if paths.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    for path_arg in paths {
        let path = Path::new(&path_arg);

        if path.is_dir() && !recursive {
            return Err(format!(
                "rm: cannot remove directory '{}' without -r",
                path_arg
            ));
        }

        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|err| format!("rm: {}: {}", path_arg, err))?;
        } else {
            fs::remove_file(path).map_err(|err| format!("rm: {}: {}", path_arg, err))?;
        }
    }

    Ok(())
}

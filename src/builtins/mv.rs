use std::fs;
use std::path::Path;

pub fn mv(args: &[String]) -> Result<(), String> {
    if args.len() != 2 {
        return Err("mv: expected source and destination".to_string());
    }

    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);

    if dst.exists() && dst.is_dir() {
        let file_name = src
            .file_name()
            .ok_or_else(|| format!("mv: {}: cannot move directory into target", args[0]))?;
        let final_dst = dst.join(file_name);
        fs::rename(src, &final_dst)
            .map_err(|err| format!("mv: {} -> {}: {}", args[0], args[1], err))?;
    } else {
        fs::rename(src, dst).map_err(|err| format!("mv: {} -> {}: {}", args[0], args[1], err))?;
    }

    Ok(())
}

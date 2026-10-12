use std::fs;
use std::path::Path;

pub fn cp(args: &[String]) -> Result<(), String> {
    if args.len() != 2 {
        return Err("cp: expected source and destination".to_string());
    }

    let src = Path::new(&args[0]);
    let dst = Path::new(&args[1]);

    if src.is_dir() {
        copy_dir_all(src, dst)
    } else {
        if dst.exists() && dst.is_dir() {
            let file_name = src
                .file_name()
                .ok_or_else(|| format!("cp: {}: cannot copy directory to file", args[0]))?;
            let final_dst = dst.join(file_name);
            fs::copy(src, &final_dst)
                .map_err(|err| format!("cp: {} -> {}: {}", args[0], args[1], err))?;
            Ok(())
        } else {
            fs::copy(src, dst).map_err(|err| format!("cp: {} -> {}: {}", args[0], args[1], err))?;
            Ok(())
        }
    }
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    if !dst.exists() {
        fs::create_dir_all(dst).map_err(|err| format!("cp: {}: {}", dst.display(), err))?;
    }

    for entry in fs::read_dir(src).map_err(|err| format!("cp: {}: {}", src.display(), err))? {
        let entry = entry.map_err(|err| format!("cp: {}: {}", src.display(), err))?;
        let source_path = entry.path();
        let target_path = dst.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_all(&source_path, &target_path)?;
        } else {
            fs::copy(&source_path, &target_path).map_err(|err| {
                format!(
                    "cp: {} -> {}: {}",
                    source_path.display(),
                    target_path.display(),
                    err
                )
            })?;
        }
    }

    Ok(())
}

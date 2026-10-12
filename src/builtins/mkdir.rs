use std::fs;
use std::path::Path;

pub fn mkdir(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err("mkdir: missing operand".to_string());
    }

    for path_arg in args {
        let path = Path::new(path_arg);
        fs::create_dir_all(path).map_err(|err| format!("mkdir: {}: {}", path_arg, err))?;
    }

    Ok(())
}

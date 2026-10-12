use std::fs;
use std::path::Path;

pub fn cat(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Ok(());
    }

    for path_arg in args {
        let path = Path::new(path_arg);
        let contents =
            fs::read_to_string(path).map_err(|err| format!("cat: {}: {}", path_arg, err))?;
        print!("{}", contents);
    }

    Ok(())
}

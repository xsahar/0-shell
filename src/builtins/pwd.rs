use std::path::Path;

pub fn pwd(current_dir: &Path) -> Result<(), String> {
    println!("{}", current_dir.display());
    Ok(())
}

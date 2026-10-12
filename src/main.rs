mod builtins;
mod parser;

use std::env;
use std::io::{self, Write};
use std::path::Path;

fn print_prompt(current_dir: &Path) {
    print!("{} $ ", current_dir.display());
    let _ = io::stdout().flush();
}

fn run() -> Result<(), String> {
    let mut current_dir =
        env::current_dir().map_err(|err| format!("Failed to get current directory: {}", err))?;

    loop {
        print_prompt(&current_dir);

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(err) => return Err(format!("Failed to read input: {}", err)),
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        let args = parser::parse_command(trimmed);
        if args.is_empty() {
            continue;
        }

        match args[0].as_str() {
            "exit" => {
                if args.len() > 1 {
                    if let Ok(code) = args[1].parse::<i32>() {
                        std::process::exit(code);
                    }
                }
                return Ok(());
            }
            "echo" => {
                if let Err(err) = builtins::echo::echo(&args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "cd" => {
                if let Err(err) = builtins::cd::cd(&mut current_dir, &args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "pwd" => {
                if let Err(err) = builtins::pwd::pwd(&current_dir) {
                    eprintln!("{}", err);
                }
            }
            "ls" => {
                if let Err(err) = builtins::ls::ls(&current_dir, &args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "cat" => {
                if let Err(err) = builtins::cat::cat(&args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "cp" => {
                if let Err(err) = builtins::cp::cp(&args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "rm" => {
                if let Err(err) = builtins::rm::rm(&args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "mv" => {
                if let Err(err) = builtins::mv::mv(&args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "mkdir" => {
                if let Err(err) = builtins::mkdir::mkdir(&args[1..]) {
                    eprintln!("{}", err);
                }
            }
            "help" => {
                println!("Available builtins: cd, echo, pwd, ls, cat, cp, rm, mv, mkdir, exit");
            }
            _ => println!("Command '{}' not found", args[0]),
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod builtin_tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_root(label: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "zero-shell-{label}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create isolated test directory");
        path
    }

    fn arg(path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }

    #[test]
    fn mkdir_creates_multiple_nested_directories() {
        let root = test_root("mkdir");
        let first = root.join("parent/first");
        let second = root.join("parent/second");

        crate::builtins::mkdir::mkdir(&[arg(&first), arg(&second)]).unwrap();

        assert!(first.is_dir());
        assert!(second.is_dir());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn cp_copies_a_file_into_an_existing_directory() {
        let root = test_root("cp");
        let source = root.join("new_doc.txt");
        let destination = root.join("new_folder2");
        fs::write(&source, "audit sample\n").unwrap();
        fs::create_dir(&destination).unwrap();

        crate::builtins::cp::cp(&[arg(&source), arg(&destination)]).unwrap();

        assert_eq!(
            fs::read_to_string(destination.join("new_doc.txt")).unwrap(),
            "audit sample\n"
        );
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn mv_moves_a_directory_into_an_existing_directory() {
        let root = test_root("mv");
        let source = root.join("new_folder2");
        let destination = root.join("new_folder1");
        fs::create_dir_all(source.join("nested")).unwrap();
        fs::create_dir(&destination).unwrap();

        crate::builtins::mv::mv(&[arg(&source), arg(&destination)]).unwrap();

        assert!(!source.exists());
        assert!(destination.join("new_folder2/nested").is_dir());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rm_recursively_removes_a_directory_tree() {
        let root = test_root("rm");
        let tree = root.join("new_folder1/new_folder2");
        fs::create_dir_all(&tree).unwrap();
        fs::write(tree.join("new_doc.txt"), "audit sample\n").unwrap();

        crate::builtins::rm::rm(&["-r".to_string(), arg(&root.join("new_folder1"))]).unwrap();

        assert!(!root.join("new_folder1").exists());
        fs::remove_dir_all(root).unwrap();
    }
}

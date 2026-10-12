use std::fs;
use std::path::{Path, PathBuf};

pub fn ls(current_dir: &Path, args: &[String]) -> Result<(), String> {
    let mut show_hidden = false;
    let mut long_listing = false;
    let mut marker = false;
    let mut target_dir = current_dir.to_path_buf();
    let mut pos_args = Vec::new();

    for arg in args {
        if arg.starts_with('-') && arg != "-" {
            for ch in arg.chars().skip(1) {
                match ch {
                    'a' => show_hidden = true,
                    'l' => long_listing = true,
                    'F' => marker = true,
                    _ => return Err(format!("ls: invalid option -- '{}'", ch)),
                }
            }
        } else {
            pos_args.push(arg.clone());
        }
    }

    if pos_args.len() > 1 {
        return Err("ls: too many arguments".to_string());
    }

    if let Some(path_arg) = pos_args.first() {
        let candidate = Path::new(path_arg);
        target_dir = if candidate.is_absolute() {
            PathBuf::from(path_arg)
        } else {
            current_dir.join(path_arg)
        };
    }

    let dir_entries = fs::read_dir(&target_dir)
        .map_err(|err| format!("ls: {}: {}", target_dir.display(), err))?;

    let mut entries = Vec::new();
    for entry in dir_entries {
        let entry = entry.map_err(|err| format!("ls: {}: {}", target_dir.display(), err))?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if !show_hidden && file_name.starts_with('.') {
            continue;
        }
        let path = target_dir.join(&file_name);
        let metadata = fs::symlink_metadata(&path)
            .map_err(|err| format!("ls: {}: {}", path.display(), err))?;
        entries.push((file_name, path, metadata));
    }
    if show_hidden {
        for (name, path) in [
            (".".to_string(), target_dir.clone()),
            (
                "..".to_string(),
                target_dir.parent().unwrap_or(&target_dir).to_path_buf(),
            ),
        ] {
            let metadata = fs::symlink_metadata(&path)
                .map_err(|err| format!("ls: {}: {}", path.display(), err))?;
            entries.push((name, path, metadata));
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    if long_listing {
        print_long_listing(&entries, marker)?;
    } else {
        let rendered: Vec<String> = entries
            .iter()
            .map(|(name, _, metadata)| style_name(name, metadata, marker))
            .collect();
        println!("{}", rendered.join(" "));
    }

    Ok(())
}

fn print_long_listing(
    entries: &[(String, PathBuf, fs::Metadata)],
    marker: bool,
) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;

        let total_blocks = entries
            .iter()
            .map(|(_, _, metadata)| metadata.blocks())
            .fold(0_u64, u64::saturating_add);
        println!("total {}", total_blocks);

        for (name, path, metadata) in entries {
            let permissions = permissions(metadata);
            let mut display_name = style_name(name, metadata, marker);
            if metadata.file_type().is_symlink() {
                let target = fs::read_link(path)
                    .map_err(|err| format!("ls: {}: {}", path.display(), err))?;
                display_name.push_str(&format!(" -> {}", target.display()));
            }
            println!(
                "{} {:>3} {:>5} {:>5} {:>8} {} {}",
                permissions,
                metadata.nlink(),
                metadata.uid(),
                metadata.gid(),
                metadata.size(),
                format_timestamp(metadata.mtime()),
                display_name,
            );
        }
    }

    #[cfg(not(unix))]
    {
        for (name, _, metadata) in entries {
            let modified = metadata
                .modified()
                .map(|time| format!("{time:?}"))
                .unwrap_or_else(|_| "-".to_string());
            println!(
                "{} {:>8} {} {}",
                if metadata.is_dir() { "d" } else { "-" },
                metadata.len(),
                modified,
                style_name(name, metadata, marker),
            );
        }
    }
    Ok(())
}

#[cfg(unix)]
fn permissions(metadata: &fs::Metadata) -> String {
    use std::os::unix::fs::MetadataExt;

    let mode = metadata.mode();
    let file_type = match mode & 0o170000 {
        0o040000 => 'd',
        0o120000 => 'l',
        0o020000 => 'c',
        0o060000 => 'b',
        0o010000 => 'p',
        0o140000 => 's',
        _ => '-',
    };
    let mut result = String::with_capacity(10);
    result.push(file_type);

    for (read, write, execute, special, special_char) in [
        (0o400, 0o200, 0o100, 0o4000, 's'),
        (0o040, 0o020, 0o010, 0o2000, 's'),
        (0o004, 0o002, 0o001, 0o1000, 't'),
    ] {
        result.push(if mode & read != 0 { 'r' } else { '-' });
        result.push(if mode & write != 0 { 'w' } else { '-' });
        result.push(match (mode & execute != 0, mode & special != 0) {
            (true, true) => special_char,
            (false, true) => special_char.to_ascii_uppercase(),
            (true, false) => 'x',
            (false, false) => '-',
        });
    }

    result
}

#[cfg(unix)]
fn format_timestamp(timestamp: i64) -> String {
    let days = timestamp.div_euclid(86_400);
    let seconds_in_day = timestamp.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_in_day / 3_600;
    let minute = seconds_in_day % 3_600 / 60;
    let month_name = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ][month as usize - 1];

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);

    if timestamp < now - 180 * 24 * 60 * 60 || timestamp > now + 60 * 60 {
        format!("{month_name} {day:>2}  {year}")
    } else {
        format!("{month_name} {day:>2} {hour:02}:{minute:02}")
    }
}

#[cfg(unix)]
fn civil_from_days(days_since_epoch: i64) -> (i64, u32, u32) {
    let days = days_since_epoch + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);

    (year, month as u32, day as u32)
}

fn style_name(name: &str, metadata: &fs::Metadata, marker: bool) -> String {
    let mut text = name.to_string();

    if marker {
        if metadata.is_dir() {
            text.push('/');
        } else if metadata.file_type().is_symlink() {
            text.push('@');
        } else if is_executable(metadata) {
            text.push('*');
        }
    }

    text
}

fn is_executable(metadata: &fs::Metadata) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        return metadata.permissions().mode() & 0o111 != 0;
    }

    #[cfg(not(unix))]
    {
        let _ = metadata;
        false
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::{civil_from_days, format_timestamp};

    #[test]
    fn formats_unix_epoch_timestamp() {
        assert_eq!(format_timestamp(0), "Jan  1  1970");
    }

    #[test]
    fn converts_leap_day_timestamp() {
        assert_eq!(civil_from_days(18_321), (2020, 2, 29));
    }
}

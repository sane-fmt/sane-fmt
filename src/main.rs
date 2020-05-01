mod act;
mod cli_opt;
mod cross_platform_path;
mod diff;
mod file_list;
mod rules;
mod term;

use cli_opt::{CliOpt, DetailLevel, When};
use path_slash::*;
use relative_path::RelativePath;
use rules::build_fmt;
use std::fs;
use term::color::{BoxedColorScheme, ColorfulScheme, ColorlessScheme};

fn main() -> Result<(), String> {
    let opt = CliOpt::get();

    let files = if opt.files.is_empty() {
        file_list::default_files()
    } else {
        file_list::create_list(
            opt.files
                .iter()
                .map(|x| cross_platform_path::from_string(x.as_str())),
        )
    }
    .map_err(|error| error.to_string())?;

    let file_count = files.len();
    let mut diff_count = 0;
    let mut skip_count = 0;
    let fmt = build_fmt();

    let theme: BoxedColorScheme = if opt.color == When::Never {
        Box::new(ColorlessScheme)
    } else {
        Box::new(ColorfulScheme)
    };

    let log_scan = act::log_scan::get(opt.color);
    let log_skip = act::log_skip::get(opt.details, opt.show_skipped, &theme);
    let log_same = act::log_same::get(opt.details, opt.hide_passed, &theme);
    let log_diff = act::log_diff::get(opt.details, &theme);
    let may_write = act::may_write::get(opt.write);
    let clear_current_line = act::may_clear_current_line::get(opt.color);

    for item in files {
        let file_list::Item {
            ref path,
            file_type: stats,
        } = item;
        if cfg!(win32) {
            println!("WINDOWS {:?}", path);
        }
        let path = &RelativePath::from_path(path)
            .unwrap()
            .normalize()
            .to_path("");
        log_scan(path);
        if !stats.is_file() {
            clear_current_line();
            log_skip(path);
            skip_count += 1;
            continue;
        }
        let file_content = fs::read_to_string(path)
            .map_err(|error| format!("Failed to read {:?}: {}", path.to_slash_lossy(), error))?;
        clear_current_line();
        let formatted = fmt
            .format_text(&path.to_path_buf(), &file_content)
            .map_err(|error| format!("Failed to parse {:?}: {}", path.to_slash_lossy(), error))?;
        if file_content == formatted {
            log_same(path);
        } else {
            diff_count += 1;
            log_diff(path, &file_content, &formatted);
            may_write(path, &formatted).map_err(|error| {
                format!("failed to write to {:?}: {}", path.to_slash_lossy(), error)
            })?;
        }
    }

    println!(
        "SUMMARY: total {}; changed {}; unchanged {}; skipped {}",
        file_count,
        diff_count,
        file_count - diff_count - skip_count,
        skip_count,
    );

    if file_count == 0 {
        return Err("No files found".to_string());
    }

    if !opt.write && diff_count != 0 {
        return Err(format!("There are {} unformatted files", diff_count));
    }

    Ok(())
}

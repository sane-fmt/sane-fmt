mod act;
mod cli_opt;
mod diff;
mod rules;
mod term;

use cli_opt::{CliOpt, DetailLevel, When};
use globwalk::GlobWalkerBuilder;
use relative_path::RelativePath;
use rules::build_fmt;
use std::{fs, path::Path};
use term::color::{BoxedColorScheme, ColorfulScheme, ColorlessScheme};

fn main() -> Result<(), String> {
    use structopt::*;
    let opt: CliOpt = CliOpt::from_args();

    let patterns = &if opt.patterns.len() != 0 {
        opt.patterns
    } else {
        vec![
            "*.{ts,js,tsx,jsx}".to_string(),
            "!*.d.ts".to_string(),
            "!.git".to_string(),
            "!node_modules".to_string(),
        ]
    };

    let walker = GlobWalkerBuilder::from_patterns(".", patterns)
        .follow_links(false)
        .build()
        .map_err(|error| format!("error: {}", error))?
        .into_iter();

    let mut file_count = 0;
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

    for res in walker {
        let entry = res.map_err(|error| format!("Unexpected Error: {}", error))?;
        let path: &Path = &RelativePath::from_path(entry.path())
            .unwrap()
            .normalize()
            .to_path("");
        log_scan(path);
        let stats = fs::symlink_metadata(path).map_err(|error| error.to_string())?;
        if !stats.is_file() {
            clear_current_line();
            log_skip(path);
            skip_count += 1;
            continue;
        }
        let file_content = fs::read_to_string(path)
            .map_err(|error| format!("Failed to read {:?}: {}", path, error))?;
        clear_current_line();
        let formatted = fmt
            .format_text(&path.to_path_buf(), &file_content)
            .map_err(|error| format!("Failed to parse {:?}: {}", path, error))?;
        if file_content == formatted {
            log_same(path);
        } else {
            diff_count += 1;
            log_diff(path, &file_content, &formatted);
            may_write(path, &formatted)
                .map_err(|error| format!("failed to write to {:?}: {}", path, error))?;
        }
        file_count += 1;
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

use super::super::{
    act,
    cli_opt::{LogFormat, When},
    cross_platform_path, file_list,
    term::color::{BoxedColorScheme, ColorfulScheme, ColorlessScheme},
};
use super::App;
use relative_path::RelativePath;
use std::{fs, path::MAIN_SEPARATOR};

impl App {
    /// Run the program base on application state.
    pub fn run(&self) -> Result<(), String> {
        let Self { opt, fmt } = self;

        let files = if opt.files.is_empty() {
            file_list::default_files()
        } else {
            file_list::create_list(
                opt.files
                    .iter()
                    .map(|x| cross_platform_path::from_string(x.as_str(), MAIN_SEPARATOR)),
            )
        }
        .map_err(|error| error.to_string())?;

        let file_count = files.len();
        let mut diff_count = 0;

        let theme: BoxedColorScheme = if opt.color == When::Never {
            Box::new(ColorlessScheme)
        } else {
            Box::new(ColorfulScheme)
        };

        let log_same = act::log_same::get(opt.details, opt.hide_passed, &theme);
        let log_diff = act::log_diff::get(opt.details, opt.log_format, &theme);
        let may_write = act::may_write::get(opt.write);

        for item in files {
            let file_list::Item { path, .. } = item;

            // Problem: RelativePath only recognize unix path separator
            // Workaround: Always use unix path separator
            let path = if cfg!(unix) {
                path
            } else {
                // This is an expensive operation, therefore should only be performed when necessary
                cross_platform_path::convert_path(&path, '/')
            };

            let path = RelativePath::from_path(&path)
                .unwrap()
                .normalize()
                .to_path("");

            // Because of the above workaround, this is necessary
            let path = if cfg!(unix) {
                path
            } else {
                cross_platform_path::convert_path(&path, MAIN_SEPARATOR)
            };

            let path = &path;
            let file_content = fs::read_to_string(path).map_err(|error| {
                format!(
                    "Failed to read {:?}: {}",
                    cross_platform_path::to_string(path, '/'),
                    error
                )
            })?;

            let formatted = fmt
                .format_text(&path.to_path_buf(), &file_content)
                .map_err(|error| {
                    format!(
                        "Failed to parse {:?}: {}",
                        cross_platform_path::to_string(path, '/'),
                        error
                    )
                })?;
            if file_content == formatted {
                log_same(path);
            } else {
                diff_count += 1;
                log_diff(path, &file_content, &formatted);
                may_write(path, &formatted).map_err(|error| {
                    format!(
                        "failed to write to {:?}: {}",
                        cross_platform_path::to_string(path, '/'),
                        error
                    )
                })?;
            }
        }

        println!(
            "SUMMARY: total {}; changed {}; unchanged {}",
            file_count,
            diff_count,
            file_count - diff_count,
        );

        if opt.log_format == LogFormat::GitHubActions {
            println!("::set-output name=total::{}", file_count);
            println!("::set-output name=changed::{}", diff_count);
            println!("::set-output name=unchanged::{}", file_count - diff_count);
        }

        if file_count == 0 {
            return Err("No files found".to_owned());
        }

        if !opt.write && diff_count != 0 {
            return Err(format!("There are {} unformatted files", diff_count));
        }

        Ok(())
    }
}

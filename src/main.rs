mod cli_opt;

use std::{
    env,
    fs,
    path::Path,
};
use globwalk::GlobWalkerBuilder;
use cli_opt::CliOpt;

fn main() -> Result<(), String> {
    use structopt::*;
    let opt: CliOpt = CliOpt::from_args();

    let patterns = &if opt.patterns.len() != 0 {
        opt.patterns
    } else {
        vec![
            "*.{ts,js}".to_owned(),
            "!*.d.ts".to_owned(),
            "!.git".to_owned(),
            "!node_modules".to_owned(),
        ]
    };

    let current_dir = env::current_dir().unwrap();
    let walker = GlobWalkerBuilder::from_patterns(current_dir, patterns)
        .follow_links(false)
        .build()
        .map_err(|error| format!("error: {}", error))?
        .into_iter();

    let mut count = 0;
    let mut skip_count = 0;
    let mut err_count = 0;

    for res in walker {
        match res {
            Ok(entry) => {
                let path: &Path = entry.path();
                let stats = fs::metadata(path)
                    .map_err(|error| error.to_string())?;
                if stats.is_file() {
                    println!("file {:?}", path);
                    count += 1;
                } else {
                    println!("skip {:?} (not a file)", path);
                    skip_count += 1;
                }
                count += 1;
            },

            Err(error) => {
                eprintln!("error: {}", error);
                err_count += 1;
            },
        }
    }

    println!("SUMMARY: scanned {}; skipped {}; failed {}", count, skip_count, err_count);

    if err_count != 0 {
        return Err(format!("There are {} errors", err_count));
    }

    if count == 0 {
        return Err("No files found".to_owned());
    }

    Ok(())
}

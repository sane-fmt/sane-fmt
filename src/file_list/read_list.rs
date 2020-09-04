use super::super::cli_opt::InputStreamAddress;
use super::{create_list, List};
use pipe_trait::*;
use std::{
    fs::File,
    io::{self, stdin, Read},
    path::PathBuf,
};

/// Read a list of TypeScript/JavaScript files from an `InputStreamAddress`
pub fn read_list(list_file_address: &InputStreamAddress) -> io::Result<List> {
    let mut list_file: Box<dyn Read> = match list_file_address {
        InputStreamAddress::Stdin => Box::new(stdin()),
        InputStreamAddress::File(path) => path.pipe(File::open)?.pipe(Box::new),
    };

    let mut list_file_content = String::new();
    list_file.read_to_string(&mut list_file_content)?;

    list_file_content
        .lines()
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .pipe(create_list)
}

use super::super::cli_opt::InputStreamAddress;
use super::{create_list, Error, List};
use pipe_trait::*;
use std::{
    fs::File,
    io::{stdin, Read},
    path::PathBuf,
};

/// Read a list of TypeScript/JavaScript files from an `InputStreamAddress`
pub fn read_list(list_file_address: &InputStreamAddress) -> Result<List, Error> {
    let (mut list_file, path): (Box<dyn Read>, PathBuf) = match list_file_address {
        InputStreamAddress::Stdin => (Box::new(stdin()) as Box<dyn Read>, PathBuf::from("STDIN")),
        InputStreamAddress::File(path) => path
            .pipe(File::open)
            .map_err(|error| Error::new(path.clone(), error))?
            .pipe(|x| Box::new(x) as Box<dyn Read>)
            .pipe(|x| (x, path.clone())),
    };

    let mut list_file_content = String::new();
    list_file
        .read_to_string(&mut list_file_content)
        .map_err(|error| Error::new(path, error))?;

    list_file_content
        .lines()
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .pipe(create_list)
}

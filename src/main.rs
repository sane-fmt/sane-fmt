mod cli_opt;

use cli_opt::CliOpt;

fn main() {
    use structopt::*;
    let opt: CliOpt = CliOpt::from_args();
    println!("{:?}", opt);
}

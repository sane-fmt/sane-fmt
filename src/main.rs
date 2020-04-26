mod cli_opt;

fn main() {
    use structopt::*;
    let opt = cli_opt::CliOpt::from_args();
    println!("{:?}", opt);
}

use clap::Parser;
use rdr_compose::ComposeFile;

#[derive(Parser, Debug)]
#[clap(name = "rdr-compose", about = "Radar Route Compose")]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let compose: ComposeFile = toml::from_str(&std::fs::read_to_string(&args.path).unwrap()).unwrap();
    println!("{:#?}", compose);
    // compose.executables.iter().for_each(|exec| {
    //     println!("{:?}", exec);
    // });
}

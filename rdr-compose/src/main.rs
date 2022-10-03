use std::collections::HashMap;
use clap::Parser;
use rdr_compose::{attribute_id_to_index, ComposeFile};
use rdr_compose::model::Executable;

#[derive(Parser, Debug)]
#[clap(name = "rdr-compose", about = "Radar Route Compose")]
struct Cli {
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,

    dry_run: bool,
}

fn main() {
    let args = Cli::parse();
    let compose: ComposeFile = toml::from_str(&std::fs::read_to_string(&args.path).unwrap()).unwrap();
    // println!("{:#?}", compose);

    let mut port_inc = 5555;

    let out_port_map: Vec<(&Executable, HashMap<&String, i32>)> =
        compose.executables.iter().map(
            |exe| (exe, exe.outputs.iter().map(|name| {
                port_inc += 1;
                println!("{}.{} will take port {}", name, exe.name, port_inc);
                (name, port_inc)
            }).collect())).collect();


    for ((si, soi), (ci, cii))
    in compose.links.iter().map(|(s, c)| (attribute_id_to_index(*s), attribute_id_to_index(*c))) {
        let (server, server_ports) = &out_port_map[si];
        let client = &compose.executables[ci];
        println!("{}.{} will subscribe to {}.{} (port {})", client.name, client.inputs[cii], server.name, server.outputs[soi], server_ports[&server.outputs[soi]]);
    }

    if args.dry_run {}
}

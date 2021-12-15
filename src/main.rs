use std::net::IpAddr;
use std::path::Path;
use std::process::exit;
use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use log::error;

mod pack;

fn main() {
    env_logger::init();

    let matches = app_from_crate!()
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(clap::SubCommand::with_name("pack")
            .help("Create a httpack file")
            .arg(clap::Arg::with_name("input")
                .help("Path to the directory tree to pack for use by \"httpack serve\"")
                .value_name("DIRECTORY")
                .required(true))
            .arg(clap::Arg::with_name("output")
                .long("output")
                .help("Output filename")
                .value_name("FILENAME")
                .default_value("files.httpack")))
        .subcommand(clap::SubCommand::with_name("serve")
            .help("Serve a httpack file")
            .arg(clap::Arg::with_name("addr")
                .long("bind-addr")
                .help("Local address to bind to")
                .env("BIND_ADDR")
                .value_name("IP")
                .default_value("0.0.0.0")
                .validator(|v| v.parse::<IpAddr>().map(|_| ()).map_err(|e| e.to_string())))
            .arg(clap::Arg::with_name("port")
                .long("bind-port")
                .help("Port to bind to")
                .env("BIND_PORT")
                .value_name("PORT")
                .default_value("8080")
                .validator(|v| v.parse::<u16>().map(|_| ()).map_err(|e| e.to_string())))
            .arg(clap::Arg::with_name("pack-path")
                .help("Path to the pack file created by \"httpack pack\"")
                .env("PACK_PATH")
                .value_name("FILENAME")
                .default_value("files.httpack")))
        .get_matches();

    match matches.subcommand() {
        ("pack", Some(sub_m)) => {
            let input_path = Path::new(sub_m.value_of("input").unwrap());
            let output_path = Path::new(sub_m.value_of("output").unwrap());
            if !input_path.is_dir() {
                error!("input path {} isn't a directory", input_path.display());
                exit(1);
            }
            if output_path.exists() {
                error!("output path {} already exists", output_path.display());
                exit(1);
            }

            match crate::pack::create_pack(input_path, output_path) {
                Ok(()) => {}
                Err(e) => {
                    error!("Error creating pack: {}", e);
                    exit(1);
                }
            }
        }
        ("serve", Some(sub_m)) => {
            unimplemented!();
        }
        (s, _) => panic!("Unrecognized command {}", s)
    }
}

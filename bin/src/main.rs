use build::{
    config::read_config, create_journal_entry, get_config_location, get_data_dir_location, install,
    pages::Builder, update, RefBuilder,
};
use std::{path::PathBuf, process::exit, time::Instant};
use tasks::{git_update, normalize_wiki_location, sync};
use www::server;

#[tokio::main]
async fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut build_all = false;
    if !args.is_empty() {
        let arg = args[0].as_str();
        match arg {
            "-v" | "--version" => return print_version(),
            "-h" | "--help" => return print_help(),
            "-b" | "--build" => build_all = true,
            "-i" | "--init" => return install(),
            "-u" | "--update" => return update(),
            _ => {
                if arg.starts_with('-') {
                    eprintln!("unknown option: {}", arg);
                    exit(1);
                }
                if !arg.is_empty() {
                    let config = read_config();
                    let location = normalize_wiki_location(&config.general.wiki_location);
                    create_journal_entry(location.clone(), args.join(" ")).unwrap();
                    if config.sync.use_git {
                        git_update(&location, config.sync.branch);
                    }
                    exit(0);
                }
            }
        }
    }
    let config = read_config();
    let location = normalize_wiki_location(&config.general.wiki_location);
    if build_all {
        let now = Instant::now();
        if PathBuf::from("./public").exists() {
            std::fs::remove_dir_all("./public").unwrap();
        }
        let builder = Builder::new();
        builder.sweep(&location);
        builder.compile_all();
        println!("Built static site in: {}ms", now.elapsed().as_millis());
    } else {
        if config.sync.use_git {
            sync(&location, config.sync.sync_interval, config.sync.branch).await;
        }
        let mut ref_builder = RefBuilder::new();
        ref_builder.build(&location);
        server(config.general, ref_builder).await;
    }
}

fn print_version() {
    println!("tendril-wiki v{}", env!("CARGO_PKG_VERSION"),);
}

fn print_help() {
    println!(
        "\nConfig file found at {}\nInstall files found at {}\n",
        format!("\x1b[38;5;47m{:#?}\x1b[0m", get_config_location().0),
        format!("\x1b[38;5;37m{:#?}\x1b[0m", get_data_dir_location())
    );
    print!(
        "Usage: tendril [options]
        Options:
        -i, --init                   Initialize config file and install
        -b, --build                  Build all pages as HTML and output to ./public
        -v, --version                Print version.
        -h, --help                   Show this message.
        -u, --update                 Update the installation by copying over any new files or updating config.toml.

",
    );
}

// TODO: Maybe later add in multi-config, multi-folder stuff
// -c <path>, --config <path>   Use config at <path>
//
// Examples:

//   - Start wiki in the ~/work/wiki directory
//         $ tendril ~/work/wiki
//   - Start wiki at location specified in config file
//         $ tendril
//   - Start wiki in current folder with a custom config file
//         $ tendril . -c ./config.toml

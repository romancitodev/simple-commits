use clap::Parser;
use cli::Command;
use merge2::Merge;

pub mod cli;
pub mod git;
pub mod helpers;

pub fn get_config() -> (cli::SimpleCommitsConfig, Option<Command>) {
    let mut args = cli::CliConfig::parse();
    let mut config = cli::SimpleCommitsConfig::default();

    match args.mode {
        Some(Command::Init(option)) => {
            let path = helpers::create_config(option);
            config.config = path;
        }
        _ => {
            let (global_path, local_path) = helpers::load_config(args.config, &mut config);

            if let Some(local_path) = local_path {
                config.config = local_path;
            } else {
                config.config = global_path;
            }
            config.merge(&mut args.sc_config);
        }
    }
    (config, args.mode)
}

pub fn start_logging() {
    env_logger::builder().init();
}

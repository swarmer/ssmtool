mod cmdline;
mod env;

use log::error;
use tokio::runtime::Runtime;

pub async fn cli_future() -> i32 {
    let mut app = cmdline::build_clap_app();
    let matches = app.clone().get_matches();

    let result = match matches.subcommand_name() {
        Some("env") => {
            let submatches = matches.subcommand_matches("env").unwrap();
            let args = env::EnvArgs {
                path: String::from(submatches.value_of("PATH").unwrap()),
                uppercase: submatches.is_present("uppercase"),
                add_prefix: submatches.value_of("add-prefix").map(String::from),
                command: submatches
                    .values_of("COMMAND")
                    .unwrap()
                    .map(String::from)
                    .collect(),
            };
            env::env_subcommand(args).await
        }

        None => {
            app.print_help().unwrap();
            Ok(1)
        }
        Some(unknown_name) => {
            panic!("Unexpected subcommand: {}", unknown_name);
        }
    };

    match result {
        Ok(exit_code) => exit_code,
        Err(error) => {
            error!("{}", error);
            for underlying_error in error.iter_causes() {
                error!("  Caused by: {}", underlying_error);
            }
            1
        }
    }
}

pub fn run() -> i32 {
    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    Runtime::new().unwrap().block_on(cli_future())
}

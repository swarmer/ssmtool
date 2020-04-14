use clap::{App, Arg, SubCommand};

pub fn build_clap_app() -> App<'static, 'static> {
    App::new("ssmtool")
        .version(crate::VERSION)
        .author("Author: Anton Barkovsky")
        .about("A CLI for SSM Parameter Store")
        .subcommand(
            SubCommand::with_name("env")
                .about("Run a command with environment populated by SSM parameters")
                .arg(
                    Arg::with_name("prefix")
                        .short("p")
                        .takes_value(true)
                        .value_name("PREFIX")
                        .required(true)
                        .help("Prefix of the parameter names that should be put into environment"),
                ),
        )
}

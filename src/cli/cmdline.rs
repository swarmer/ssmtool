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
                    Arg::with_name("uppercase")
                        .short("u")
                        .long("uppercase")
                        .takes_value(false)
                        .help(concat!(
                            "Convert ssm parameter names to upper case when inserting them",
                            " into environment",
                        )),
                )
                .arg(
                    Arg::with_name("add-prefix")
                        .long("add-prefix")
                        .takes_value(true)
                        .value_name("PREFIX")
                        .required(false)
                        .help("A Prefix that will be added to environment variable names"),
                )
                .arg(
                    Arg::with_name("PATH")
                        .takes_value(true)
                        .required(true)
                        .help("Path to a directory of parameters, must start with slash"),
                )
                .arg(
                    Arg::with_name("COMMAND")
                        .multiple(true)
                        .required(true)
                        .help("Command that will be ran in the augmented environment"),
                ),
        )
}

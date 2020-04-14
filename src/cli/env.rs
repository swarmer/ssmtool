use std::collections::HashMap;
use std::time::Duration;

use failure::Error;
use log;
use rusoto_core::Region;
use rusoto_ssm::Ssm;
use rusoto_ssm as ssm;
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct EnvArgs {
    pub prefix: String,
}

fn get_client() -> ssm::SsmClient {
    let mut chain_provider = rusoto_core::credential::ChainProvider::new();
    chain_provider.set_timeout(Duration::from_secs(1));

    ssm::SsmClient::new_with(
        rusoto_core::request::HttpClient::new().unwrap(),
        chain_provider,
        Region::default(),
    )
}

async fn get_parameters(client: &ssm::SsmClient, prefix: &str) -> Result<Vec<ssm::Parameter>, Error> {
    let mut pagination_token = None;
    let mut parameters = vec![];

    loop {
        let result = client.get_parameters_by_path(ssm::GetParametersByPathRequest {
            next_token: pagination_token.clone(),

            path: String::from(prefix),
            recursive: Some(true),
            with_decryption: Some(true),

            ..ssm::GetParametersByPathRequest::default()
        }).await?;
        log::debug!("SSM result: {:?}", result);

        parameters.extend(result.parameters.unwrap());

        match result.next_token {
            Some(t) => {
                pagination_token = Some(t);
            },
            None => {
                break;
            }
        }
    }

    Ok(parameters)
}

fn build_env_map<'a, 'b>(prefix: &'a str, parameters: &'b Vec<ssm::Parameter>) -> HashMap<&'b str, &'b str> {
    let mut env = HashMap::new();

    for param in parameters {
        let param_name = param.name.as_ref().unwrap();

        assert!(param_name.starts_with(prefix));
        let variable_name = &param_name[prefix.len()..];
        env.insert(variable_name, param.value.as_ref().unwrap().as_str());
    }

    env
}

pub async fn env_subcommand(args: EnvArgs) -> Result<i32, Error> {
    let client = get_client();

    let parameters = get_parameters(&client, &args.prefix).await?;
    log::trace!("Using parameters: {:?}", parameters);

    let env = build_env_map(&args.prefix, &parameters);
    log::debug!("Using environment: {:?}", env);

    let exit_status: std::process::ExitStatus = Command::new("env")
        .envs(env)
        .status()
        .await?;
    log::debug!("Subcommand exited with status: {:?}", exit_status);

    Ok(exit_status.code().unwrap())
}

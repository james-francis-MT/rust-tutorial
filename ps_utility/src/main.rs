use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_ssm::{types::Parameter, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("eu-west-2");

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);

    let params = get_params(&client).await?;

    let _ = put_params(&client, &params).await.unwrap();

    Ok(())
}

async fn get_params(client: &Client) -> Result<Vec<Parameter>, Error> {
    let pages: Result<Vec<_>, _> = client
        .get_parameters_by_path()
        .path("/")
        .into_paginator()
        .send()
        .collect()
        .await;

    let mut params: Vec<Parameter> = vec![];

    for page in pages? {
        let mut page_params = page.parameters.unwrap();
        params.append(&mut page_params);
    }

    Ok(params)
}

async fn put_params(client: &Client, params: &Vec<Parameter>) -> Result<(), Error> {
    for param in params {
        let resp = client
            .put_parameter()
            .name(param.name.clone().unwrap())
            .value(param.value.clone().unwrap())
            .r#type(param.r#type.clone().unwrap())
            .overwrite(false)
            .send()
            .await;

        println!("{:?}", resp.unwrap());

        // TODO: ignore Parameter already exists exceptions
    }

    Ok(())
}

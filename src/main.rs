use std::str::FromStr;
use sui_sdk::{SuiClientBuilder, types::base_types::{SuiAddress, ObjectID}};

#[tokio::main]
async fn main() {
    // let url = "https://fullnode.devnet.sui.io:443";
    let url = "https://fullnode.testnet.sui.io:443";
    let sui_client_builder = SuiClientBuilder::default();
    let sui_client = sui_client_builder.build(url).await.unwrap();

    let coin_read_api = sui_client.coin_read_api();
    let read_api = sui_client.read_api();
    let governance_api = sui_client.governance_api();

    // これで特定アカウントのコイン全部取れる。
    let balances = coin_read_api.get_all_balances(
        SuiAddress::from_str("").unwrap(),
    ).await.unwrap();

    println!("{:#?}", balances);

    // これで名前とかdecimalsとかとれる
    let coin_metadata = coin_read_api.get_coin_metadata(
        "0x2::sui::SUI".to_string()
    ).await.unwrap();

    println!("{:#?}", coin_metadata);


    // TODO NFTどうやって特定するか
    let objects = read_api.get_objects_owned_by_address(
        SuiAddress::from_str("").unwrap(),
    ).await.unwrap();
    
    println!("{:#?}", objects);

    let parsed_object = read_api.get_parsed_object(
        ObjectID::from_str("").unwrap(),
    ).await.unwrap();

    println!("{:#?}", parsed_object);


    let stakes = governance_api.get_delegated_stakes(
        SuiAddress::from_str("").unwrap(),
    ).await.unwrap();

    println!("{:#?}", stakes);

    let validators = governance_api.get_validators().await.unwrap();
    println!("{:#?}", validators);

}
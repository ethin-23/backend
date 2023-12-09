use std::str::FromStr;

use jsonrpsee::{
    core::{async_trait, client::ClientT},
    http_client::HttpClientBuilder,
    proc_macros::rpc,
    rpc_params,
    types::{error::ErrorCode, Params},
    RpcModule,
};
use primitive_types::U256;
use serde_json::Value;

use crate::paillier;

pub fn parse_params(p: Params) -> Vec<String> {
    let params = p.as_str().unwrap().to_string().replace("\"", "");
    params.split(',').map(|s| s.to_string()).collect()
}

// pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
//     (0..s.len())
//         .step_by(2)
//         .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
//         .collect()
// }

pub fn encrypt(message: &str) -> anyhow::Result<(u128, u128, u128)> {
    let message = message.parse::<u128>()?;
    let n = dotenv::var("n").unwrap();
    let n = n.parse::<u128>()?;
    let g = dotenv::var("g").unwrap();
    let g = g.parse::<u128>()?;

    let r = rand::random::<u64>() & !1; // Remove one bit
    let c = paillier::encrypt(message, r.into(), n, g);

    Ok((n, c.as_u128(), r.into()))
}

pub struct RPC;

#[rpc(server)]
pub trait RPC: Send + Sync {
    #[method(name = "play")]
    async fn play(&self) -> String;

    #[method(name = "decipher")]
    async fn decipher(
        &self,
        addr: String,
        r: String,
        s: String,
        msg: String,
    ) -> Result<String, ErrorCode>;
}

#[async_trait]
impl RPCServer for RPC {
    async fn play(&self) -> String {
        println!("play");

        return "Hi".to_string();
    }
    async fn decipher(
        &self,
        addr: String,
        r: String,
        s: String,
        rnd: String,
        msg: String,
    ) -> Result<String, ErrorCode> {
        println!("decipher");
        // verify sig ECDSA
        let starknet_goerli_alchemy_rpc_url: String =
            dotenv::var("STARKNET_GOERLI_ALCHEMY_RPC_URL").unwrap();
        let contract_address = "0x009522ba2de47439843d43509bb6d3e968e6e3c8bf08254de6db9eab3a13434e";
        // entry point selector for is_valid_signature
        let is_valid_signature =
            "0x28420862938116cb3bbdbedee07451ccc54d4e9412dbef71142ad1980a30941";

        let http_client = HttpClientBuilder::default()
            .build(starknet_goerli_alchemy_rpc_url)
            .unwrap();

        let mut raw_params = serde_json::json!({
            "request": {
                "contract_address": contract_address,
                "entry_point_selector": is_valid_signature,
                "calldata": [msg, 0x2, r, s]
            },
            "block_id": "pending"
        });

        let params = rpc_params! {raw_params};
        let rpc_response: std::result::Result<std::option::Option<String>, String> = http_client
            .request("is_valid_signature", params)
            .await
            .unwrap();
        match rpc_response {
            Ok(Some(s)) => println!("ok some {s}"),
            Ok(None) => println!("ok none"),
            Err(e) => println!("%%%Error: {e:?}"),
        };

        // Fetch amount of address from contract
        // entrypoint selector for balance_of
        let balance_of = "0x035a73cd311a05d46deda634c5ee045db92f811b4e74bca4437fcb5302b7af33";
        raw_params = serde_json::json!({
            "request": {
                "contract_address": contract_address,
                "entry_point_selector": balance_of,
                "calldata": [addr]
            },
            "block_id": "pending"
        });
        let params = rpc_params! {raw_params};
        let rpc_response: std::result::Result<std::option::Option<String>, String> =
            http_client.request("starknet_call", params).await.unwrap();
        let mut c: u128 = 0;
        match rpc_response {
            Ok(Some(s_)) => {
                println!("ok some {s_}");
                c = s_.parse::<u128>().unwrap();
            }
            Ok(None) => println!("ok none"),
            Err(e) => println!("%%%Error: {e:?}"),
        };

        // convert string to u128
        let n = dotenv::var("n").unwrap();
        let n = n.parse::<u128>().unwrap();
        let mu = dotenv::var("mu").unwrap();
        let mu = U256::from_str(&mu).unwrap();
        // convert string to U256
        let lambda = U256::from(dotenv::var("lambda").unwrap().parse::<u128>().unwrap());

        // Decrypt the amount
        let amount = paillier::decrypt(c, lambda, n, mu);

        // Send the decrypted amount
        Ok(serde_json::json!({
            "amount": amount.as_u128(),
        })
        .to_string())
    }
}
pub fn register_methods<Context: Send + Sync + 'static>(
    module: &mut RpcModule<Context>,
) -> anyhow::Result<()> {
    module.register_method("encrypt", |params, _| {
        let params = parse_params(params);
        let amount = &params[1];

        match encrypt(&amount) {
            Ok((n, c, r)) => {
                return serde_json::json!({
                    "cipher": c.to_string(),
                    "r": r.to_string(),
                    "n": n.to_string(),
                });
            }
            Err(er) => Value::String(format!("{er:?}")),
            // Err(er) => return Err(ErrorCode::InvalidParams),
        }
    })?;

    module.merge(RPC.into_rpc())?;
    Ok(())
}

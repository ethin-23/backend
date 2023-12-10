use jsonrpsee::{
    core::async_trait,
    proc_macros::rpc,
    types::{error::ErrorCode, Params},
    RpcModule,
};

use primitive_types::U256;

use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};

use crate::paillier;

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
    #[method(name = "decipher")]
    async fn decipher(&self, addr: String, balance: String) -> Result<String, ErrorCode>;
    #[method(name = "encrypt")]
    async fn encrypt(&self, addr: String, amount: String) -> Result<String, ErrorCode>;
}

#[async_trait]
impl RPCServer for RPC {
    async fn decipher(&self, _addr: String, balance: String) -> Result<String, ErrorCode> {
        // convert string to u128
        let n = dotenv::var("n").unwrap().parse::<u128>().unwrap();
        let balance = balance.parse::<u128>().unwrap();
        let mu = U256::from(dotenv::var("mu").unwrap().parse::<u128>().unwrap());
        // convert string to U256
        let lambda = U256::from(dotenv::var("lambda").unwrap().parse::<u128>().unwrap());

        // Decrypt the amount
        let amount = paillier::decrypt(balance, lambda, n, mu);

        // Send the decrypted amount
        Ok(serde_json::json!({
            "amount": amount,
        })
        .to_string())
    }
    async fn encrypt(&self, _addr: String, amount: String) -> Result<String, ErrorCode> {
        match encrypt(&amount) {
            Ok((n, c, r)) => Ok(serde_json::json!({
                "cipher": c.to_string(),
                "r": r.to_string(),
                "n": n.to_string(),
            })
            .to_string()),
            Err(er) => Ok(format!("{er:?}")),
        }
    }
}
pub fn register_methods<Context: Send + Sync + 'static>(
    module: &mut RpcModule<Context>,
) -> anyhow::Result<()> {
    let rpc_url: String = dotenv::var("STARKNET_GOERLI_ALCHEMY_RPC_URL").unwrap();
    let rpc = JsonRpcClient::new(HttpTransport::new(url::Url::parse(&rpc_url).unwrap()));

    module.merge(RPC.into_rpc())?;
    Ok(())
}

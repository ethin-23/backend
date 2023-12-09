use jsonrpsee::{types::Params, RpcModule};
use rug::{rand::RandState, Integer};
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

    let mut rng = RandState::new();
    let r = Integer::from(Integer::random_bits(62, &mut rng)).to_u128_wrapping();
    let c = paillier::encrypt(message, r, n, g);

    Ok((n, c.as_u128(), r))
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

    Ok(())
}

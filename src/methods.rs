use fast_paillier::{Ciphertext, Nonce};
use jsonrpsee::{types::Params, RpcModule};
use rug::{rand::RandState, Integer};
use serde_json::Value;

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

pub fn encrypt(message: &str) -> anyhow::Result<(Integer, Ciphertext, Nonce)> {
    let n = Integer::from(678348576345876347_i128);

    println!("let key");
    let key = fast_paillier::EncryptionKey::from_n(n.clone());
    println!("let mut rng");
    let mut rng = RandState::new();
    println!("let r");
    let r = Integer::random_bits(64, &mut rng).into();
    println!("message {message}");
    let message = message.parse::<u128>().unwrap();

    println!("let c");
    let c = key.encrypt_with(&message.into(), &r)?;

    Ok((n, c, r))
}

pub fn register_methods<Context: Send + Sync + 'static>(
    module: &mut RpcModule<Context>,
) -> anyhow::Result<()> {
    module.register_method("encrypt", |params, _| {
        let params = parse_params(params);
        let _recepient = &params[0];
        let amount = &params[1];
        println!("Params {}:\n {}|{}", params.len(), params[0], params[1]);

        match encrypt(&amount) {
            Ok((n, c, r)) => {
                return serde_json::json!({
                    "cipher": c.to_i128().unwrap().to_string(),
                    "r": r.to_i128().unwrap().to_string(),
                    "n": n.to_i128().unwrap().to_string(),
                });
            }
            Err(er) => Value::String(format!("{er:?}")),
            // Err(er) => return Err(ErrorCode::InvalidParams),
        }
    })?;

    Ok(())
}

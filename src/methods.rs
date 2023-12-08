use jsonrpsee::{types::Params, RpcModule};

pub fn parse_params(p: Params) -> Vec<String> {
    let params = p.as_str().unwrap();
    params.split(',').map(|s| s.to_string()).collect()
}

pub fn register_methods<Context: Send + Sync + 'static>(
    module: &mut RpcModule<Context>,
) -> anyhow::Result<()> {
    module.register_method("encrypt", |params, _| {
        let params = parse_params(params);
        println!("Params {}:\n {}|{}", params.len(), params[0], params[1]);
        let c: u64 = 0xc54a38b7a46;
        let r: u64 = 0xb34ae908d7c;

        format!(r#"{{"cipher":"{}","r":"{}"}}"#, c, r)
    })?;

    Ok(())
}

use jsonrpsee::RpcModule;

pub fn register_methods<Context: Send + Sync + 'static>(
    module: &mut RpcModule<Context>,
) -> anyhow::Result<()> {
    module.register_method("say_hello", |_, _| {
        println!("say_hello method called!");
        "Hello there!!"
    })?;

    Ok(())
}

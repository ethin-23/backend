use jsonrpsee::RpcModule;

pub fn register_methods(module: &mut RpcModule) {
    module.register_method("say_hello", |_, _| {
        println!("say_hello method called!");
        "Hello there!!"
    })?;
}

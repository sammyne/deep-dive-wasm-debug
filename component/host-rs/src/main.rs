use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;
use wasmtime::component::{Component, Linker, Val};
use wasmtime::{Config, Engine, OptLevel, Store};
use wasmtime_wasi::WasiCtxBuilder;

fn main() -> anyhow::Result<()> {
    let Cli { path, name } = Cli::parse();

    let mut config = Config::default();
    config
        .wasm_component_model(true)
        .debug_info(true)
        .cranelift_opt_level(OptLevel::None);

    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Add the command world (aka WASI CLI) to the linker
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("link command world")?;

    let ctx = WasiCtxBuilder::new().inherit_stdout().build_p1();
    let mut store = Store::new(&engine, ctx);

    let component = Component::from_file(&engine, path).context("Component file not found")?;

    const INTERFACE: &str = "sammyne:helloworld/greeter@1.0.0";
    const FUNC_NAME: &str = "say-hello";
    const FUNC_NAME2: &str = "say-hello-again";

    let instance = linker
        .instantiate(&mut store, &component)
        .context("instantiate")?;

    // ref: https://docs.rs/wasmtime/30.0.2/wasmtime/component/struct.Instance.html#method.get_func
    let say_hello = {
        let instance_idx = instance
            .get_export(&mut store, None, INTERFACE)
            .with_context(|| format!("miss interface: {INTERFACE}"))?;
        let func_idx = instance
            .get_export(&mut store, Some(&instance_idx), FUNC_NAME)
            .with_context(|| format!("locate func '{FUNC_NAME}'"))?;
        instance
            .get_func(&mut store, &func_idx)
            .with_context(|| format!("load func '{FUNC_NAME}'"))?
    };
    let params = [new_hello_request(name.clone())];
    let mut results = [Val::Bool(false)];
    say_hello
        .call(&mut store, &params, &mut results)
        .context("call")?;
    // post-return 清理 say-hello 关联的状态。
    say_hello
        .post_return(&mut store)
        .with_context(|| format!("post return '{FUNC_NAME}'"))?;
    println!("say-hello returns {results:?}");

    // ref: https://docs.rs/wasmtime/30.0.2/wasmtime/component/struct.Instance.html#method.get_func
    let say_hello_again = instance
        .get_func(&mut store, FUNC_NAME2)
        .with_context(|| format!("load func '{FUNC_NAME2}'"))?;
    let params = [new_hello_request(name)];
    let mut results = [Val::Bool(false)];
    say_hello_again
        .call(&mut store, &params, &mut results)
        .context("call")?;
    // post-return 清理 say-hello 关联的状态。
    say_hello_again
        .post_return(&mut store)
        .with_context(|| format!("post return '{FUNC_NAME}'"))?;
    println!("say-hello-again returns {results:?}");

    Ok(())
}

/// A CLI for executing WebAssembly components that
/// implement the `example` world.
#[derive(Parser)]
#[clap(name = "hello-world-host", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    /// WASM 组件的路径
    #[clap(short, long)]
    path: PathBuf,
    /// greet 函数的入参依赖的 name 字段。
    #[clap(short, long)]
    name: String,
}

fn new_hello_request(name: String) -> Val {
    Val::Record(vec![("name".to_owned(), Val::String(name))])
}

# Extism Rust PDK

[![crates.io](https://img.shields.io/crates/v/extism_pdk.svg)](https://crates.io/crates/extism-pdk)

This library can be used to write [Extism Plug-ins](https://extism.org/docs/concepts/plug-in) in Rust.

## Install

Make sure you generate a `lib` project:

```bash
cargo new --lib my-plugin
```

Add the library from [crates.io](https://crates.io/crates/extism-pdk).

```bash
cargo add extism-pdk
```

Change your `Cargo.toml` to set the crate-type to `cdylib`:

```toml
[lib]
crate_type = ["cdylib"]
```

## Getting Started

The goal of writing an [Extism plug-in](https://extism.org/docs/concepts/plug-in) is to compile the your rust code to a Wasm module with some special export functions that the host application can invoke. The first thing you should understand is creating an export. Let's write a simple program that exports a `greet` function which will take a name as a string and return a greeting string:


```rust
use extism_pdk::*;

#[plugin_fn]
pub fn greet(name: String) -> FnResult<String> {
    Ok(format!("Hello, {}!", name))
}
```

Since we don't need any system access for this, we can compile this to the lightweight `wasm32-unknown-unknown` target instead of using the `wasm32-wasi` target:

```bash
cargo build --target wasm32-unknown-unknown
```

> *Note*: You can also put a default target in `.cargo/config.toml`:
> ```toml
> [build]
>   target = "wasm32-unknown-unknown"
> ```

This will put your compiled wasm in `target/wasm32-unknown-unknown/debug`.
We can now test it using the [Extism CLI](https://github.com/extism/cli)'s `run`
command:

```bash
extism call target/wasm32-unknown-unknown/debug/my_plugin.wasm greet --input "Benjamin"
# => Hello, Benjamin!
```

> **Note**: We also have a web-based, plug-in tester called the [Extism Playground](https://playground.extism.org/)

### More About Exports

Adding the [plugin_fn](https://docs.rs/extism-pdk/latest/extism_pdk/attr.plugin_fn.html) macro to your function does a couple things. It exposes your function as an export and it handles some of the lower level ABI details that allow you to declare your Wasm function as if it were a normal rust function. Here are a few examples of exports you can define.

### Primitive Types

A common thing you may want to do is pass some primitive rust data back and forth.
The [plugin_fn](https://docs.rs/extism-pdk/latest/extism_pdk/attr.plugin_fn.html) macro can map these types for you:

> **Note**: The [plugin_fn](https://docs.rs/extism-pdk/latest/extism_pdk/attr.plugin_fn.html) macro uses the [convert crate](https://github.com/extism/extism/tree/main/convert) to automatically convert and pass types across the guest / host boundary.

```rust
// f32 and f64
#[plugin_fn]
pub fn add_pi(input: f32) -> FnResult<f64> {
    Ok(input as f64 + 3.14f64)
}

// i32, i64, u32, u64
#[plugin_fn]
pub fn sum_42(input: i32) -> FnResult<i64> {
    Ok(input as i64 + 42i64)
}

// u8 vec
#[plugin_fn]
pub fn process_bytes(input: Vec<u8>) -> FnResult<Vec<u8>> {
    // process bytes here
    Ok(input)
}

```

### Json

We provide a [Json](https://docs.rs/extism-pdk/latest/extism_pdk/struct.Json.html) type that allows you to pass structs
that implement serde::Deserialize as parameters and serde::Serialize
as returns:

```rust
#[derive(serde::Deserialize)]
struct Add {
    a: u32,
    b: u32,
}
#[derive(serde::Serialize)]
struct Sum {
    sum: u32,
}

#[plugin_fn]
pub fn add(Json(add): Json<Add>) -> FnResult<Json<Sum>> {
    let sum = Sum { sum: add.a + add.b };
    Ok(Json(sum))
}
```

### Raw Export Interface

[plugin_fn](https://docs.rs/extism-pdk/latest/extism_pdk/attr.plugin_fn.html) is a nice macro abstraction but there may be times where you want more control. You can code directly to the raw ABI interface of export functions.


```rust
#[no_mangle]
pub unsafe extern "C" fn greet() -> i32 {
    let name = unwrap!(input::<String>());
    let result = format!("Hello, {}!", name);
    unwrap!(output(result));
    0i32
}

## Configs

Configs are key-value pairs that can be passed in by the host when creating a
plug-in. These can be useful to statically configure the plug-in with some data that exists across every function call. Here is a trivial example:

```rust
#[plugin_fn]
pub fn greet() -> FnResult<String> {
    let user = config::get("user").expect("'user' key set in config");
    Ok(format!("Hello, {}!", user))
}
```

To test it, the [Extism CLI](https://github.com/extism/cli) has a `--config` option that lets you pass in `key=value` pairs:

```bash
extism call my_plugin.wasm greet --config user=Benjamin
# => Hello, Benjamin!
```

## Variables

Variables are another key-value mechanism but it's a mutable data store that
will persist across function calls. These variables will persist as long as the
host has loaded and not freed the plug-in. You can use [var::get](https://docs.rs/extism-pdk/latest/extism_pdk/var/fn.get.html) and [var::set](https://docs.rs/extism-pdk/latest/extism_pdk/var/fn.set.html) to manipulate them.

```rust
#[plugin_fn]
pub fn count() -> FnResult<i64> {
    let mut c = var::get("count")?.unwrap_or(0);
    c = c + 1;
    var::set("count", c)?;
    Ok(c)
}
```

## Logging

Because Wasm modules by default do not have access to the system, printing to stdout won't work (unless you use WASI). Extism provides some simple logging macros that allow you to use the host application to log without having to give the plug-in permission to make syscalls. The primary one is [log!](https://docs.rs/extism-pdk/latest/extism_pdk/macro.log.html) but we also have some convenience macros named by log level:

```rust
#[plugin_fn]
pub fn log_stuff() -> FnResult<()> {
    log!(LogLevel::Info, "Some info!");
    log!(LogLevel::Warn, "A warning!");
    log!(LogLevel::Error, "An error!");

    // optionally you can use the leveled macros: 
    info!("Some info!");
    warn!("A warning!");
    error!("An error!");

    Ok(())
}
```

From [Extism CLI](https://github.com/extism/cli):

```bash
extism call my_plugin.wasm log_stuff --log-level=info
2023/09/30 11:52:17 Some info!
2023/09/30 11:52:17 A warning!
2023/09/30 11:52:17 An error!
```

> *Note*: From the CLI you need to pass a level with `--log-level`. If you are running the plug-in in your own host using one of our SDKs, you need to make sure that you call `set_log_file` to `"stdout"` or some file location.

## HTTP

Sometimes it is useful to let a plug-in make HTTP calls.

> **Note**: See [HttpRequest](https://docs.rs/extism-pdk/latest/extism_pdk/struct.HttpRequest.html) docs for more info on the request and response types:

> TODO: let's show a meatier example here:

```rust
#[plugin_fn]
pub fn http_get(Json(req): Json<HttpRequest>) -> FnResult<HttpResponse> {
    info!("Request to: {}", req.url);
    let res = http::request::<()>(&req, None)?;
    Ok(res)
}
```

## Imports (Host Functions)

Like any other code module, Wasm not only let's you export functions to the outside world, you can
import them too. Host Functions allow a plug-in to import functions defined in the host. For example,
if you host application is written in python, it can pass a python function down to your rust plug-in
where you can invoke it.

This topic can get fairly complicated and we have not yet fully abstracted the Wasm knowledge you need
to do this correctly. So we recommend reading out [concept doc on Host Functions](https://extism.org/docs/concepts/host-functions) before you get started.

### A Simple Example

Host functions have a similar interface as exports. You just need to declare them as `extern` on the top of your `lib.rs`. You only declare the interface as it is the host's responsibility to provide the implementation:

```rust
#[host_fn]
extern "ExtismHost" {
    fn a_python_func(input: String) -> String; 
}
```

> **Note**: The types we accept here are the same as the exports as the interface also uses the [convert crate](https://docs.rs/extism-convert/latest/extism_convert/).

To call this function, we must use the `unsafe` keyword. Also note that it automatically wraps the
function return with a Result in case the call fails.


```rust
#[plugin_fn]
pub fn hello_from_python(_: ()) -> FnResult<String> {
    let output = unsafe { a_python_func("An argument to send to Python".into())? };
    Ok(output)
}
```

### Testing it out

We can't really test this from the Extism CLI as something must provide the implementation. So let's
write out the python side here. You should check the docs for the language you're using on the host side
for how to implement host functions.

> TODO: update this after we publish and hone new python-sdk

```python
from extism import host_fn, Function, ValType

@host_fn
def a_python_func(plugin, input_, output, _user_data):
    # The plug-in is passing us a string
    input_str = plugin.input_string(input_[0])

    # just printing this out to prove we're in python land
    print("Hello from python!")

    # let's just add "!" to the input string
    # but you could imagine here we could add some
    # applicaiton code like query or manipulate the database
    # or our application APIs
    input_str += "!"

    # set the new string as the return value to the plug-in
    plugin.return_string(output[0], input_str)
```

Now when we load the plug-in we pass the host function:

> TODO: we should have an explanation in the host function concept doc that we can link to here
> that tells people how to determine their host function signature

> TODO: test this code or come up with a simpler runnable example. Maybe there could be something in extism CLI baked in you can use to test host functions or some python script you can use.
 
```python
functions = [
    Function(
        "a_python_func",
        [ValType.I64],
        [ValType.I64],
        a_python_func,
    )
]

plugin = Plugin(manifest, functions=functions)
result = plugin.call('hello_from_python')
print(result)
```

```bash
python3 app.py
# => Hello from Python!
# => An argument to send to Python!
```





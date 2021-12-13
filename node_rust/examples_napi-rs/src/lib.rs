#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;

use napi::{CallContext, JsNumber, JsObject, Result};

#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("sync", sync_fn)?;
    Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    ctx.env.create_uint32(argument + 100)
}

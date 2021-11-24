#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::convert::TryInto;

use napi::{CallContext, Env, JsNumber, JsObject, JsString, Result, Task};

struct AsyncTask(u32);

impl Task for AsyncTask {
    type Output = u32;
    type JsValue = JsNumber;

    fn compute(&mut self) -> Result<Self::Output> {
        use std::thread::sleep;
        use std::time::Duration;
        sleep(Duration::from_millis(self.0 as u64));
        Ok(self.0 * 2)
    }

    fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_uint32(output)
    }
}

///Primero tiempo de computo, segundo string a devolver
struct AmliAsyncTask(i32, &'static str);

impl Task for AmliAsyncTask {
    type Output = &'static str;
    type JsValue = JsString;

    /// Code computed inside a libuv thread
    fn compute(&mut self) -> Result<Self::Output> {
        use napi::{Error as Napi_error, Status};
        use std::thread::sleep;
        use std::time::Duration;

        sleep(Duration::from_millis(self.0 as u64));

        let will_return_error = match self.1 {
            "rs rocks" => true,
            _ => false,
        };
        match will_return_error {
            true => Err(Napi_error::new(
                Status::InvalidArg,
                String::from("\"rs rocks\" is an invalid argument"),
            )),
            false => Ok(self.1),
        }
    }

    /// This function is called if compute return Ok
    fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
        env.create_string(output)
    }

    /// This function is called if compute return Err
    fn reject(self, _env: Env, err: napi::Error) -> Result<Self::JsValue> {
        Err(err)
    }
}

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
    exports.create_named_method("sync", sync_fn)?;

    exports.create_named_method("sleep", sleep)?;

    exports.create_named_method("amliPromise", amli_promise)?;

    Ok(())
}

#[js_function(1)]
fn sync_fn(ctx: CallContext) -> Result<JsNumber> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    ctx.env.create_uint32(argument + 100)
}

#[js_function(1)]
fn sleep(ctx: CallContext) -> Result<JsObject> {
    let argument: u32 = ctx.get::<JsNumber>(0)?.try_into()?;
    let task = AsyncTask(argument);
    let async_task = ctx.env.spawn(task)?;
    Ok(async_task.promise_object())
}

#[js_function(2)]
fn amli_promise(ctx: CallContext) -> Result<JsObject> {
    let arg0: i32 = ctx.get::<JsNumber>(0)?.get_int32()?;
    let arg1 = ctx.get::<JsString>(1)?.into_utf8()?.into_owned()?;
    // Box::leak is used to improve performance, leaks the content of arg1 returning a 'static str
    // This operation is faster than cloning the content of arg1
    let task = AmliAsyncTask(arg0, Box::leak(arg1.into_boxed_str()));
    let async_task = ctx.env.spawn(task)?;
    Ok(async_task.promise_object())
}

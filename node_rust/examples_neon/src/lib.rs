use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn hi_to_charli(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hola charli como estamos, desde rust"))
}

fn hi_to_isma(mut cx: FunctionContext) -> JsResult<JsString> {
    let param1 = cx.argument::<JsString>(0)?;
    // let param2 = cx.argument::<JsString>(2)?;
    println!("{}", param1.value(&mut cx));
    Ok(param1)
}

fn sum(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let param1 = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let param2 = cx.argument::<JsNumber>(1)?.value(&mut cx);

    let suma = param1 + param2;

    std::thread::spawn(move || {
        println!("{}", suma);
    });

    // let param2 = cx.argument::<JsString>(2)?;
    // println!("{}", param1.value(&mut cx));
    Ok(cx.number(suma))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("hiToCharli", hi_to_charli)?;
    cx.export_function("", hi_to_isma)?;
    cx.export_function("sum", sum)?;

    Ok(())
}

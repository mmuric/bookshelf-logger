use neon::prelude::*;

// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     Ok(cx.string("hello node"))
// }

fn print_log(mut cx: FunctionContext, uuid: JsString) -> JsResult<JsString> {
    OK(cx.string("Hello" + uuid))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // cx.export_function("hello", hello)?;
    cx.export_function("print_log", print_log)?;
    Ok(())
}



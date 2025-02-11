use chrono::prelude::*;
use neon::prelude::*;
use pyo3::prelude::*;

pub fn print_log_entry(uuid: &str, msrv: &str, comment: &str, payload: &serde_json::Value) {
    let current_time = Local::now();
    let formatted_time = current_time.format("%Y-%m-%d %H:%M").to_string();

    println!(
        "[\x1b[1m{}\x1b[0m]: \x1b[0;32m{}\x1b[0m (\x1b[1;34m{}\x1b[0m) - {} => {}",
        formatted_time, uuid, msrv, comment, payload
    );
}

pub fn print_log_entry_js(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let uuid = cx.argument::<JsString>(0)?.value(&mut cx);
    let msrv = cx.argument::<JsString>(1)?.value(&mut cx);
    let comment = cx.argument::<JsString>(2)?.value(&mut cx);
    let payload = cx
        .argument::<JsValue>(3)?
        .to_string(&mut cx)?
        .value(&mut cx);
    print_log_entry(
        &uuid,
        &msrv,
        &comment,
        &payload.parse::<serde_json::Value>().unwrap(),
    );
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("print_log_entry", print_log_entry_js)?;
    Ok(())
}


#[pyfunction]
#[pyo3(name="print_log_entry")]
fn print_log_entry_py(uuid: &str, msrv: &str, comment: &str, payload: &str) -> PyResult<()> {
    print_log_entry(
        uuid,
        msrv,
        comment,
        &payload.parse::<serde_json::Value>().unwrap(),
    );
    Ok(())
}

#[pymodule]
#[pyo3(name="bookshelflogger")]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_log_entry_py, m)?)?;
    Ok(())
}

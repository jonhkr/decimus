use std::str::FromStr;

use neon::prelude::*;
use neon::register_module;
use rust_decimal::Decimal;

fn new(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_str = cx.argument::<JsString>(0)?;
    match Decimal::from_str(js_str.value().as_str()) {
        Ok(d) => Ok(to_js_array(&mut cx, d)),
        Err(e) => cx.throw_error(e.to_string())
    }
}

fn to_string(mut cx: FunctionContext) -> JsResult<JsString> {
    let arr = cx.argument::<JsArray>(0)?;
    let d1 = from_js_array(&mut cx, arr)?;
    Ok(JsString::new(&mut cx, d1.to_string()))
}

fn add(mut cx: FunctionContext) -> JsResult<JsArray> {
    let arr = cx.argument::<JsArray>(0)?;

    let d1 = from_js_array(&mut cx, arr)?;

    if let Some(d2) = d1.checked_add(Decimal::new(10, 0)) {
        let result = to_js_array(&mut cx, d2);

        Ok(result)
    } else {
        return cx.throw_error("failed to add");
    }
}

fn from_js_array<'a, C: Context<'a>>(cx: &mut C, arr: Handle<JsArray>) -> NeonResult<Decimal> {
    if arr.len() != 16 {
        return cx.throw_error("not a valid decimal");
    }

    let mut bytes: [u8; 16] = [0; 16];

    let mut i: usize = 0;
    loop {
        if i >= 16 {
            break;
        }
        bytes[i] = arr.get(cx, i as u32)?
            .downcast::<JsNumber>()
            .or_throw(cx)?
            .value() as u8;

        i += 1;
    }

    Ok(Decimal::deserialize(bytes))
}

fn to_js_array<'a, C: Context<'a>>(cx: &mut C, decimal: Decimal) -> Handle<'a, JsArray> {
    let result = JsArray::new(cx, 16);

    for (i, byte) in decimal.serialize().iter().enumerate() {
        let js_n = cx.number(*byte as f64);
        result.set(cx, i as u32, js_n).unwrap();
    }

    result
}

register_module!(mut m, {
    m.export_function("new", new)?;
    m.export_function("add", add)?;
    m.export_function("toString", to_string)?;
    Ok(())
});

use std::str::FromStr;

use neon::prelude::*;
use neon::register_module;
use rust_decimal::{Decimal, RoundingStrategy};

fn new(mut cx: FunctionContext) -> JsResult<JsArray> {
    let js_str = cx.argument::<JsString>(0)?;
    match Decimal::from_str(js_str.value().as_str()) {
        Ok(d) => Ok(to_js_array(&mut cx, d)),
        Err(e) => cx.throw_error(e.to_string())
    }
}

fn str(mut cx: FunctionContext) -> JsResult<JsString> {
    let arr = cx.argument::<JsArray>(0)?;
    let d1 = from_js_array(&mut cx, arr)?;
    Ok(JsString::new(&mut cx, d1.to_string()))
}

fn op2(mut cx: FunctionContext, op: fn(d1: Decimal, d2: Decimal) -> Option<Decimal>) -> JsResult<JsArray> {
    let arg1 = cx.argument::<JsArray>(0)?;
    let arg2 = cx.argument::<JsArray>(1)?;

    let d1 = from_js_array(&mut cx, arg1)?;
    let d2 = from_js_array(&mut cx, arg2)?;

    if let Some(d3) = op(d1, d2) {
        let result = to_js_array(&mut cx, d3);

        Ok(result)
    } else {
        return cx.throw_error("failed to add");
    }
}


fn add(cx: FunctionContext) -> JsResult<JsArray> {
    return op2(cx, Decimal::checked_add);
}

fn sub(cx: FunctionContext) -> JsResult<JsArray> {
    return op2(cx, Decimal::checked_sub);
}

fn mul(cx: FunctionContext) -> JsResult<JsArray> {
    return op2(cx, Decimal::checked_mul);
}

fn round(mut cx: FunctionContext) -> JsResult<JsArray> {
    let arg1 = cx.argument::<JsArray>(0)?;
    let arg2 = cx.argument::<JsNumber>(1)?;
    let arg3 = cx.argument::<JsNumber>(2)?;

    let d1 = from_js_array(&mut cx, arg1)?;
    let dp = arg2.value() as u32;
    let s = arg3.value() as u8;

    let strategy = match s {
        0 => RoundingStrategy::BankersRounding,
        1 => RoundingStrategy::RoundHalfUp,
        2 => RoundingStrategy::RoundHalfDown,
        _ => panic!("invalid rounding strategy")
    };

    let rounded = d1.round_dp_with_strategy(dp, strategy);
    let result = to_js_array(&mut cx, rounded);

    Ok(result)
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
    m.export_function("sub", sub)?;
    m.export_function("mul", mul)?;
    m.export_function("round", round)?;
    m.export_function("str", str)?;
    Ok(())
});

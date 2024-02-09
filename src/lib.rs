use neon::prelude::*;
mod types;
mod io;
use io::*;

fn read(mut cx: FunctionContext) -> JsResult<JsPromise> {

    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let file = cx.argument::<JsString>(0)?.value(&mut cx);
    let format = cx.argument::<JsBoolean>(1)?.value(&mut cx);

    let promise = cx.task(move || read_all(file, format)).promise(move |mut cx, map| {
        match map {
            Ok(map) => {
                let result = cx.empty_object();

                let mut keys: Vec<String> = map.keys().cloned().collect();
                keys.sort();

                for key in keys {

                    let prop_key = cx.string(&key);
                    let prop_value = cx.string(map.get(&key).unwrap());

                    result.set(&mut cx, prop_key, prop_value)?;
                }

                Ok(result)
            },
            Err(e) => cx.throw_error(e.message().to_string()),
        }
    });

    Ok(promise)
}

fn get_value(mut cx: FunctionContext) -> JsResult<JsPromise> {

    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let file = cx.argument::<JsString>(0)?.value(&mut cx);
    let files = vec![file];
    let prop_name = cx.argument::<JsString>(1)?.value(&mut cx);

    let promise = cx.task(move || read_values(files, prop_name)).promise(move |mut cx, values| {
        match values {
            Ok(values) => Ok(cx.string(values.values().nth(0).unwrap_or(&"".to_string()))),
            Err(e) => cx.throw_error(e.message().to_string()),
        }
    });

    Ok(promise)

}

fn get_values(mut cx: FunctionContext) -> JsResult<JsPromise> {

    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let mut files = Vec::new();
    let array = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;

    for file in array {
        let full_path = file.to_string(&mut cx)?.value(&mut cx);
        files.push(full_path);
    }

    let prop_name = cx.argument::<JsString>(1)?.value(&mut cx);

    let promise = cx.task(move || read_values(files, prop_name)).promise(move |mut cx, values| {
        match values {
            Ok(values) => {
                let result = cx.empty_object();
                for (key, value) in values {
                    let path = cx.string(key);
                    let comment = cx.string(value);
                    result.set(&mut cx, path, comment)?;
                }
                Ok(result)
            },
            Err(e) => cx.throw_error(e.message().to_string()),
        }
    });

    Ok(promise)

}

fn set_value(mut cx: FunctionContext) -> JsResult<JsPromise> {

    if cx.len() != 3 {
        return cx.throw_error("Invalid number of arguments");
    }

    let file = cx.argument::<JsString>(0)?.value(&mut cx);
    let key = cx.argument::<JsString>(1)?.value(&mut cx);
    let value = cx.argument::<JsString>(2)?.value(&mut cx);

    let promise = cx.task(move || write_value(file, key, value)).promise(move |mut cx, result| {
        match result {
            Ok(result) => Ok(cx.boolean(result)),
            Err(e) => cx.throw_error(e.message().to_string()),
        }
    });

    Ok(promise)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("read", read)?;
    cx.export_function("getValue", get_value)?;
    cx.export_function("getValues", get_values)?;
    cx.export_function("setValue", set_value)?;
    Ok(())
}

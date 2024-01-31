use neon::prelude::*;
use std::mem::ManuallyDrop;
use std::collections::HashMap;
use windows::{
    core::{PWSTR, HSTRING},
    Win32::{
        System::Variant::VT_LPWSTR,
        System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED,
            StructuredStorage::{
                PropVariantGetElementCount,
                PropVariantGetStringElem,
                PropVariantClear,
                PROPVARIANT_0_0,
                PROPVARIANT,
                PROPVARIANT_0,
                PROPVARIANT_0_0_0,
            }
        },
        UI::Shell::PropertiesSystem::{
            IPropertyStore, SHGetPropertyStoreFromParsingName, GPS_READWRITE,
        },
        Storage::EnhancedStorage::PKEY_Comment
    },
};

fn get_comments(mut cx: FunctionContext) -> JsResult<JsObject> {

    let array = cx.argument::<JsArray>(0)?.to_vec(&mut cx).unwrap();
    let mut files = Vec::new();
    for file in array{
        let x = file.to_string(&mut cx).unwrap().value(&mut cx);
        files.push(x);
    }

    let map = read_metadata(files).unwrap();

    let result = cx.empty_object();

    cx.execute_scoped(|mut cx| {

        for (key, value) in map {
            let path = cx.string(key);
            let comment = cx.string(value);
            result.set(&mut cx, path, comment).unwrap();
        }

    });

    Ok(result)

}

fn set_comment(mut cx: FunctionContext) -> JsResult<JsBoolean> {

    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let file = cx.argument::<JsString>(0).unwrap().value(&mut cx);
    let comment = cx.argument::<JsString>(1).unwrap().value(&mut cx);

    let result = write_metadata(file, comment);

    Ok(cx.boolean(result.unwrap()))
}


fn get_store(path:&String) -> windows::core::Result<IPropertyStore> {

    unsafe{
        let store:IPropertyStore = SHGetPropertyStoreFromParsingName(
            &HSTRING::from(path),
            None,
            GPS_READWRITE
        )?;

        Ok(store)
    }
}

fn read_metadata(files:Vec<String>) -> windows::core::Result<HashMap<String, String>> {

    unsafe {

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let mut result = HashMap::new();

        for file in files{

            let store = get_store(&file)?;

            let mut prop_value = store.GetValue(&PKEY_Comment)?;

            let prop_count = PropVariantGetElementCount(&prop_value);
            if prop_count == 1 {

                let string_elm = PropVariantGetStringElem(&prop_value, 0);
                PropVariantClear(&mut prop_value)?;
                let comment = string_elm.unwrap().to_string().unwrap();

                result.insert(file, comment);

            }

        }

        Ok(result)
    }

}

fn write_metadata(file:String, comment:String) -> windows::core::Result<bool> {

    unsafe {

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let store = get_store(&file).unwrap();

        let mut nstr = comment.to_owned();
        nstr.push_str("\0");
        let mut str: Vec<u16> = nstr.encode_utf16().collect();

        let value = PROPVARIANT_0_0 {
            vt: VT_LPWSTR,
            Anonymous: PROPVARIANT_0_0_0 {
                pwszVal: PWSTR(str.as_mut_ptr()),
            },
            ..Default::default()
        };

        let prop = PROPVARIANT {
            Anonymous: PROPVARIANT_0 {
                Anonymous: ManuallyDrop::new(value),
            },
        };

        store.SetValue(&PKEY_Comment, &prop)?;
        store.Commit()?;

        //PropVariantClear(&mut prop)?;

    }

    Ok(true)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getComments", get_comments)?;
    cx.export_function("setComment", set_comment)?;
    Ok(())
}

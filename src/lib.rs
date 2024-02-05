use neon::prelude::*;
use std::mem::ManuallyDrop;
use std::path::Path;
use std::collections::HashMap;
use windows::{
    core::{ComInterface, Interface, BSTR, GUID, HSTRING, PCWSTR, PWSTR},
    Win32::{
        System::Variant::{
            VARIANT,
            VARIANT_0_0,
            VT_BSTR,
            VT_LPWSTR,
            VT_DISPATCH,
            VariantClear,
        },
        System::Com::{
            CoInitializeEx,
            CoUninitialize,
            CoCreateInstance,
            COINIT_APARTMENTTHREADED,
            CLSCTX_INPROC_SERVER,
            StructuredStorage::{
                PropVariantClear,
                PROPVARIANT_0_0,
                PROPVARIANT,
            }
        },
        UI::Shell::{
            IShellDispatch,
            FolderItem,
            PropertiesSystem::{
                IPropertyStore,
                PROPERTYKEY,
                SHGetPropertyStoreFromParsingName,
                PSGetPropertyKeyFromName,
                PSGetNameFromPropertyKey,
                PSFormatForDisplayAlloc,
                GPS_NO_OPLOCK,
                GPS_READWRITE,
                PDFF_DEFAULT,
            },
        },
        Storage::EnhancedStorage::PKEY_Comment
    },
};

const CLSID_SHELL: GUID = GUID {
    data1: 0x13709620,
    data2: 0xC279,
    data3: 0x11CE,
    data4: [0xA4, 0x9E, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
};

/* -------------- types -------------------- */
struct Variant(VARIANT);

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe {
            let _ = VariantClear(&mut self.0);
        }
    }
}

trait ToVariant {
    fn from_str(s: &str) -> VARIANT;
    fn from_item(item:&FolderItem) -> windows::core::Result<VARIANT>;
}

impl ToVariant for VARIANT {
    fn from_str(s: &str) -> VARIANT {
        let mut variant = VARIANT::default();
        let mut v00 = VARIANT_0_0::default();
        v00.vt = VT_BSTR;
        let bstr = BSTR::from(s);
        v00.Anonymous.bstrVal = ManuallyDrop::new(bstr);
        variant.Anonymous.Anonymous = ManuallyDrop::new(v00);
        variant
    }

    fn from_item(item:&FolderItem) -> windows::core::Result<VARIANT> {
        let mut variant = VARIANT::default();
        let mut v00 = VARIANT_0_0::default();
        v00.vt = VT_DISPATCH;
        v00.Anonymous.pdispVal = ManuallyDrop::new(Some(item.cast()?));
        variant.Anonymous.Anonymous = ManuallyDrop::new(v00);
        Ok(variant)
    }
}

struct PropVariant(PROPVARIANT);

impl Drop for PropVariant {
    fn drop(&mut self) {
        unsafe {
            let _ = PropVariantClear(&mut self.0);
        }
    }
}

trait ToPropVariant {
    fn from_str(s:&str) -> PROPVARIANT;
}

impl ToPropVariant for PROPVARIANT {
    fn from_str(s:&str) -> PROPVARIANT {
        let mut variant = PROPVARIANT::default();
        let mut v00 = PROPVARIANT_0_0::default();
        v00.vt = VT_LPWSTR;
        let mut str: Vec<u16> = s.encode_utf16().chain([0u16]).collect();
        let pwstr = PWSTR(str.as_mut_ptr());
        v00.Anonymous.pwszVal = pwstr;
        variant.Anonymous.Anonymous = ManuallyDrop::new(v00);
        variant
    }
}

fn to_pcwstr(str:&String) -> PCWSTR {
    PCWSTR(HSTRING::from(str).as_ptr())
}

fn to_btsr(str:&str) -> windows::core::Result<BSTR> {
    let vec: Vec<u16> = str.encode_utf16().collect();
    let btsr = BSTR::from_wide(&vec)?;
    Ok(btsr)
}

/* ---------------------------------- */

fn read(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let file = cx.argument::<JsString>(0)?.value(&mut cx);

    let promise = cx.task(move || read_all(file)).promise(move |mut cx, map| {
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

fn get_comments(mut cx: FunctionContext) -> JsResult<JsPromise> {

    let array = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
    let mut files = Vec::new();
    for file in array{
        let full_path = file.to_string(&mut cx)?.value(&mut cx);
        files.push(full_path);
    }

    let promise = cx.task(move || read_comments(files)).promise(move |mut cx, map| {
        match map {
            Ok(map) => {
                let result = cx.empty_object();
                for (key, value) in map {
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

fn set_comment(mut cx: FunctionContext) -> JsResult<JsPromise> {

    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let file = cx.argument::<JsString>(0)?.value(&mut cx);
    let comment = cx.argument::<JsString>(1)?.value(&mut cx);

    let promise = cx.task(move || write_metadata(file, comment)).promise(move |mut cx, result| {
        match result {
            Ok(result) => Ok(cx.boolean(result)),
            Err(e) => cx.throw_error(e.message().to_string()),
        }
    });

    Ok(promise)
}

/*--------- private functions ------------ */

fn get_store(path:&String, write:bool) -> windows::core::Result<IPropertyStore> {

    let p_path = to_pcwstr(path);

    unsafe{
        let store:IPropertyStore = SHGetPropertyStoreFromParsingName(
            p_path,
            None,
            if write { GPS_READWRITE } else { GPS_NO_OPLOCK }
        )?;

        Ok(store)
    }
}

#[allow(dead_code)]
fn get_propertykey(name:String) -> windows::core::Result<PROPERTYKEY> {

    let mut propkey:PROPERTYKEY = PROPERTYKEY::default();

    unsafe{
        let pstr = to_pcwstr(&name);

        PSGetPropertyKeyFromName(pstr, &mut propkey)?;

        Ok(propkey)
    }

}

fn read_all(file:String) -> windows::core::Result<HashMap<String,String>> {

    let mut result = HashMap::new();

    unsafe {

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let store = get_store(&file, false)?;

        let count = store.GetCount()?;

        for i in 0..count {

            let mut propkey = PROPERTYKEY::default();
            if store.GetAt(i, &mut propkey).is_ok() {

                match store.GetValue(&mut propkey) {

                    Ok(propvalue) => {

                        match PSGetNameFromPropertyKey(&propkey) {
                            Ok(keyname) => {
                                let value = PSFormatForDisplayAlloc(&propkey, &propvalue, PDFF_DEFAULT)?;
                                result.insert(keyname.to_string()?, value.to_string()?);
                            }
                            Err(_) => (),
                        };

                    },
                    Err(_) => (),
                }

            }
        }

        store.into_raw();

        CoUninitialize();

    }

    Ok(result)
}

fn read_comments(files:Vec<String>) -> windows::core::Result<HashMap<String, String>> {

    let mut result = HashMap::new();

    let directory_name = Path::new(&files[0]).parent().unwrap().to_str().unwrap();
    let v_dir = VARIANT::from_str(directory_name);

    unsafe {

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let shell_dispatch:IShellDispatch = CoCreateInstance(
            &CLSID_SHELL,
            None,
            CLSCTX_INPROC_SERVER
        )?;

        let folder = shell_dispatch.NameSpace(v_dir)?;

        for file in files {
            let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
            let name = to_btsr(&file_name)?;
            let folder_item = folder.ParseName(&name)?;
            let v_item = VARIANT::from_item(&folder_item)?;
            let comment = folder.GetDetailsOf(v_item, 24)?;
            result.insert(file, comment.to_string());
        }

        CoUninitialize();

    }

    Ok(result)

}

fn write_metadata(file:String, comment:String) -> windows::core::Result<bool> {

    unsafe {

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let store = get_store(&file, true)?;

        let prop = PROPVARIANT::from_str(&comment);

        store.SetValue(&PKEY_Comment, &prop)?;
        store.Commit()?;

        store.into_raw();

        CoUninitialize();

    }

    Ok(true)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("read", read)?;
    cx.export_function("getComments", get_comments)?;
    cx.export_function("setComment", set_comment)?;
    Ok(())
}

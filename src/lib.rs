use neon::prelude::*;
use std::mem::ManuallyDrop;
use std::path::Path;
use std::collections::HashMap;
use windows::{
    core::{ComInterface, BSTR, GUID, HSTRING, PWSTR},
    Win32::{
        System::Variant::{
            VARIANT,
            VARIANT_0_0,
            VT_BSTR,
            VT_LPWSTR,
            VT_DISPATCH,
            VariantClear
        },
        System::Com::{
            CoInitializeEx,
            CoUninitialize,
            CoCreateInstance,
            COINIT_APARTMENTTHREADED,
            CLSCTX_INPROC_SERVER,
            StructuredStorage::{
                PROPVARIANT_0_0,
                PROPVARIANT,
                PROPVARIANT_0,
                PROPVARIANT_0_0_0,
            }
        },
        UI::Shell::{
            IShellDispatch,
            FolderItem,
            PropertiesSystem::{
                IPropertyStore,
                SHGetPropertyStoreFromParsingName,
                GPS_NO_OPLOCK,
                GPS_READWRITE,
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

fn to_btsr(str:&str) -> windows::core::Result<BSTR> {
    let vec: Vec<u16> = str.encode_utf16().collect();
    let btsr = BSTR::from_wide(&vec)?;
    Ok(btsr)
}

fn get_comments(mut cx: FunctionContext) -> JsResult<JsPromise> {

    let (deferred, promise) = cx.promise();

    let array = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
    let mut files = Vec::new();
    for file in array{
        let full_path = file.to_string(&mut cx)?.value(&mut cx);
        files.push(full_path);
    }

    let map = match read_metadata(files) {
        Ok(map) => map,
        Err(error) => return cx.throw_error(error.message().to_string()),
    };

    let result = cx.empty_object();

    cx.execute_scoped(|mut cx| {

        for (key, value) in map {
            let path = cx.string(key);
            let comment = cx.string(value);
            result.set(&mut cx, path, comment).unwrap();
        }

    });

    deferred.resolve(&mut cx, result);

    Ok(promise)

}

fn set_comment(mut cx: FunctionContext) -> JsResult<JsPromise> {

    if cx.len() != 2 {
        return cx.throw_error("Invalid number of arguments");
    }

    let (deferred, promise) = cx.promise();

    let file = cx.argument::<JsString>(0)?.value(&mut cx);
    let comment = cx.argument::<JsString>(1)?.value(&mut cx);

    let result = match write_metadata(file, comment) {
        Ok(result) => result,
        Err(error) => return cx.throw_error(error.message().to_string()),
    };

    let js_result = cx.boolean(result);

    deferred.resolve(&mut cx, js_result);

    Ok(promise)
}

fn read_metadata(files:Vec<String>) -> windows::core::Result<HashMap<String, String>> {

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

        let mut result = HashMap::new();

        for file in files {
            let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
            let name = to_btsr(&file_name)?;
            let folder_item =folder.ParseName(&name)?;
            let v_item = VARIANT::from_item(&folder_item)?;
            let comment = folder.GetDetailsOf(v_item, 24)?;
            result.insert(file, comment.to_string());
        }

        CoUninitialize();

        Ok(result)
    }

}

fn get_store(path:&String, write:bool) -> windows::core::Result<IPropertyStore> {

    unsafe{
        let store:IPropertyStore = SHGetPropertyStoreFromParsingName(
            &HSTRING::from(path),
            None,
            if write { GPS_READWRITE } else { GPS_NO_OPLOCK }
        )?;

        Ok(store)
    }
}

fn write_metadata(file:String, comment:String) -> windows::core::Result<bool> {

    unsafe {

        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let store = get_store(&file, true)?;

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

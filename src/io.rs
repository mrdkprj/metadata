use std::collections::HashMap;
use std::path::Path;
use windows::{
    core::{ComInterface, GUID},
    Win32::{
        System::{
            Com::{
                CoCreateInstance, CoInitializeEx, CoUninitialize, StructuredStorage::PROPVARIANT,
                CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED,
            },
            Variant::VARIANT,
        },
        UI::Shell::{
            FolderItem2, IShellDispatch,
            PropertiesSystem::{
                IPropertyStore, PSFormatForDisplayAlloc, PSGetNameFromPropertyKey,
                PSGetPropertyKeyFromName, SHGetPropertyStoreFromParsingName, GPS_DEFAULT,
                GPS_READWRITE, PDFF_DEFAULT, PROPERTYKEY,
            },
        },
    },
};

use crate::types::*;

const CLSID_SHELL: GUID = GUID {
    data1: 0x13709620,
    data2: 0xC279,
    data3: 0x11CE,
    data4: [0xA4, 0x9E, 0x44, 0x45, 0x53, 0x54, 0x00, 0x00],
};

fn get_store(path: &String, write: bool) -> windows::core::Result<IPropertyStore> {
    let p_path = to_hstring(path);

    unsafe {
        let store: IPropertyStore = SHGetPropertyStoreFromParsingName(
            &p_path,
            None,
            if write { GPS_READWRITE } else { GPS_DEFAULT },
        )?;

        Ok(store)
    }
}

fn get_propertykey(name: &String) -> windows::core::Result<PROPERTYKEY> {
    let mut propkey: PROPERTYKEY = PROPERTYKEY::default();

    unsafe {
        let pstr = to_hstring(name);

        PSGetPropertyKeyFromName(&pstr, &mut propkey)?;

        Ok(propkey)
    }
}

pub fn read_all(file: String, format: bool) -> windows::core::Result<HashMap<String, String>> {
    let mut result = HashMap::new();

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let store = get_store(&file, false)?;

        let count = store.GetCount()?;

        for i in 0..count {
            let mut propkey = PROPERTYKEY::default();

            if store.GetAt(i, &mut propkey).is_ok() {
                if let Ok(propvalue) = store.GetValue(&propkey) {
                    if let Ok(keyname) = PSGetNameFromPropertyKey(&propkey) {
                        let key = keyname.to_string()?.replace("System", "").replace('.', "");
                        let value = if format {
                            PSFormatForDisplayAlloc(&propkey, &propvalue, PDFF_DEFAULT)?
                                .to_string()?
                        } else {
                            propvalue.to_string()?
                        };
                        result.insert(key, value.to_string());
                    };
                }
            }
        }

        CoUninitialize();
    }

    Ok(result)
}

pub fn read_values(
    files: Vec<String>,
    prop_name: String,
) -> windows::core::Result<HashMap<String, String>> {
    let mut result = HashMap::new();

    let directory_name = Path::new(&files[0]).parent().unwrap().to_str().unwrap();
    let v_dir = VARIANT::from_str(directory_name);

    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let shell_dispatch: IShellDispatch =
            CoCreateInstance(&CLSID_SHELL, None, CLSCTX_INPROC_SERVER)?;

        let folder = shell_dispatch.NameSpace(v_dir)?;

        for file in files {
            let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
            let name = to_btsr(file_name)?;
            let folder_item = folder.ParseName(&name)?;
            let folder_item2 = folder_item.cast::<FolderItem2>()?;
            let b_prop_name = to_btsr(&prop_name)?;
            let v_comment = folder_item2.ExtendedProperty(&b_prop_name)?;
            let comment = VARIANT::to_string(&v_comment)?;

            result.insert(file, comment);
        }

        CoUninitialize();
    }

    Ok(result)
}

pub fn write_value(file: String, key: String, value: String) -> windows::core::Result<bool> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED)?;

        let store = get_store(&file, true)?;

        let prop_key = get_propertykey(&key)?;
        let prop_value = PROPVARIANT::from_str(&value);

        store.SetValue(&prop_key, &prop_value)?;
        store.Commit()?;

        CoUninitialize();
    }

    Ok(true)
}

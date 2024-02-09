use std::mem::ManuallyDrop;
use windows::{
    core::{ComInterface, BSTR, HSTRING, PCWSTR, PWSTR},
    Win32::{
        Foundation::VARIANT_TRUE, System::{
            Com::StructuredStorage::{
                PropVariantChangeType, PropVariantClear, PROPVARIANT, PROPVARIANT_0_0, PROPVAR_CHANGE_FLAGS
            },
            Variant::{
                VariantChangeType, VariantClear, VARIANT, VARIANT_0_0, VAR_CHANGE_FLAGS, VT_BOOL, VT_BSTR, VT_DISPATCH, VT_LPWSTR
            }
        }, UI::Shell::FolderItem
    },
};

pub struct Variant(VARIANT);

impl Drop for Variant {
    fn drop(&mut self) {
        unsafe {
            let _ = VariantClear(&mut self.0);
        }
    }
}

pub trait ToVariant {
    fn from_str(s: &str) -> VARIANT;
    fn from_item(item:&FolderItem) -> windows::core::Result<VARIANT>;
    fn to_string(&self) -> windows::core::Result<String>;
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

    fn to_string(&self) -> windows::core::Result<String> {
        unsafe {
            let mut variant = VARIANT::default();
            VariantChangeType(&mut variant, self, VAR_CHANGE_FLAGS(0), VT_BSTR)?;
            let v00 = &variant.Anonymous.Anonymous;
            let str = v00.Anonymous.bstrVal.to_string();
            VariantClear(&mut variant)?;
            Ok(str)
        }
    }
}

pub struct PropVariant(PROPVARIANT);

impl Drop for PropVariant {
    fn drop(&mut self) {
        unsafe {
            let _ = PropVariantClear(&mut self.0);
        }
    }
}

pub trait ToPropVariant {
    fn from_str(s:&str) -> PROPVARIANT;
    fn to_string(&self) -> windows::core::Result<String>;
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

    fn to_string(&self) -> windows::core::Result<String> {
        unsafe{
            match &self.Anonymous.Anonymous.vt {
                &VT_BOOL => {
                    let str = if &self.Anonymous.Anonymous.Anonymous.boolVal == &VARIANT_TRUE { "true" } else { "false"};
                    Ok(str.to_string())
                },
                _ => {
                    let mut variant = PROPVARIANT::default();
                    match PropVariantChangeType(&mut variant, self, PROPVAR_CHANGE_FLAGS(0), VT_BSTR) {
                        Ok(_) => {
                            let v00 = &variant.Anonymous.Anonymous;
                            let str = v00.Anonymous.bstrVal.to_string();
                            PropVariantClear(&mut variant)?;
                            Ok(str)
                        }
                        Err(_) => {
                            PropVariantClear(&mut variant)?;
                            Ok("N/A".to_string())
                        }
                    }
                }
            }
        }
    }
}

pub fn to_hstring(str:&String) -> HSTRING {
    HSTRING::from(str)
}

#[allow(dead_code)]
pub fn to_pcwstr(str:&String) -> PCWSTR {
    let hstr = HSTRING::from(str);
    PCWSTR::from_raw(hstr.as_ptr())
}

pub fn to_btsr(str:&str) -> windows::core::Result<BSTR> {
    let vec: Vec<u16> = str.encode_utf16().collect();
    let btsr = BSTR::from_wide(&vec)?;
    Ok(btsr)
}
extern crate objc;

use objc::runtime::{Class, Object};
use objc::declare::ClassDecl;

pub fn save_private_key_to_keychain(private_key: &str) {
    unsafe {
        let nsstring: *mut Object = msg_send![class!(NSString), stringWithUTF8String:private_key.as_ptr()];

        let keychain_class = Class::get("CryptoMasterTimeKeychainClass").unwrap();
        let shared_keychain: *mut Object = msg_send![keychain_class, defaultKeychain];

        let query_dict: *mut Object = msg_send![class!(NSMutableDictionary), new];
        let key_str: *mut Object = msg_send![class!(NSString), stringWithUTF8String:"privateKey"];
        let value_str: *mut Object = nsstring;

        let _: () = msg_send![query_dict, setObject:value_str forKey:key_str];

        let _: () = msg_send![shared_keychain, storeKeychainItem:query_dict];
    }
}

pub fn read_private_key_from_keychain() -> Option<String> {
    unsafe {
        let keychain_class = Class::get("CryptoMasterTimeKeychainClass").unwrap();
        let shared_keychain: *mut Object = msg_send![keychain_class, defaultKeychain];

        let query_dict: *mut Object = msg_send![class!(NSMutableDictionary), new];
        let key_str: *mut Object = msg_send![class!(NSString), stringWithUTF8String:"privateKey"];
        
        let result_dict: *mut Object = msg_send![shared_keychain, fetchKeychainItem:query_dict];

        let key_str_ptr: *mut Object = msg_send![result_dict, objectForKey:key_str];
        if key_str_ptr.is_null() {
            return None;
        }

        let key_str: *const u8 = msg_send![key_str_ptr, UTF8String];
        Some(String::from_utf8_lossy(std::ffi::CStr::from_ptr(key_str).to_bytes()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_read_private_key_to_keychain() {
        
        let private_key = "your_private_key";

        save_private_key_to_keychain(private_key);
       
        let retrieved_private_key = read_private_key_from_keychain().unwrap();
       
        assert_eq!(private_key, retrieved_private_key);
    }
}


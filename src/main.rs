extern crate libc;
use libc::c_char;
use libc::uint8_t;
use libc::uint16_t;
use libc::uint32_t;
use libc::size_t;

use std::ffi::CString;

extern crate rustc_serialize as serialize;
use serialize::hex::FromHex;
use serialize::hex::ToHex;

mod toxcore;
use toxcore::*;


#[link(name = "toxcore")]
extern { fn tox_version_major() -> libc::uint32_t ; }


extern "C" fn friend_request(_tox: *mut Tox, public_key: *const uint8_t, message: *const uint8_t, length: size_t, user_data: *mut ::std::os::raw::c_void)
{
    unsafe {
        let len = tox_address_size() as usize;
        let pubkey_slice = std::slice::from_raw_parts(public_key, len);
        //let pubkey_vec = Vec::from_raw_parts(public_key, len, len);
        println!("Got friend request from {:?}", pubkey_slice.to_hex());
    }
}

extern "C" fn callback(_tox: *mut Tox, connection_status: TOX_CONNECTION, _context: *mut ::std::os::raw::c_void) {
    println!("Connection to Tox network via {:?}", connection_status);
} 


fn main() {
    let major = unsafe { tox_version_major() };
    let minor = unsafe { tox_version_minor() };
    let patch = unsafe { tox_version_patch() };
    println!("Tox Version v{}.{}.{}", major, minor, patch);

    let mut err = 0;

    let mut tox = unsafe {tox_new(std::ptr::null(), &mut err)};

    unsafe { tox_callback_self_connection_status(tox, Some(callback)) };

    println!("tox_new(error={}) -> {:?}", err, tox);

    unsafe {
        let len = tox_address_size() as usize;
        let mut pubkey = Vec::<u8>::with_capacity(len);
        pubkey.set_len(len);
        
        tox_self_get_address(tox, pubkey.as_mut_ptr());
        println!("My public address: {}", pubkey.to_hex().to_uppercase());
    }

    unsafe {
        tox_callback_friend_request(tox, Some(friend_request));
    }

    unsafe {
        let address = CString::new("78.46.73.141").unwrap();
        let pubkey = "02807CF4F8BB8FB390CC3794BDF1E8449E9A8392C5D3F2200019DA9F1E812E46".from_hex().unwrap();
        let result = tox_bootstrap(tox, address.as_ptr(), 33445, pubkey.as_ptr(), &mut err);
        println!("tox_bootstrap -> {}", result);
    }

    unsafe {
        let mut nothing = 0 as *const ::std::os::raw::c_void;
        println!("loop tox_iterate()");
        loop { tox_iterate(tox, std::ptr::null_mut()); }
    }
    
}

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


#[repr(C)] pub struct Tox_Options { private: [u8; 0] }
#[repr(C)] pub struct Tox { private: [u8; 0] }
//#[repr(C)] pub struct Tox_Options { private: [u8; 0] }

#[repr(C)]
#[derive(Debug)]
enum TOX_CONNECTION {
    TOX_CONNECTION_NONE,
    TOX_CONNECTION_TCP,
    TOX_CONNECTION_UDP
}

#[repr(C)]
struct ToxContext {
    private: [u8; 0]
}

type tox_callback_self_connection_status_cb = extern "C" fn (_tox: *mut Tox, connection_status: TOX_CONNECTION, _context: *mut ToxContext);
type tox_friend_request_cb = extern "C" fn (_tox: *mut Tox, public_key: *const uint8_t, message: *const uint8_t, length: size_t, user_data: *mut ToxContext);


extern "C" fn friend_request(_tox: *mut Tox, public_key: *const uint8_t, message: *const uint8_t, length: size_t, user_data: *mut ToxContext)
{
    unsafe {
        let len = tox_address_size() as usize;
        let pubkey_slice = std::slice::from_raw_parts(public_key, len);
        //let pubkey_vec = Vec::from_raw_parts(public_key, len, len);
        println!("Got friend request from {:?}", pubkey_slice.to_hex());
    }
}

extern "C" fn callback(_tox: *mut Tox, connection_status: TOX_CONNECTION, _context: *mut ToxContext) {
    println!("Connection to Tox network via {:?}", connection_status);
} 

#[link(name = "toxcore")]
extern {
    fn tox_version_major() -> uint32_t;
    fn tox_version_minor() -> uint32_t;
    fn tox_version_patch() -> uint32_t;

    fn tox_new(options: *const Tox_Options, err: *mut uint32_t) -> *mut Tox;

    fn tox_public_key_size() -> uint32_t;
    fn tox_self_get_public_key(tox: *const Tox, public_address: *mut uint8_t);
    
    fn tox_address_size() -> uint32_t;
    fn tox_self_get_address(tox: *const Tox, public_address: *mut uint8_t);

    fn tox_callback_self_connection_status(tox: *const Tox, cb: tox_callback_self_connection_status_cb);

    fn tox_callback_friend_request(tox: *mut Tox, cb: tox_friend_request_cb);

    fn tox_iterate(tox: *const Tox);

    fn tox_bootstrap(tox: *mut Tox, address: *const c_char, port: uint16_t, public_key: *const uint8_t, error: *mut uint32_t) -> bool;
}

fn main() {
    let major = unsafe { tox_version_major() };
    let minor = unsafe { tox_version_minor() };
    let patch = unsafe { tox_version_patch() };
    println!("Tox Version v{}.{}.{}", major, minor, patch);

    let mut err = 0;

    let mut tox = unsafe {tox_new(std::ptr::null(), &mut err)};

    unsafe { tox_callback_self_connection_status(tox, callback) };

    println!("tox_new(error={}) -> {:?}", err, tox);

    unsafe {
        let len = tox_address_size() as usize;
        let mut pubkey = Vec::<u8>::with_capacity(len);
        pubkey.set_len(len);
        
        tox_self_get_address(tox, pubkey.as_mut_ptr());
        println!("My public address: {}", pubkey.to_hex().to_uppercase());
    }

    unsafe {
        tox_callback_friend_request(tox, friend_request);
    }

    unsafe {
        let address = CString::new("78.46.73.141").unwrap();
        let pubkey = "02807CF4F8BB8FB390CC3794BDF1E8449E9A8392C5D3F2200019DA9F1E812E46".from_hex().unwrap();
        let result = tox_bootstrap(tox, address.as_ptr(), 33445, pubkey.as_ptr(), &mut err);
        println!("tox_bootstrap -> {}", result);
    }

    unsafe {
        println!("loop tox_iterate()");
        loop { tox_iterate(tox) }
    }
    
}

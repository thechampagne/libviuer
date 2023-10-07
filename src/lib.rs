use std::os::raw::c_char;
use std::os::raw::c_int;
use std::ffi::CStr;
use viuer::{KittySupport,
	    Config,
	    get_kitty_support,
	    is_iterm_supported,
	    terminal_size,
	    print_from_file};

#[repr(C)]
#[allow(non_camel_case_types)]
enum viuer_kitty_support_t {
    VIUER_KITTY_SUPPORT_NONE,
    VIUER_KITTY_SUPPORT_LOCAL,
    VIUER_KITTY_SUPPORT_REMOTE,
}

#[repr(C)]
struct viuer_terminal_size_t {
    w: u16,
    h: u16
}

#[repr(C)]
struct viuer_image_size_t {
    w: u32,
    h: u32
}

#[repr(C)]
struct viuer_config_t {
    is_transparent: c_int,
    is_absolute_offset: c_int,
    x: u16,
    y: i16,
    is_restore_cursor: c_int,
    width: u32,
    height: u32,
    is_truecolor: c_int,
    is_use_kitty: c_int,
    is_use_iterm: c_int,
}

#[no_mangle]
unsafe extern "C" fn viuer_get_kitty_support() -> viuer_kitty_support_t {
    match get_kitty_support() {
	KittySupport::None => viuer_kitty_support_t::VIUER_KITTY_SUPPORT_NONE,
	KittySupport::Local => viuer_kitty_support_t::VIUER_KITTY_SUPPORT_LOCAL,
	KittySupport::Remote => viuer_kitty_support_t::VIUER_KITTY_SUPPORT_REMOTE,
    }
}

#[no_mangle]
unsafe extern "C" fn viuer_is_iterm_supported() -> c_int {
    if is_iterm_supported() {
	1
    } else {
	0
    }
}

#[no_mangle]
unsafe extern "C" fn viuer_terminal_size() -> viuer_terminal_size_t {
    let (w, h) = terminal_size();
    viuer_terminal_size_t{ w: w, h: h}
}

#[no_mangle]
unsafe extern "C" fn viuer_print_from_file(filename: *const c_char, config: *const viuer_config_t, img_size: *mut viuer_image_size_t) -> c_int {
    let cstr = match CStr::from_ptr(filename).to_str() {
	Ok(s) => s,
	Err(_) => return -1
    };
    match print_from_file(cstr, &config_handler(config)) {
	Ok(v) => {
	    let (w, h) = v;
	    (*img_size).w = w;
	    (*img_size).h = h;
	    0
	},
	Err(_) => -1
    }
}

#[inline(always)]
unsafe fn config_handler(config: *const viuer_config_t) -> Config {
    let c = &*config;
    Config {
	transparent: if c.is_transparent <= 0 { false } else { true },
	absolute_offset: if c.is_absolute_offset <= 0 { false } else { true },
	x: c.x,
	y: c.y,
	restore_cursor: if c.is_restore_cursor <= 0 { false } else { true },
	width: if c.width == 0 { None } else { Some(c.width) },
	height: if c.height == 0 { None } else { Some(c.height) },
	truecolor: if c.is_truecolor <= 0 { false } else { true },
	use_kitty: if c.is_use_kitty <= 0 { false } else { true },
	use_iterm: if c.is_use_iterm <= 0 { false } else { true },
    }
}

use macroquad::{
    prelude::*,
    ui::{hash, root_ui},
};
use std::str;

use macroquad::miniquad::native::android::{self, ndk_sys, ndk_utils};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

struct GlobalData {
    openfile: ndk_sys::jobject,
    data: Option<Arc<Mutex<Option<String>>>>,
    finish: Option<Arc<Mutex<bool>>>,
}
unsafe impl Send for GlobalData {}
unsafe impl Sync for GlobalData {}

static GLOBALS: Lazy<Mutex<GlobalData>> = Lazy::new(|| {
    Mutex::new(GlobalData {
        openfile: std::ptr::null_mut(),
        data: None,
        finish: None,
    })
});

#[no_mangle]
pub unsafe extern "C" fn Java_rust_quad_1log_1test_FileOpen_init() {
    let env = android::attach_jni_env();

    let mut globals = GLOBALS.lock().unwrap();
    let openfile = ndk_utils::new_object!(env, "rust/quad_log_test/FileOpen", "()V");
    assert!(!openfile.is_null());
    globals.openfile = ndk_utils::new_global_ref!(env, openfile);
}

#[no_mangle]
pub unsafe extern "C" fn Java_rust_quad_1log_1test_FileOpen_saveUri(
    env: *mut ndk_sys::JNIEnv,
    _: ndk_sys::jobject,
    array: ndk_sys::jbyteArray,
) {
    let mut globals = GLOBALS.lock().unwrap();

    let len = ((**env).GetArrayLength.unwrap())(env, array);
    let elements = ((**env).GetByteArrayElements.unwrap())(env, array, std::ptr::null_mut());
    let data = std::slice::from_raw_parts(elements as *mut u8, len as usize);

    if let Some(ref mut d) = globals.data {
        let s = match str::from_utf8(data) {
            Ok(v) => v.to_string(),
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        *d.lock().unwrap() = Some(s);
    }
    ((**env).ReleaseByteArrayElements.unwrap())(env, array, elements, 0);
}

#[no_mangle]
pub unsafe extern "C" fn Java_rust_quad_1log_1test_FileOpen_finish(
    _: *mut ndk_sys::JNIEnv,
    _: ndk_sys::jobject,
    _: ndk_sys::jbyteArray,
) {
    let mut globals = GLOBALS.lock().unwrap();
    if let Some(ref mut f) = globals.finish {
        *f.lock().unwrap() = false;
    }
}

fn finish_main_activity() {
    let env = unsafe { android::attach_jni_env() };
    let globals = GLOBALS.lock().unwrap();

    unsafe {
        ndk_utils::call_void_method!(env, globals.openfile, "finishMainActivity", "()V");
    }
}

fn find_file(data: Arc<Mutex<Option<String>>>, finish: Arc<Mutex<bool>>) {
    let env = unsafe { android::attach_jni_env() };
    let mut globals = GLOBALS.lock().unwrap();

    globals.data = Some(data);
    globals.finish = Some(finish);
    unsafe {
        ndk_utils::call_void_method!(env, globals.openfile, "OpenFileDialog", "()V");
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Quad!".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut exit = false;
    let data = std::sync::Arc::new(std::sync::Mutex::new(None));
    let finish = std::sync::Arc::new(std::sync::Mutex::new(false));
    let mut first = true;
    let mut text0 = None;

    loop {
        let val = &mut *finish.lock().unwrap();
        if *val == false {
            if !first {
                exit = true;
                break;
            }
            *val = true;
            first = false;
            find_file(data.clone(), finish.clone());
        }
        if let Some(_) = &*data.lock().unwrap() {
            break;
        }
    }

    loop {
        clear_background(WHITE);
        if exit {
            finish_main_activity();
        }
        {
            if let Some(data_val) = &*data.lock().unwrap() {
                let mut subtext = String::new();
                let mut i = 0;
                for part in data_val.bytes() {
                    i += 1;
                    let ar = &[part];
                    let s = str::from_utf8(ar).unwrap();
                    subtext.push_str(s);
                    if i % 20 == 0 {
                        subtext.push_str("\n");
                    }
                }
                text0 = Some(subtext);
            }

            let d = data.clone();
            *d.lock().unwrap() = None;
            if let Some(ref mut text) = text0 {
                root_ui().editbox(hash!(), vec2(440., 400.), text);
            }
        }
        next_frame().await;
    }
}

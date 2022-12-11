#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, unused)]

use std::error::Error;
use std::sync::Mutex;
use crate::library::library;
use crate::library::library_error::LibraryError;
use crate::sys;
use crate::util::util::ResultType;

type JniCreateJavaVm = unsafe extern "system" fn(
    pvm: *mut *mut sys::JavaVM,
    penv: *mut *mut std::os::raw::c_void,
    args: *mut std::os::raw::c_void,
) -> sys::jint;

static mut LIBRARY: Mutex<Option<libloading::Library>> = Mutex::new(None);

pub fn load_library(library_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let mut library = LIBRARY.lock().unwrap();
        if library.is_some() {
            return Err(Box::from(LibraryError::new("Library already loaded")));
        }

        library.replace(libloading::Library::new(library_path)?);
    }

    Ok(())
}

pub fn library_loaded() -> bool {
    unsafe { LIBRARY.lock().unwrap().is_some() }
}

pub fn get_jni_create_java_vm(
) -> ResultType<JniCreateJavaVm> {
    unsafe {
        Ok(*LIBRARY.lock()
            .unwrap()
            .as_ref()
            .ok_or(LibraryError::new("The library is not loaded"))?
            .get::<JniCreateJavaVm>(b"JNI_CreateJavaVM\0")?)
    }
}

use crate::java::java_vm::{InternalJavaOptions, JavaVM};
use lazy_static::lazy_static;

lazy_static! {
    static ref VM: JavaVM = JavaVM::new(
        &"1.8".to_string(),
        None,
        &vec![],
        InternalJavaOptions::default(),
    )
    .unwrap();
}

pub fn get_vm() -> JavaVM {
    VM.clone()
}

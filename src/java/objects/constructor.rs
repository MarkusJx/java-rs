use crate::java::java_env::JavaEnv;
use crate::java::objects::args::JavaArgs;
use crate::java::objects::class::{GlobalJavaClass, JavaClass};
use crate::java::objects::object::LocalJavaObject;
use crate::java::util::util::ResultType;
use crate::sys;
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct JavaConstructor<'a> {
    method: sys::jmethodID,
    class: &'a JavaClass<'a>,
}

impl<'a> JavaConstructor<'a> {
    pub(in crate::java) fn new(method: sys::jmethodID, class: &'a JavaClass<'a>) -> Self {
        Self { method, class }
    }

    pub fn new_instance<'b>(
        &self,
        env: &'b JavaEnv<'b>,
        args: JavaArgs,
    ) -> ResultType<LocalJavaObject<'b>> {
        env.get_env().new_instance(self, args)
    }

    pub(in crate::java) unsafe fn class(&self) -> ResultType<sys::jclass> {
        self.class.class()
    }

    pub(in crate::java) unsafe fn id(&self) -> sys::jmethodID {
        self.method
    }

    pub fn from_global(global: &GlobalJavaConstructor, class: &'a JavaClass<'a>) -> Self {
        Self {
            method: global.method.load(Ordering::Relaxed),
            class,
        }
    }
}

pub struct GlobalJavaConstructor {
    method: AtomicPtr<sys::_jmethodID>,
    class: GlobalJavaClass,
}

impl GlobalJavaConstructor {
    pub fn from_local(local: JavaConstructor<'_>, class: GlobalJavaClass) -> Self {
        Self {
            method: AtomicPtr::new(local.method),
            class,
        }
    }

    pub fn get_class<'a, 'b>(&'a self, env: &'b JavaEnv<'b>) -> JavaClass<'b>
    where
        'a: 'b,
    {
        JavaClass::from_global(&self.class, env)
    }
}

impl Clone for GlobalJavaConstructor {
    fn clone(&self) -> Self {
        Self {
            method: AtomicPtr::new(self.method.load(Ordering::Relaxed)),
            class: self.class.clone(),
        }
    }
}

unsafe impl Send for GlobalJavaConstructor {}

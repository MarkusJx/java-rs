use crate::java::java_env::JavaEnv;
use crate::java::objects::args::JavaArgs;
use crate::java::objects::class::{GlobalJavaClass, JavaClass};
use crate::java::objects::object::LocalJavaObject;
use crate::java::util::util::ResultType;
use crate::signature::Signature;
use crate::{assert_non_null, sys};
use std::sync::atomic::{AtomicPtr, Ordering};

pub struct JavaConstructor<'a> {
    method: sys::jmethodID,
    class: &'a JavaClass<'a>,
    signature: Signature,
}

impl<'a> JavaConstructor<'a> {
    pub(in crate::java) fn new(
        method: sys::jmethodID,
        class: &'a JavaClass<'a>,
        signature: Signature,
    ) -> Self {
        assert_non_null!(method);
        Self {
            method,
            class,
            signature,
        }
    }

    pub fn new_instance<'b>(
        &self,
        env: &'b JavaEnv<'b>,
        args: JavaArgs,
    ) -> ResultType<LocalJavaObject<'b>> {
        env.get_env().new_instance(self, args)
    }

    pub(in crate::java) unsafe fn class(&self) -> sys::jclass {
        self.class.class()
    }

    pub(in crate::java) unsafe fn id(&self) -> sys::jmethodID {
        self.method
    }

    pub fn get_signature(&self) -> &Signature {
        &self.signature
    }

    pub fn get_class(&self) -> &JavaClass<'a> {
        self.class
    }

    pub fn from_global(global: &GlobalJavaConstructor, class: &'a JavaClass<'a>) -> Self {
        Self {
            method: global.method.load(Ordering::Relaxed),
            class,
            signature: global.signature.clone(),
        }
    }
}

pub struct GlobalJavaConstructor {
    method: AtomicPtr<sys::_jmethodID>,
    class: GlobalJavaClass,
    signature: Signature,
}

impl GlobalJavaConstructor {
    pub fn from_local(
        local: JavaConstructor<'_>,
        class: GlobalJavaClass,
        signature: Signature,
    ) -> Self {
        Self {
            method: AtomicPtr::new(local.method),
            class,
            signature,
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
            signature: self.signature.clone(),
        }
    }
}

unsafe impl Send for GlobalJavaConstructor {}

use crate::java::java_env::JavaEnv;
use crate::java::java_env_wrapper::JavaEnvWrapper;
use crate::java::java_field::JavaField;
use crate::java::java_type::JavaType;
use crate::java::objects::constructor::JavaConstructor;
use crate::java::objects::method::{
    JavaBooleanMethod, JavaByteMethod, JavaCharMethod, JavaDoubleMethod, JavaFloatMethod,
    JavaIntMethod, JavaLongMethod, JavaObjectMethod, JavaShortMethod, JavaVoidMethod,
    StaticJavaBooleanMethod, StaticJavaByteMethod, StaticJavaCharMethod, StaticJavaDoubleMethod,
    StaticJavaFloatMethod, StaticJavaIntMethod, StaticJavaLongMethod, StaticJavaObjectMethod,
    StaticJavaShortMethod, StaticJavaVoidMethod,
};
use crate::java::objects::object::{GlobalJavaObject, LocalJavaObject};
use crate::java::traits::GetRaw;
use crate::java::util::util::ResultType;
use crate::traits::GetSignature;
use crate::{assert_non_null, define_get_method_method, sys};
use std::error::Error;

pub struct JavaClass<'a> {
    object: LocalJavaObject<'a>,
    signature: JavaType,
}

impl<'a> JavaClass<'a> {
    pub fn new(object: sys::jclass, env: &'a JavaEnvWrapper<'a>, signature: JavaType) -> Self {
        assert_non_null!(object);
        Self {
            object: LocalJavaObject::new(object, env, JavaType::object()),
            signature,
        }
    }

    pub fn by_name(name: &str, env: &'a JavaEnv<'a>) -> ResultType<Self> {
        env.find_class(name)
    }

    pub fn by_java_name(name: String, env: &'a JavaEnv<'a>) -> ResultType<Self> {
        env.find_class_by_java_name(name)
    }

    pub fn get_object_method(
        &'a self,
        method_name: &str,
        signature: &str,
    ) -> ResultType<JavaObjectMethod<'a>> {
        self.get_object_method_with_errors(method_name, signature, true)
    }

    pub(in crate::java) fn env(&'a self) -> &'a JavaEnvWrapper<'a> {
        self.object.env()
    }

    define_get_method_method!(get_int_method, get_method_id, JavaIntMethod);
    define_get_method_method!(get_long_method, get_method_id, JavaLongMethod);
    define_get_method_method!(get_double_method, get_method_id, JavaDoubleMethod);
    define_get_method_method!(get_float_method, get_method_id, JavaFloatMethod);
    define_get_method_method!(get_boolean_method, get_method_id, JavaBooleanMethod);
    define_get_method_method!(get_short_method, get_method_id, JavaShortMethod);
    define_get_method_method!(get_byte_method, get_method_id, JavaByteMethod);
    define_get_method_method!(get_char_method, get_method_id, JavaCharMethod);
    define_get_method_method!(get_void_method, get_method_id, JavaVoidMethod);

    define_get_method_method!(
        get_static_int_method,
        get_static_method_id,
        StaticJavaIntMethod
    );
    define_get_method_method!(
        get_static_long_method,
        get_static_method_id,
        StaticJavaLongMethod
    );
    define_get_method_method!(
        get_static_double_method,
        get_static_method_id,
        StaticJavaDoubleMethod
    );
    define_get_method_method!(
        get_static_float_method,
        get_static_method_id,
        StaticJavaFloatMethod
    );
    define_get_method_method!(
        get_static_short_method,
        get_static_method_id,
        StaticJavaShortMethod
    );
    define_get_method_method!(
        get_static_byte_method,
        get_static_method_id,
        StaticJavaByteMethod
    );
    define_get_method_method!(
        get_static_char_method,
        get_static_method_id,
        StaticJavaCharMethod
    );
    define_get_method_method!(
        get_static_void_method,
        get_static_method_id,
        StaticJavaVoidMethod
    );

    pub fn get_object_method_with_errors(
        &'a self,
        method_name: &str,
        signature: &str,
        resolve_errors: bool,
    ) -> ResultType<JavaObjectMethod<'a>> {
        let method = self.object.env().get_method_id_with_errors(
            &self,
            method_name,
            signature,
            resolve_errors,
        )?;

        JavaObjectMethod::new(method)
    }

    pub fn get_static_object_method(
        &'a self,
        method_name: &str,
        signature: &str,
    ) -> ResultType<StaticJavaObjectMethod<'a>> {
        let method = self
            .object
            .env()
            .get_static_method_id(&self, method_name, signature)?;

        StaticJavaObjectMethod::new(method)
    }

    pub fn get_static_boolean_method(
        &'a self,
        method_name: &str,
        signature: &str,
    ) -> ResultType<StaticJavaBooleanMethod<'a>> {
        let method = self
            .object
            .env()
            .get_static_method_id(&self, method_name, signature)?;

        StaticJavaBooleanMethod::new(method)
    }

    pub fn get_constructor(&self, signature: &str) -> ResultType<JavaConstructor> {
        self.env().get_constructor(self, signature)
    }

    pub fn get_field(
        &'a self,
        name: String,
        signature: JavaType,
        is_static: bool,
    ) -> ResultType<JavaField<'a>> {
        self.object
            .env()
            .get_field_id(&self, name, signature, is_static)
    }

    pub fn from_global(object: &'a GlobalJavaClass, env: &'a JavaEnv<'a>) -> Self {
        Self {
            object: LocalJavaObject::from(&object.object, env),
            signature: object.get_signature().clone(),
        }
    }

    pub fn to_object(&'a self) -> &'a LocalJavaObject<'a> {
        &self.object
    }

    pub fn is_assignable_from(&self, other: &JavaClass) -> ResultType<bool> {
        unsafe {
            self.object
                .env()
                .is_assignable_from(other.class(), self.class())
        }
    }

    pub(in crate::java) unsafe fn class(&self) -> sys::jclass {
        self.object.get_raw()
    }

    pub fn from_local(object: LocalJavaObject<'a>, signature: JavaType) -> Self {
        Self { object, signature }
    }
}

impl GetSignature for JavaClass<'_> {
    fn get_signature(&self) -> &JavaType {
        &self.signature
    }
}

#[derive(Clone)]
pub struct GlobalJavaClass {
    object: GlobalJavaObject,
    signature: JavaType,
}

impl GlobalJavaClass {
    pub fn by_name(name: &str, env: &JavaEnv<'_>) -> ResultType<Self> {
        env.find_global_class_by_java_name(name.replace('/', "."))
    }

    pub fn into_object(self) -> GlobalJavaObject {
        self.object
    }

    pub(in crate::java) unsafe fn class(&self) -> sys::jclass {
        self.object.get_raw()
    }

    pub fn from_global(object: GlobalJavaObject, signature: JavaType) -> Self {
        Self { object, signature }
    }

    pub fn try_from_local(object: LocalJavaObject<'_>, signature: JavaType) -> ResultType<Self> {
        let global = GlobalJavaObject::try_from(object)?;
        Ok(Self {
            object: global,
            signature,
        })
    }
}

impl TryFrom<JavaClass<'_>> for GlobalJavaClass {
    type Error = Box<dyn Error>;

    fn try_from(class: JavaClass) -> ResultType<Self> {
        let global = GlobalJavaObject::try_from(class.object)?;
        Ok(Self {
            object: global,
            signature: class.signature,
        })
    }
}

impl GetSignature for GlobalJavaClass {
    fn get_signature(&self) -> &JavaType {
        &self.signature
    }
}

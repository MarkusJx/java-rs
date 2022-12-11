use crate::signature::Signature;

#[test]
fn parse_signature() {
    let sig = Signature::from_jni("(Ljava/lang/String;I)Ljava/lang/String;").unwrap();
    assert_eq!(sig.num_args(), 2);
    assert_eq!(sig.get_args()[0].to_string(), "java.lang.String");
    assert_eq!(sig.get_args()[1].to_string(), "int");
    assert_eq!(sig.get_return_type().to_string(), "java.lang.String");
}

#[test]
fn parse_signature_with_array() {
    let sig = Signature::from_jni("([Ljava/lang/String;I)Ljava/lang/String;").unwrap();
    println!("{}", sig);
    assert_eq!(sig.num_args(), 2);
    assert_eq!(sig.get_args()[0].to_string(), "java.lang.String[]");
    assert!(sig.get_args()[0].is_array());
    assert_eq!(sig.get_args()[1].to_string(), "int");
    assert_eq!(sig.get_return_type().to_string(), "java.lang.String");
}

#[test]
fn parse_signature_with_void() {
    let sig = Signature::from_jni("()V").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "void");
}

#[test]
fn parse_signature_with_void_array() {
    let sig = Signature::from_jni("()[V").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "void[]");
    assert!(sig.get_return_type().is_array());
}

#[test]
fn parse_signature_with_void_array_array() {
    let sig = Signature::from_jni("()[[V").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "void[][]");
    assert!(sig.get_return_type().is_array());
}

#[test]
fn parse_signature_with_int_array() {
    let sig = Signature::from_jni("()[I").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "int[]");
    assert!(sig.get_return_type().is_array());
}

#[test]
fn parse_signature_with_int_array_array() {
    let sig = Signature::from_jni("()[[I").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "int[][]");
    assert!(sig.get_return_type().is_array());
}

#[test]
fn parse_signature_with_int_array_array_array() {
    let sig = Signature::from_jni("()[[[I").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "int[][][]");
    assert!(sig.get_return_type().is_array());
}

#[test]
fn parse_signature_with_boolean_array() {
    let sig = Signature::from_jni("()[Z").unwrap();
    assert_eq!(sig.num_args(), 0);
    assert_eq!(sig.get_return_type().to_string(), "boolean[]");
    assert!(sig.get_return_type().is_array());
}

#[test]
fn parse_signature_with_many_args() {
    let sig = Signature::from_jni("(Ljava/lang/String;I[[I[[[Z)V").unwrap();
    println!("{}", sig);
    assert_eq!(sig.num_args(), 4);
    assert_eq!(sig.get_args()[0].to_string(), "java.lang.String");
    assert_eq!(sig.get_args()[1].to_string(), "int");
    assert_eq!(sig.get_args()[2].to_string(), "int[][]");
    assert_eq!(sig.get_args()[3].to_string(), "boolean[][][]");
    assert_eq!(sig.get_return_type().to_string(), "void");
}

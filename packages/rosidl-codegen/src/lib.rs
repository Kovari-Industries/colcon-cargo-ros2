pub mod generator;
pub mod idl_generator;
pub mod templates;
pub mod types;
pub mod utils;

pub use generator::{
    GeneratedActionPackage, GeneratedPackage, GeneratedServicePackage, GeneratorError,
    generate_action_package, generate_message_package, generate_service_package,
};
pub use idl_generator::{GeneratedIdlCode, extract_annotations, generate_idl_file};
pub use types::{
    FieldTypeExt, IdlTypeExt, escape_keyword, idl_constant_value_to_rust, rust_type_for_field,
    rust_type_for_idl, rust_type_for_idl_constant,
};

#[cfg(test)]
mod tests {
    use super::*;
    use rosidl_parser::{FieldType, PrimitiveType, parse_message};

    #[test]
    fn test_basic_type_mapping() {
        let field_type = FieldType::Primitive(PrimitiveType::Int32);
        let rust_type = rust_type_for_field(&field_type, false, None);
        assert_eq!(rust_type, "i32");
    }

    #[test]
    fn test_keyword_escaping() {
        assert_eq!(escape_keyword("type"), "type_");
        assert_eq!(escape_keyword("match"), "match_");
        assert_eq!(escape_keyword("normal"), "normal");
    }

    #[test]
    fn test_simple_message_generation() {
        let msg = parse_message("int32 x\nfloat64 y\n").unwrap();
        let result = generate_message_package(
            "test_msgs",
            "TestMessage",
            &msg,
            &std::collections::HashSet::new(),
        );
        assert!(result.is_ok());
    }
}

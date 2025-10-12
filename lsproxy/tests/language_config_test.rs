use lsproxy::api_types::SupportedLanguages;
use std::str::FromStr;

#[test]
fn test_parse_valid_single_language() {
    let result = SupportedLanguages::from_str("python");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), SupportedLanguages::Python);
}

#[test]
fn test_parse_valid_typescript_javascript() {
    let result = SupportedLanguages::from_str("typescript_javascript");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), SupportedLanguages::TypeScriptJavaScript);
}

#[test]
fn test_parse_all_valid_languages() {
    let languages = vec![
        ("python", SupportedLanguages::Python),
        ("typescript_javascript", SupportedLanguages::TypeScriptJavaScript),
        ("rust", SupportedLanguages::Rust),
        ("cpp", SupportedLanguages::CPP),
        ("csharp", SupportedLanguages::CSharp),
        ("java", SupportedLanguages::Java),
        ("golang", SupportedLanguages::Golang),
        ("php", SupportedLanguages::PHP),
        ("ruby", SupportedLanguages::Ruby),
        ("ruby_sorbet", SupportedLanguages::RubySorbet),
    ];

    for (lang_str, expected) in languages {
        let result = SupportedLanguages::from_str(lang_str);
        assert!(
            result.is_ok(),
            "Failed to parse language: {}",
            lang_str
        );
        assert_eq!(result.unwrap(), expected);
    }
}

#[test]
fn test_parse_invalid_language() {
    let result = SupportedLanguages::from_str("invalid_language");
    assert!(result.is_err());
}

#[test]
fn test_parse_empty_string() {
    let result = SupportedLanguages::from_str("");
    assert!(result.is_err());
}

#[test]
fn test_parse_case_sensitive() {
    // The EnumString should be case-sensitive based on strum(serialize_all = "lowercase")
    let result = SupportedLanguages::from_str("Python");
    assert!(result.is_err(), "Should be case-sensitive");
}


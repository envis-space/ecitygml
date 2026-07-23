/// A typed decoding of a `gml:CodeType` value against one specific code
/// list (one authority's `codeSpace`), e.g. a SIG3D standard list, a
/// regional extension, or a project-specific vocabulary.
///
/// Implement this once per concrete code list to get typed, pattern-
/// matchable access to properties like `class`, `function`, and `usage`
/// without the core model needing to know about every list that exists.
/// Multiple implementations can coexist for the same property; callers
/// pick the one they want via the type parameter on `interpret`/`from_value`.
pub trait CodeListValue: Sized {
    /// The codeSpace URI this implementation decodes values against.
    /// Values with no codeSpace at all are still matched against this
    /// list, since many real CityGML files omit it and rely on convention.
    const CODE_SPACE: &'static str;

    fn from_code_value(value: &str) -> Self;

    fn to_code_value(&self) -> &str;
}

/// Generates a `gml:CodeType` wrapper newtype: always holds the raw `Code`
/// as read/written, preserving codeSpace and value exactly even for
/// authorities the core model doesn't know about, with optional typed
/// interpretation against any code list implementing `CodeListValue`.
#[macro_export]
macro_rules! code_list_value_wrapper {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $name(egml::model::basic_types::Code);

        impl $name {
            pub fn code(&self) -> &egml::model::basic_types::Code {
                &self.0
            }

            pub fn value(&self) -> &str {
                &self.code().value()
            }

            pub fn into_code(self) -> egml::model::basic_types::Code {
                self.0
            }

            /// Interprets this value against a specific code list, if its
            /// codeSpace matches (or this value has no codeSpace at all).
            pub fn interpret<T: $crate::model::common::CodeListValue>(&self) -> Option<T> {
                let matches = self.0.code_space().is_none_or(|cs| cs == T::CODE_SPACE);
                matches.then(|| T::from_code_value(self.0.value()))
            }

            /// Encodes a typed value using that code list's codeSpace.
            pub fn from_value<T: $crate::model::common::CodeListValue>(value: &T) -> Self {
                Self(egml::model::basic_types::Code::with_code_space(
                    T::CODE_SPACE,
                    value.to_code_value(),
                ))
            }
        }

        impl From<egml::model::basic_types::Code> for $name {
            fn from(code: egml::model::basic_types::Code) -> Self {
                Self(code)
            }
        }
    };
}

pub trait Validate {
    fn validate(&self) -> Result<ValidationResult, ValidationError> {
        self.validate_with_context(ValidationContext::default())
    }

    fn validate_with_context(
        &self,
        context: ValidationContext,
    ) -> Result<ValidationResult, ValidationError>;
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationContext(pub(crate) Vec<ValidationPathComponent>);

impl ValidationContext {
    pub(crate) fn extend_context(&self, components: Vec<ValidationPathComponent>) -> Self {
        let mut extended_context = self.0.clone();
        extended_context.extend(components);
        Self(extended_context)
    }

    pub(crate) fn extend_context_with_struct_field(
        &self,
        struct_name: impl ToString,
        field_name: impl ToString,
    ) -> Self {
        let component = vec![ValidationPathComponent::Struct {
            struct_name: struct_name.to_string(),
            field_name: field_name.to_string(),
        }];

        self.extend_context(component)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationPathComponent {
    Struct {
        struct_name: String,
        field_name: String,
    },
    Array {
        index: usize,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationResult {
    Passed,
    Failed { reasons: Vec<FailureReason> },
}

impl ValidationResult {
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::Passed, Self::Passed) => Self::Passed,
            (Self::Passed, Self::Failed { reasons }) => Self::Failed { reasons },
            (Self::Failed { reasons }, Self::Passed) => Self::Failed { reasons },
            (
                Self::Failed {
                    reasons: mut left_reasons,
                },
                Self::Failed {
                    reasons: mut right_reasons,
                },
            ) => {
                left_reasons.append(&mut right_reasons);
                Self::Failed {
                    reasons: left_reasons,
                }
            }
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::Passed
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FailureReason {
    pub message: String,
    pub context: ValidationContext,
}
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ValidationError {
    #[error("Failed to compile regular expression: {0}")]
    InvalidRegularExpressionError(#[from] regex::Error),
}

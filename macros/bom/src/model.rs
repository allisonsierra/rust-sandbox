use validate::validate::{
    FailureReason, Validate, ValidationContext, ValidationError, ValidationResult,
};
use validate_derive::Validate;

#[derive(Debug, PartialEq)]
pub struct Bom {
    pub version: u32,
    pub serial_number: Option<UrnUuid>,
    pub metadata: Option<Metadata>,
    pub component: Option<Component>,
}

#[derive(Validate, Debug, PartialEq)]
pub enum Component {
    Gadget,
    Widget,
    Wizzlebop,
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub category: Option<Category>,
    pub tags: Option<Tags>,
}

#[derive(Debug, PartialEq)]
pub enum Category {
    Dookie,
    Nifty,
    Uncategorized,
    Unknown(String),
}

impl Validate for Category {
    fn validate_with_context(
        &self,
        context: ValidationContext,
    ) -> Result<ValidationResult, ValidationError> {
        match self {
            Category::Unknown(string) => Ok(ValidationResult::Failed {
                reasons: vec![FailureReason {
                    message: format!("Category unknown: {}", string),
                    context,
                }],
            }),
            _ => Ok(ValidationResult::Passed),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Tags(pub Vec<String>);

#[derive(Debug, PartialEq)]
pub struct UrnUuid(pub(crate) String);

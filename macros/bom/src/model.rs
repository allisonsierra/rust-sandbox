#[derive(Debug, PartialEq)]
pub struct Bom {
    pub version: u32,
    pub serial_number: Option<UrnUuid>,
    pub metadata: Option<Metadata>,
    pub component: Option<Component>,
}

#[derive(Debug, PartialEq)]
pub enum Component {
    Gadget,
    Widget,
    Wizzlebop,
    Custom(String),
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
    Custom(String),
}

#[derive(Debug, PartialEq)]
pub struct Tags(pub Vec<String>);

#[derive(Debug, PartialEq)]
pub struct UrnUuid(pub(crate) String);

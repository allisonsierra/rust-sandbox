pub struct Groups {
    groups: Vec<Group>,
}

pub struct Group {
    group: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_translate_to_groups() {
        use std::fs;

        let mock_filename = "./mock/invert-me.json".to_string();
        let json = fs::read_to_string(mock_filename).expect("Error reading ./mock/invert-me.json");
    }
}

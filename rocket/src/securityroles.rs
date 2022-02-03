use quick_xml::de::{from_str, DeError};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename = "security")]
pub struct SecurityRoles {
    #[serde(rename = "userRoleMappings")]
    user_role_mappings: Vec<UserRoleMapping>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserRoleMapping {
    user_id: String,
    source: String,
    roles: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_from_xml() {
        let xml_simple = r#"<?xml version="1.0" encoding="UTF-8"?>
        <security>
          <userRoleMappings>
            <userRoleMapping>
              <userId>user-with-one-role</userId>
              <source>default</source>
              <roles>
                <role>org.group.4users-deployer</role>
              </roles>
            </userRoleMapping>
          </userRoleMappings>
        </security>
        "#;

        let xml_complex = r#"<?xml version="1.0" encoding="UTF-8"?>
        <security>
          <userRoleMappings>
            <userRoleMapping>
              <userId>user-with-one-role</userId>
              <source>default</source>
              <roles>
                <role>org.group.4users-deployer</role>
              </roles>
            </userRoleMapping>
            <userRoleMapping>
              <userId>user-with-two-roles</userId>
              <source>default</source>
              <roles>
                <role>org.group.4users-deployer</role>
                <role>org.group.3users-deployer</role>
              </roles>
            </userRoleMapping>
          </userRoleMappings>
        </security>
        "#;

        let expected: SecurityRoles = SecurityRoles {
            user_role_mappings: vec![UserRoleMapping {
                user_id: "user-with-one-role".to_string(),
                source: "default".to_string(),
                roles: vec!["org.group.4users-deployer".to_string()],
            }],
        };

        dbg!(expected.clone());

        let actual: SecurityRoles = from_str(xml_simple).expect("Failed to parse XML");
        assert_eq!(actual, expected);
    }
}

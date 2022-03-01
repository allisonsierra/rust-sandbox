use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SecurityRoles {
    security: UserRoleMappings,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct UserRoleMappings {
    #[serde(rename(deserialize = "userRoleMappings"))]
    user_role_mappings: Vec<UserRoleMapping>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct UserRoleMapping {
    #[serde(rename(deserialize = "userId"))]
    user_id: String,
    source: String,
    roles: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_deserialize_from_json() {
        let json = r#"{
          "security": {
              "userRoleMappings": [
                  {
                      "userId": "user-with-one-role",
                      "source": "default",
                      "roles": [
                          "org.group.4users-deployer"
                      ]
                  },
                  {
                      "userId": "user-with-two-roles",
                      "source": "default",
                      "roles": [
                          "org.group.4users-deployer",
                          "org.group.3users-deployer"
                      ]
                  },
                  {
                      "userId": "user-with-three-roles",
                      "source": "default",
                      "roles": [
                          "org.group.4users-deployer",
                          "org.group.3users-deployer",
                          "org.group.2users-deployer"
                      ]
                  },
                  {
                      "userId": "user-with-ten-roles",
                      "source": "default",
                      "roles": [
                          "org.group.4users-deployer",
                          "org.group.3users-deployer",
                          "org.group.2users-deployer",
                          "org.group.1user1-deployer",
                          "org.group.1user2-deployer",
                          "org.group.1user3-deployer",
                          "org.group.1user4-deployer",
                          "org.group.1user5-deployer",
                          "org.group.1user6-deployer",
                          "org.group.1user7-deployer"
                      ]
                  }
              ]
          }
      }
      "#;

        let expected: SecurityRoles = SecurityRoles {
            security: UserRoleMappings {
                user_role_mappings: vec![
                    UserRoleMapping {
                        user_id: "user-with-one-role".to_string(),
                        source: "default".to_string(),
                        roles: vec!["org.group.4users-deployer".to_string()],
                    },
                    UserRoleMapping {
                        user_id: "user-with-two-roles".to_string(),
                        source: "default".to_string(),
                        roles: vec![
                            "org.group.4users-deployer".to_string(),
                            "org.group.3users-deployer".to_string(),
                        ],
                    },
                    UserRoleMapping {
                        user_id: "user-with-three-roles".to_string(),
                        source: "default".to_string(),
                        roles: vec![
                            "org.group.4users-deployer".to_string(),
                            "org.group.3users-deployer".to_string(),
                            "org.group.2users-deployer".to_string(),
                        ],
                    },
                    UserRoleMapping {
                        user_id: "user-with-ten-roles".to_string(),
                        source: "default".to_string(),
                        roles: vec![
                            "org.group.4users-deployer".to_string(),
                            "org.group.3users-deployer".to_string(),
                            "org.group.2users-deployer".to_string(),
                            "org.group.1user1-deployer".to_string(),
                            "org.group.1user2-deployer".to_string(),
                            "org.group.1user3-deployer".to_string(),
                            "org.group.1user4-deployer".to_string(),
                            "org.group.1user5-deployer".to_string(),
                            "org.group.1user6-deployer".to_string(),
                            "org.group.1user7-deployer".to_string(),
                        ],
                    },
                ],
            },
        };

        let actual: SecurityRoles = serde_json::from_str(json).expect("Failed to parse JSON");
        assert_eq!(actual, expected);
    }
}

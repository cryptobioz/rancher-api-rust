use serde_json;
use super::Rancher;

/// Struct that contains all data for a Rancher Environment
/// (http://rancher.com/docs/rancher/v1.6/en/api/v2-beta/api-resources/project/)
#[derive(PartialEq, PartialOrd, Debug)]
pub struct Environment {
    /// Unique ID.
    pub id: String,
    /// Name.
    pub name: String
}


impl Environment {
    /// Get all environments.
    pub fn get_all(rancher: &mut Rancher) -> Result<Vec<Environment>, &'static str> {
        let mut data: Vec<Environment> = Vec::new();
        let results: serde_json::Value = rancher.call_api("/projects").unwrap();
        let projects = results["data"].as_array().unwrap();
        for project in projects {
            data.push(Environment{
                id: project["id"].as_str().unwrap().to_string(),
                name: project["name"].as_str().unwrap().to_string()
            });
        }
        return Ok(data);
    }
    /// Get one environment from a pattern that should match with the environment id or the
    /// environment name.
    pub fn get_one(rancher: &mut Rancher, pattern: &str) -> Result<Environment, &'static str> {
        let results: serde_json::Value = rancher.call_api("/projects").unwrap();
        let projects = results["data"].as_array().unwrap();
        for project in projects {
            if pattern == project["id"].as_str().unwrap() || pattern == project["name"].as_str().unwrap() {
                return Ok(Environment{
                    id: project["id"].as_str().unwrap().to_string(),
                    name: project["name"].as_str().unwrap().to_string()
                });
            }
        }
        return Err("No environment found.");
    }
}


/// This trait allows you to call Environment's functions with `rancher.get_environment(........)`.
pub trait EnvironmentTrait {
    /// Get all environments.
    fn get_environments(&mut self) -> Result<Vec<Environment>, &'static str>;
    /// Get one environment from a pattern that should match with the environment id or the
    /// environment name.
    fn get_environment(&mut self, pattern: &str) -> Result<Environment, &'static str>;
}

impl EnvironmentTrait for Rancher {
    fn get_environments(&mut self) -> Result<Vec<Environment>, &'static str> {
        return Environment::get_all(self);
    }
    fn get_environment(&mut self, pattern: &str) -> Result<Environment, &'static str> {
        return Environment::get_one(self, pattern);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all() {
        let expected_result = vec![
            Environment{
                id: "1a10".to_owned(),
                name: "fakeEnvironment1".to_owned(),
            },
            Environment{
                id: "1a11".to_owned(),
                name: "fakeEnvironment2".to_owned(),
            }
        ];
        let api_response = r#"{
                                "data": [
                                    {
                                        "id": "1a10",
                                        "name": "fakeEnvironment1"
                                    },
                                    {
                                        "id": "1a11",
                                        "name": "fakeEnvironment2"
                                    }
                                ]
                            }"#;
        let mut data: Vec<Environment> = Vec::new();
        let results: serde_json::Value = serde_json::from_str(api_response).unwrap();
        let projects = results["data"].as_array().unwrap();
        for project in projects {
            data.push(Environment{
                id: project["id"].as_str().unwrap().to_string(),
                name: project["name"].as_str().unwrap().to_string()
            });
        }

        assert_eq!(expected_result.eq(&data), true, "Expected {:?}, got {:?}", expected_result, data);
    }
    
    #[test]
    fn get_one() {
        let expected_result = Environment{
            id: "1a10".to_owned(),
            name: "fakeEnvironment1".to_owned(),
        };
        let api_response = r#"{
                                "data": [
                                    {
                                        "id": "1a10",
                                        "name": "fakeEnvironment1"
                                    },
                                    {
                                        "id": "1a11",
                                        "name": "fakeEnvironment2"
                                    }
                                ]
                            }"#;
        let pattern = "1a10";
        let mut data: Option<Environment> = None;
        let results: serde_json::Value = serde_json::from_str(api_response).unwrap();
        let projects = results["data"].as_array().unwrap();
        for project in projects {
            if pattern == project["id"].as_str().unwrap() || pattern == project["name"].as_str().unwrap() {
                data = Some(Environment{
                    id: project["id"].as_str().unwrap().to_string(),
                    name: project["name"].as_str().unwrap().to_string()
                });
                break;
            }
        }
        if data.is_some() {
            let result = data.unwrap();
            assert_eq!(expected_result.eq(&result), true, "Expected {:?}, got {:?}", expected_result, result);
        }
    }
}

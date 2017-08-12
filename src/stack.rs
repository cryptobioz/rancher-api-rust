use serde_json;
use super::Rancher;

/// Struct that contains all data for a Rancher Stack
/// (http://rancher.com/docs/rancher/v1.6/en/api/v2-beta/api-resources/stack/)
pub struct Stack {
    /// Stack unique ID.
    pub id: String,
    /// Stack name.
    pub name: String
}

impl Stack {
    /// Get all stacks from an environment.
    pub fn get_all(rancher: &mut Rancher, environment_id: &str) -> Result<Vec<Stack>, &'static str> {
        let mut data: Vec<Stack> = Vec::new();
        let path = format!("/projects/{}/stacks", environment_id);
        let results = rancher.call_api(&path).unwrap();
        let stacks = results["data"].as_array().unwrap();
        for stack in stacks {
            data.push(Stack{
                id: stack["id"].as_str().unwrap().to_string(),
                name: stack["name"].as_str().unwrap().to_string()
            });
        }
        return Ok(data);
    }
    /// Get a stack from an environment and a pattern that should match with the stack ID or the
    /// stack name.
    pub fn get_one(rancher: &mut Rancher, environment_id: &str, pattern: &str) -> Result<Stack, &'static str> {
        let path = format!("/projects/{}/stacks", environment_id);
        let results: serde_json::Value = rancher.call_api(&path).unwrap();
        let stacks = results["data"].as_array().unwrap();
        for stack in stacks {
            if pattern == stack["id"].as_str().unwrap() || pattern == stack["name"].as_str().unwrap() {
                return Ok(Stack{
                    id: stack["id"].as_str().unwrap().to_string(),
                    name: stack["name"].as_str().unwrap().to_string()
                });
            }
        }
        return Err("No stack found.");
    }
}
/// This trait allows you to call Stack's functions with `rancher.get_stack(......)`.
pub trait StackTrait {
    /// Get all stacks from an environment.
    fn get_stacks(&mut self, environment_id: &str) -> Result<Vec<Stack>, &'static str>;
    /// Get a stack from an environment and a pattern that should match with the stack ID or the
    /// stack name.
    fn get_stack(&mut self, environment_id: &str, pattern: &str) -> Result<Stack, &'static str>;
}

impl StackTrait for Rancher {
    fn get_stacks(&mut self, environment_id: &str) -> Result<Vec<Stack>, &'static str> {
        return Stack::get_all(self, environment_id);
    }

    fn get_stack(&mut self, environment_id: &str, pattern: &str) -> Result<Stack, &'static str> {
        return Stack::get_one(self, environment_id, pattern);
    }
}

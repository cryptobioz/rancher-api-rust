use serde_json;
use super::Rancher;

/// Struct that contains all data for a Rancher Service
/// (http://rancher.com/docs/rancher/v1.6/en/api/v2-beta/api-resources/service/).
pub struct Service {
    /// Service unique ID.
    pub id: String,
    /// Service name.
    pub name: String
}

impl Service {
    /// Get all services from an environment.
    pub fn get_all(rancher: &mut Rancher, environment_id: &str) -> Result<Vec<Service>, &'static str> {
        let mut data: Vec<Service> = Vec::new();
        let path = format!("/projects/{}/services", environment_id);
        let results = rancher.call_api(&path).unwrap();
        let services = results["data"].as_array().unwrap();
        for service in services {
            data.push(Service{
                id:     service["id"].as_str().unwrap().to_string(),
                name:   service["name"].as_str().unwrap().to_string()
            });
        }
        return Ok(data);
    }
    /// Get a service from an environment and a pattern that should match with the service ID or
    /// the service name.
    pub fn get_one(rancher: &mut Rancher, environment_id: &str, pattern: &str) -> Result<Service, &'static str> {
        let path = format!("/projects/{}/services", environment_id);
        let results: serde_json::Value = rancher.call_api(&path).unwrap();
        let services = results["data"].as_array().unwrap();
        for service in services {
            if pattern == service["id"].as_str().unwrap() || pattern == service["name"].as_str().unwrap() {
                return Ok(Service{
                    id:     service["id"].as_str().unwrap().to_string(),
                    name:   service["name"].as_str().unwrap().to_string()
                });
            }
        }
        return Err("No service found.");
    }
}
/// This trait allows you to call Service's functions with `rancher.get_service(.....)`.
pub trait ServiceTrait {
    /// Get all services from an environment.
    fn get_services(&mut self, environment_id: &str) -> Result<Vec<Service>, &'static str>;
    /// Get a service from an environment and a pattern that should match with the service ID of
    /// the service name.
    fn get_service(&mut self, environment_id: &str, pattern: &str) -> Result<Service, &'static str>;
}

impl ServiceTrait for Rancher {
    fn get_services(&mut self, environment_id: &str) -> Result<Vec<Service>, &'static str> {
        return Service::get_all(self, environment_id);
    }

    fn get_service(&mut self, environment_id: &str, pattern: &str) -> Result<Service, &'static str> {
        return Service::get_one(self, environment_id, pattern);
    }
}

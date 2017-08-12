use serde_json;
use super::Rancher;

/// Structure that contains all data for a Rancher Container (http://rancher.com/docs/rancher/v1.6/en/api/v2-beta/api-resources/container/)
pub struct Container {
    /// Container unique ID
    pub id: String,
    /// Container name
    pub name: String,
    /// ID of the host on which the container is located.
    pub host_id: String,
    /// No idea what is for.
    pub requested_host_id: String,
    /// Result of healthcheck on the container
    pub health_state: String,
    /// Native container
    pub native_container: String,
    /// The unique identifier of the associated service.
    pub service_id: String,
    /// IDs of services associated with the container.
    pub service_ids: Vec<String>,
    /// IP address of the container.
    pub primary_ip_address: String,
    /// _
    pub allocation_state: String,
    /// Container state. It could be `stopped`, `running`, `started`.
    pub state: String,
}

impl Container {

    /// Get all containers from an environment.
    ///
    /// # Arguments
    ///
    /// * `rancher` - A mutable Rancher struct.
    /// * `environment_id` - An environment id.
    ///
    pub fn get_all(rancher: &mut Rancher, environment_id: &str) -> Result<Vec<Container>, &'static str> {
        let mut data: Vec<Container> = Vec::new();
        let path = format!("/projects/{}/containers?limit=100000", environment_id);
        let results = rancher.call_api(&path).unwrap();
        let containers = results["data"].as_array().unwrap();
        for container in containers {
            let mut services: Vec<String> = Vec::new();
            for service in container["serviceIds"].as_array().unwrap_or(&mut Vec::new()) {
                services.push(service.as_str().unwrap_or("").to_string());
            }
            data.push(Container{
                id:         container["id"].as_str().unwrap().to_string(),
                name:       container["name"].as_str().unwrap_or("").to_string(),
                host_id:    container["hostId"].as_str().unwrap_or("").to_string(),
                requested_host_id:    container["requestedHostId"].as_str().unwrap_or("").to_string(),
                health_state: container["healthState"].as_str().unwrap_or("").to_string(),
                native_container: container["nativeContainer"].as_str().unwrap_or("").to_string(),
                service_id: container["serviceId"].as_str().unwrap_or("").to_string(),
                service_ids: services,
                primary_ip_address: container["primaryIpAddress"].as_str().unwrap_or("").to_string(),
                allocation_state: container["allocationState"].as_str().unwrap_or("").to_string(),
                state: container["state"].as_str().unwrap_or("").to_string(),
            });
        }
        return Ok(data);
    }

    /// Get one container based on an environment and a pattern that should match with a container id or a container
    /// name.
    ///
    /// # Arguments
    ///
    /// * `rancher` - A mutable Rancher struct.
    /// * `environment_id` - An environment id.
    /// * `pattern` - A string that should match with a container id or a container name.
    ///
    pub fn get_one(rancher: &mut Rancher, environment_id: &str, pattern: &str) -> Result<Container, &'static str> {
        let mut name: String;
        let path = format!("/projects/{}/containers?limit=100000", environment_id);
        let results: serde_json::Value = rancher.call_api(&path).unwrap();
        let containers = results["data"].as_array().unwrap();
        for container in containers {
            let mut services: Vec<String> = Vec::new();
            for service in container["serviceIds"].as_array().unwrap_or(&mut Vec::new()) {
                services.push(service.as_str().unwrap_or("").to_string());
            }
            name = match container["name"].as_str() {
                Some(v) => v.to_string(),
                None    => String::from("")
            };
            if pattern == container["id"].as_str().unwrap() || pattern == name {
                return Ok(Container{
                    id:         container["id"].as_str().unwrap().to_string(),
                    name:       container["name"].as_str().unwrap_or("").to_string(),
                    host_id:    container["hostId"].as_str().unwrap_or("").to_string(),
                    requested_host_id:    container["requestedHostId"].as_str().unwrap_or("").to_string(),
                    health_state: container["healthState"].as_str().unwrap_or("").to_string(),
                    native_container: container["nativeContainer"].as_str().unwrap_or("").to_string(),
                    service_id: container["serviceId"].as_str().unwrap_or("").to_string(),
                    service_ids: services,
                    primary_ip_address: container["ip"].as_str().unwrap_or("").to_string(),
                    allocation_state: container["allocationState"].as_str().unwrap_or("").to_string(),
                    state: container["state"].as_str().unwrap_or("").to_string(),
                });
            }
        }
        return Err("No container found.");
    }

    /// Restart a container based on an environment id and a container id.
    ///
    /// # Arguments
    ///
    /// * `rancher` - A mutable Rancher struct.
    /// * `environment_id` - An environment id.
    /// * `container_id` - A container id.
    ///
    pub fn restart(rancher: &mut Rancher, environment_id: &str, container_id: &str) -> bool {
        let path = format!("/projects/{}/containers/{}?action=restart", environment_id, container_id);
        match rancher.post_api_without_data(&path) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }

    /// Start a container based on an environment id and a container id.
    ///
    /// # Arguments
    ///
    /// * `rancher` - A mutable Rancher struct.
    /// * `environment_id` - An environment id.
    /// * `container_id` - A container id.
    ///
    pub fn start(rancher: &mut Rancher, environment_id: &str, container_id: &str) -> bool {
        let path = format!("/projects/{}/containers/{}?action=start", environment_id, container_id);
        match rancher.post_api_without_data(&path) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }

    /// Stop a container based on an environment id and a container id.
    ///
    /// # Arguments
    ///
    /// * `rancher` - A mutable Rancher struct.
    /// * `environment_id` - An environment id.
    /// * `container_id` - A container id.
    ///
    pub fn stop(rancher: &mut Rancher, environment_id: &str, container_id: &str) -> bool {
        let path = format!("/projects/{}/containers/{}?action=stop", environment_id, container_id);
        match rancher.post_api_without_data(&path) {
            Ok(_) => {
                return true;
            },
            Err(_) => {
                return false;
            }
        }
    }

}

/// This trait allows you to call Container's functions with `rancher.restart_container(.....)`
pub trait ContainerTrait {
    /// Get all containers from an environment.
    fn get_containers(&mut self, environment_id: &str) -> Result<Vec<Container>, &'static str>;
    /// Get one container based on an environment and a pattern that should match with a container id or a container name.
    fn get_container(&mut self, environment_id: &str, pattern: &str) -> Result<Container, &'static str>;
    /// Restart a container based on an environment id and a container id.
    fn restart_container(&mut self, environment_id: &str, container_id: &str) -> bool;
    /// Start a container based on an environment id and a container id.
    fn start_container(&mut self, environment_id: &str, container_id: &str) -> bool;
    /// Stop a container based on an environment id and a container id.
    fn stop_container(&mut self, environment_id: &str, container_id: &str) -> bool;
}

impl ContainerTrait for Rancher {
    fn get_containers(&mut self, environment_id: &str) -> Result<Vec<Container>, &'static str> {
        return Container::get_all(self, environment_id);
    }
    fn get_container(&mut self, environment_id: &str, pattern: &str) -> Result<Container, &'static str> {
        return Container::get_one(self, environment_id, pattern);
    }
    fn restart_container(&mut self, environment_id: &str, container_id: &str) -> bool {
        return Container::restart(self, environment_id, container_id);
    }
    fn start_container(&mut self, environment_id: &str, container_id: &str) -> bool {
        return Container::start(self, environment_id, container_id);
    }
    fn stop_container(&mut self, environment_id: &str, container_id: &str) -> bool {
        return Container::restart(self, environment_id, container_id);
    }
 }

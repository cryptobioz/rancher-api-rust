use serde_json;
use super::Rancher;
use std::collections::HashMap;

/// Struct that contains all data for a Rancher Host
/// (http://rancher.com/docs/rancher/v1.6/en/api/v2-beta/api-resources/host/).
#[derive(PartialEq, Debug)]
pub struct Host {
    /// Host unique ID.
    pub id: String,
    /// Hostname.
    pub hostname: String,
    /// IP address of the host.
    pub agent_ip_address: String,
    /// State of the agent.
    pub agent_state: String,
    /// Labels associated with the host.
    pub labels: HashMap<String, String>,
}

impl Host {
    /// Get all hosts from an environment.
    pub fn get_all(rancher: &mut Rancher, environment_id: &str) -> Result<Vec<Host>, &'static str> {
        let mut data: Vec<Host> = Vec::new();
        let path = format!("/projects/{}/hosts", environment_id);
        let results = rancher.call_api(&path).unwrap();
        let hosts = results["data"].as_array().unwrap();
        for host in hosts {
            let mut labels: HashMap<String, String> = HashMap::new();
            if host["labels"].as_object().is_some() {
                for (label, value) in host["labels"].as_object().unwrap() {
                    labels.insert(label.to_string(), value.as_str().unwrap_or("").to_string().replace("\"", ""));
                }
            }

            data.push(Host{
                id: host["id"].as_str().unwrap().to_string(),
                hostname: host["hostname"].as_str().unwrap().to_string(),
                agent_ip_address: host["agentIpAddress"].as_str().unwrap().to_string(),
                agent_state: host["agentState"].as_str().unwrap_or("").to_string(),
                labels: labels,
            });
        }
        return Ok(data);
    }
    /// Get an host from an environment and a pattern that should match with the host ID or the
    /// hostname.
    pub fn get_one(rancher: &mut Rancher, environment_id: &str, pattern: &str) -> Result<Host, &'static str> {
        let path = format!("/projects/{}/hosts", environment_id);
        let results: serde_json::Value = rancher.call_api(&path).unwrap();
        let hosts = results["data"].as_array().unwrap();
        for host in hosts {
            let mut labels: HashMap<String, String> = HashMap::new();
            if host["labels"].as_object().is_some() {
                for (label, value) in host["labels"].as_object().unwrap() {
                    labels.insert(label.to_string(), value.as_str().unwrap_or("").to_string());
                }
            }

            if pattern == host["id"].as_str().unwrap() || pattern == host["hostname"].as_str().unwrap() {
                return Ok(Host{
                    id: host["id"].as_str().unwrap().to_string(),
                    hostname: host["hostname"].as_str().unwrap().to_string(),
                    agent_ip_address: host["agentIpAddress"].as_str().unwrap().to_string(),
                    agent_state: host["agentState"].as_str().unwrap_or("").to_string(),
                    labels: labels
                });
            }
        }
        return Err("No host found.");
    }
}

/// This trait allows you to call Host's functions with `rancher.get_host(.......)`.
pub trait HostTrait {
    /// Get all hosts from an environment.
    fn get_hosts(&mut self, environment_id: &str) -> Result<Vec<Host>, &'static str>;
    /// Get an host from an environment and a pattern that should match with the host ID or the
    /// hostname.
    fn get_host(&mut self, environment_id: &str, pattern: &str) -> Result<Host, &'static str>;
}

impl HostTrait for Rancher {
    fn get_hosts(&mut self, environment_id: &str) -> Result<Vec<Host>, &'static str> {
        return Host::get_all(self, environment_id);
    }

    fn get_host(&mut self, environment_id: &str, pattern: &str) -> Result<Host, &'static str> {
        return Host::get_one(self, environment_id, pattern);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_all() {
        let expected_result = vec![
            Host{
                id: String::from("1h10"),
                hostname: String::from("fake-10-10-10-10.internal"),
                agent_ip_address: String::from("10.10.10.10"),
                agent_state: String::from("active"),
                labels: [
                    (String::from("database"), String::from("true")),
                    (String::from("monitoring"), String::from("false")),
                ].iter().cloned().collect(),
            },
            Host{
                id: String::from("1h11"),
                hostname: String::from("fake-10-10-10-11.internal"),
                agent_ip_address: String::from("10.10.10.11"),
                agent_state: String::from("disconnected"),
                labels: [
                    (String::from("database"), String::from("false")),
                ].iter().cloned().collect(),
            }

        ];
        let api_response = r#"{
            "data": [
                {
                    "id": "1h10",
                    "hostname": "fake-10-10-10-10.internal",
                    "agentIpAddress": "10.10.10.10",
                    "agentState": "active",
                    "labels": {
                        "database": "true",
                        "monitoring": "false"
                    }
                },
                {
                    "id": "1h11",
                    "hostname": "fake-10-10-10-11.internal",
                    "agentIpAddress": "10.10.10.11",
                    "agentState": "disconnected",
                    "labels": {
                        "database": "false"
                    }
                }
            ]
        }"#;
        let mut data: Vec<Host> = Vec::new();
        let results: serde_json::Value = serde_json::from_str(api_response).unwrap();
        let hosts = results["data"].as_array().unwrap();
        for host in hosts {
            let mut labels: HashMap<String, String> = HashMap::new();
            if host["labels"].as_object().is_some() {
                for (label, value) in host["labels"].as_object().unwrap() {
                    labels.insert(label.to_string(), value.as_str().unwrap_or("").to_string().replace("\"", ""));
                }
            }

            data.push(Host{
                id: host["id"].as_str().unwrap().to_string(),
                hostname: host["hostname"].as_str().unwrap().to_string(),
                agent_ip_address: host["agentIpAddress"].as_str().unwrap().to_string(),
                agent_state: host["agentState"].as_str().unwrap_or("").to_string(),
                labels: labels,
            });
        }

        assert_eq!(expected_result.eq(&data), true, "Expected {:?}, got {:?}", expected_result, data);
    }

    #[test]
    fn get_one() {
        let expected_result = Host{
            id: String::from("1h11"),
            hostname: String::from("fake-10-10-10-11.internal"),
            agent_ip_address: String::from("10.10.10.11"),
            agent_state: String::from("disconnected"),
            labels: [
                (String::from("database"), String::from("false")),
            ].iter().cloned().collect(),
        };
        let api_response = r#"{
            "data": [
                {
                    "id": "1h10",
                    "hostname": "fake-10-10-10-10.internal",
                    "agentIpAddress": "10.10.10.10",
                    "agentState": "active",
                    "labels": {
                        "database": "true",
                        "monitoring": "false"
                    }
                },
                {
                    "id": "1h11",
                    "hostname": "fake-10-10-10-11.internal",
                    "agentIpAddress": "10.10.10.11",
                    "agentState": "disconnected",
                    "labels": {
                        "database": "false"
                    }
                }
            ]
        }"#;
        let pattern = "fake-10-10-10-11.internal";
        let mut data: Option<Host> = None;
        let results: serde_json::Value = serde_json::from_str(api_response).unwrap();
        let hosts = results["data"].as_array().unwrap();
        for host in hosts {
            let mut labels: HashMap<String, String> = HashMap::new();
            if host["labels"].as_object().is_some() {
                for (label, value) in host["labels"].as_object().unwrap() {
                    labels.insert(label.to_string(), value.as_str().unwrap_or("").to_string().replace("\"", ""));
                }
            }

            if pattern == host["id"].as_str().unwrap() || pattern == host["hostname"].as_str().unwrap() {
                data = Some(Host{
                    id: host["id"].as_str().unwrap().to_string(),
                    hostname: host["hostname"].as_str().unwrap().to_string(),
                    agent_ip_address: host["agentIpAddress"].as_str().unwrap().to_string(),
                    agent_state: host["agentState"].as_str().unwrap_or("").to_string(),
                    labels: labels,
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

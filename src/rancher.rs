use *;
use curl::easy::{Easy, List};
use std;
use std::fs;
use std::io::prelude::*;
use regex::Regex;

/// Struct that contains basic informations needed to use the Rancher API.
pub struct Rancher {
    /// URL of the Rancher API.
    url: String,
    /// Access key of the Rancher API.
    access_key: String,
    /// Secret key of the Rancher API.
    secret_key: String
}

impl Rancher {
    /// Return a Rancher struct filled with informations provided as arguments.
    pub fn new(url: String, access_key: String, secret_key: String) -> Rancher {
        let rancher: Rancher = Rancher {
            url: url,
            access_key: access_key,
            secret_key: secret_key
        };
        return rancher;
    }
    /// Return a Rancher struct filled with informations stored into the file provided as argument.
    pub fn new_from_file(pattern: &str) -> Result<Rancher, &'static str> {
        let mut rancher_dir = std::env::home_dir().unwrap().to_str().unwrap().to_string();
        rancher_dir.push_str("/.rancher/");
        let mut files: Vec<String> = Vec::new();
        let mut f: String;
        for path in fs::read_dir(rancher_dir).unwrap() {
            f = path.unwrap().path().to_str().unwrap().to_string();
            if f.contains(&*pattern) {
                files.push(f);
            }
        }
        if files.len() != 1 {
            return Err("Can't retrieve the config file.");
        }
        let mut file = fs::File::open(files[0].to_owned()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut config: serde_json::Value = serde_json::from_str(&*content).unwrap();
        // Set URL
        if Regex::new(r"/schemas$").unwrap().is_match(config["url"].as_str().unwrap()) {
            let url = config["url"].as_str().unwrap().replace("/schemas", "");
            config["url"] = url.into();
        }
        let rancher: Rancher = Rancher {
            url: config["url"].as_str().unwrap().to_string(),
            access_key: config["accessKey"].as_str().unwrap().to_string(),
            secret_key: config["secretKey"].as_str().unwrap().to_string()
        };
        return Ok(rancher);
    }
    /// Send a GET request to the Rancher API.
    pub fn call_api(&mut self, path: &str) -> Result<serde_json::Value, &'static str> {
        let mut dst = String::new();
        let mut handle = Easy::new();
        let mut url = self.url.clone();
        url.push_str(path);
        handle.url(&url).unwrap();
        let credentials = format!("{}:{}", self.access_key, self.secret_key);
        let auth = format!("Authorization: Basic {}", base64::encode(&credentials));
        let mut list = List::new();
        list.append(&*auth).unwrap();
        handle.http_headers(list).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                dst.push_str(std::str::from_utf8(data).unwrap());
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }
        let results = serde_json::from_str(&*dst).unwrap();
        return Ok(results);
    }

    /// Send a POST request to the Rancher API with data.
    pub fn post_api(&mut self, path: &str, data: &str) -> Result<serde_json::Value, &'static str> {
        let mut dst = String::new();
        let mut handle = Easy::new();
        let mut url = self.url.clone();
        url.push_str(path);
        handle.url(&url).unwrap();
        handle.post(true).unwrap();
        handle.post_field_size(data.len() as u64).unwrap();
        let credentials = format!("{}:{}", self.access_key, self.secret_key);
        let auth = format!("Authorization: Basic {}", base64::encode(&credentials));
        let content_type = format!("Content-Type: application/json");
        let mut list = List::new();
        list.append(&*auth).unwrap();
        list.append(&*content_type).unwrap();
        handle.http_headers(list).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.read_function(|buf| {
                Ok(data.as_bytes().read(buf).unwrap_or(0))
            }).unwrap();
            transfer.perform().unwrap();
        }
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|body| {
                dst.push_str(std::str::from_utf8(body).unwrap());
                Ok(body.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let results: serde_json::Value = serde_json::from_str(&*dst).unwrap();
        /*
        let base_type = results["baseType"].as_str().unwrap().to_owned();
        if base_type == "error" {
            return Err("Something goes wrong...");
        } else {
            return Ok(results);
        }*/
        return Ok(results);
    }
    /// Send a POST request to the Rancher API without data.
    pub fn post_api_without_data(&mut self, path: &str) -> Result<serde_json::Value, &'static str> {
        let mut dst = String::new();
        let mut handle = Easy::new();
        let mut url = self.url.clone();
        url.push_str(path);
        handle.url(&url).unwrap();
        handle.post(true).unwrap();
        let credentials = format!("{}:{}", self.access_key, self.secret_key);
        let auth = format!("Authorization: Basic {}", base64::encode(&credentials));
        let mut list = List::new();
        list.append(&*auth).unwrap();
        handle.http_headers(list).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|body| {
                dst.push_str(std::str::from_utf8(body).unwrap());
                Ok(body.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let results: serde_json::Value = serde_json::from_str(&*dst).unwrap();
        let base_type = results["baseType"].as_str().unwrap().to_owned();
        if base_type == "error" {
            return Err("Error: Something goes wrong...")
        } else {
            return Ok(results);
        }
    }
}


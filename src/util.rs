extern crate serde_json;
extern crate curl;
extern crate base64;
extern crate tabwriter;
extern crate regex;
use serde_json::{Value, Error};
use std::fs;
use std::io::prelude::*;
use std::io::{stdout, Write};
use curl::easy::{Easy, List};
use std;
use tabwriter::TabWriter;
use regex::Regex;

pub fn call_api(config: &serde_json::Value, path: &str) -> Result<serde_json::Value, &'static str> {
    
    let mut dst = String::new();
    let mut handle = Easy::new();
    let mut url = config["url"].as_str().unwrap().to_string();
    url.push_str(path);
    handle.url(&url).unwrap(); 
    let credentials = format!("{}:{}", config["accessKey"].as_str().unwrap(), config["secretKey"].as_str().unwrap());
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

pub fn get_config(config: &str) -> Result<serde_json::Value, &'static str> {
    let mut rancher_dir = std::env::home_dir().unwrap().to_str().unwrap().to_string();
    rancher_dir.push_str("/.rancher/");
    let mut files: Vec<String> = Vec::new();
    let mut f: String;
    for path in fs::read_dir(rancher_dir).unwrap() {
        f = path.unwrap().path().to_str().unwrap().to_string();
        if f.contains(config) {
            files.push(f);
        }
    }
    if files.len() != 1 {
        return Err("Can't retrieve the config file.");
    }
    let mut file = fs::File::open(files[0].to_owned()).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let mut config: Value = serde_json::from_str(&*content).unwrap();
    // Set URL
    if Regex::new(r"/schemas$").unwrap().is_match(config["url"].as_str().unwrap()) {
        let url = config["url"].as_str().unwrap().replace("/schemas", "");
        config["url"] = url.into();
    }
    return Ok(config);
}


pub fn display(data: String) {
    let mut tw = TabWriter::new(vec![]);
    write!(&mut tw, "{}", data).unwrap();
    tw.flush().unwrap();
    let written = String::from_utf8(tw.into_inner().unwrap()).unwrap();
    print!("{}", written);
}

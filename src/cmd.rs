extern crate serde_json;
extern crate clap;

use clap::Arg;
use serde_json::{Value, Error};
use std;
use util;
use environment;

pub fn list(config: &serde_json::Value, arguments: clap::ArgMatches) {
    /*
    if arguments.value_of("env").is_none() {
        let results: Value = util::call_api(&config, "/projects").unwrap();
        let projects = results["data"].as_array().unwrap();
        let mut data: String = String::from("ID\tNAME\n");
        for project in projects {
            data.push_str(&*format!("{}\t{}\n", project["id"].as_str().unwrap(), project["name"].as_str().unwrap()));
        }
        util::display(data);
    } else if arguments.value_of("host").is_none() {
        let project_id = self::get_env_from_arg(config, arguments).unwrap();
        let path = format!("/projects/{}/hosts", project_id);
        let results: Value = util::call_api(&config, &*path).unwrap();
        let hosts = results["data"].as_array().unwrap();
        let mut data: String = String::from("ID\tHOSTNAME\n");
        for host in hosts {
            data.push_str(&*format!("{}\t{}\n", host["id"].as_str().unwrap(), host["hostname"].as_str().unwrap()));
        }
        util::display(data);
    } else if arguments.value_of("container").is_none() {
        let project_id = self::get_env_from_arg(config, arguments).unwrap();
        let host_id = self::get_host_from_arg(config, arguments, &project_id).unwrap();
        let path = format!("/projects/{}/hosts/{}/containers", project_id, host_id);
        let results: Value = util::call_api(&config, &*path).unwrap();
        println!("{:?}", results);
        let containers = results["data"].as_array().unwrap();
        let mut data: String = String::from("ID\tHOSTNAME\n");
        for container in containers {
            data.push_str(&*format!("{}\t{}\n", container["id"].as_str().unwrap(), container["id"].as_str().unwrap()));
        }
        util::display(data);
    }
    */
    
    match arguments.value_of("ARGUMENT").unwrap(){
        "env" => {
            let mut data: String = String::from("ID\tNAME\n");
            let projects = environment::get_array(config).unwrap();
            for project in projects {
                data.push_str(&*format!("{}\t{}\n", project.id, project.name));
            }
            util::display(data);
        },
        "hosts" => {
        },
        _ => println!("")
    }
}

pub fn list_environments(config: &serde_json::Value, arguments: &clap::ArgMatches) {
    let results: Value = util::call_api(&config, "/projects").unwrap();
    let projects = results["data"].as_array().unwrap();
    let mut data: String = String::from("ID\tNAME\n");
    for project in projects {
        data.push_str(&*format!("{}\t{}\n", project["id"].as_str().unwrap(), project["name"].as_str().unwrap()));
    }
    util::display(data);
}

pub fn list_hosts(config: &serde_json::Value, arguments: &clap::ArgMatches) {
    let p = self::get_env_from_arg(config, arguments);
    if p.is_ok() {
        let project_id = p.unwrap();
        let path = format!("/projects/{}/hosts", project_id);
        let results: Value = util::call_api(&config, &*path).unwrap();
        let projects = results["data"].as_array().unwrap();
        let mut data: String = String::from("ID\tHOSTNAME\n");
        for project in projects {
            data.push_str(&*format!("{}\t{}\n", project["id"].as_str().unwrap(), project["hostname"].as_str().unwrap()));
        }
        util::display(data);
    } else {
        let mut data: String = String::from("ID\tENVIRONMENT\tHOSTNAME\n");
        let r: Value = util::call_api(&config, "/projects").unwrap();
        let projects = r["data"].as_array().unwrap();
        for project in projects {
            let path = format!("/projects/{}/hosts", project["id"].as_str().unwrap());
            let results: Value = util::call_api(&config, &*path).unwrap();
            let hosts = results["data"].as_array().unwrap();
            for host in hosts {
                data.push_str(&*format!("{}\t{}\t{}\n", host["id"].as_str().unwrap(), project["name"].as_str().unwrap(), host["hostname"].as_str().unwrap()));
            }
        }
        util::display(data);
    }
    
}

fn get_env_from_arg(config: &serde_json::Value, arguments: &clap::ArgMatches) -> Result<String, &'static str> {
    let env = match arguments.value_of("env") {
        Some(v) => v,
        None => return Err("No environment.")
    };
    let results: Value = util::call_api(&config, "/projects").unwrap();
    let projects = results["data"].as_array().unwrap();
    let mut data: String = String::from("ID\tNAME\n");
    let mut project_id: String;
    for project in projects {
        if env == project["id"].as_str().unwrap() || env == project["name"].as_str().unwrap() {
            let project_id = project["id"].as_str().unwrap().to_string();
            return Ok(project_id);
        }
    }
    return Err("No project found.");
}

fn get_host_from_arg(config: &serde_json::Value, arguments: &clap::ArgMatches, project_id: &String) -> Result<String, &'static str> {
    let h = arguments.value_of("host").unwrap();
    let path = format!("/projects/{}/hosts", project_id);
    let results: Value = util::call_api(&config, &*path).unwrap();
    let hosts = results["data"].as_array().unwrap();
    let mut data: String = String::from("ID\tHOSTNAME\n");
    for host in hosts {
        if h == host["id"].as_str().unwrap() || h == host["hostname"].as_str().unwrap() {
            let host_id = host["id"].as_str().unwrap().to_string();
            return Ok(host_id);
        }
    }
    return Err("No host found.");
}

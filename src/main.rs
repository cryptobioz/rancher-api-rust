extern crate clap;
extern crate rancher;

use rancher::*;
use clap::{Arg, App};

fn main(){
    let matches = App::new("Usable Rancher CLI")
        .version("0.1")
        .author("Leo Depriester <leo.depriester@exadot.fr>")
        .arg(Arg::with_name("CONFIG_FILE")
            .help("Name of the config file into ~/.rancher")
            .required(true)
            .index(1))
        .arg(Arg::with_name("COMMAND")
            .possible_values(&["ls"])
            .required(true)
            .index(2)) 
        .arg(Arg::with_name("ARGUMENT")
             .index(3))
        .arg(Arg::with_name("env")
            .short("e")
            .long("env")
            .value_name("ENVIRONMENT")
            .takes_value(true)
            .help("Set environment."))
        .arg(Arg::with_name("host")
            .short("h")
            .long("host")
            .value_name("HOST")
            .takes_value(true)
            .help("Set host."))
        .arg(Arg::with_name("container")
             .short("c")
             .long("container")
             .value_name("CONTAINER")
             .takes_value(true)
             .help("Set container."))
        .get_matches();


    let pattern = matches.value_of("CONFIG_FILE").unwrap();

    let mut rancher = match Rancher::new_from_file(&pattern) {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let monitoring_env_id = rancher.get_environment("monitoring").unwrap().id;


    for container in rancher.get_containers(&monitoring_env_id).unwrap() {
        println!("{} -> {}", container.id, container.name);
    }

}

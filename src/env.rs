use std::{
    error::Error,
    fmt,
    fs,
    net::{SocketAddr, ToSocketAddrs},
};

use yaml_rust::YamlLoader;


#[derive(Debug)]
pub enum EnvLogLevel {
    Test,
    Stage,
    Prod,
}

impl fmt::Display for EnvLogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Environment {
    pub listening_port: String,
    pub hosts: Vec<SocketAddr>,
    pub env_level: EnvLogLevel, // for logging
}

impl Environment {
    pub fn init_env(path_to_conf_yaml: String) -> Result<Environment, Box<dyn Error>> {
        match read_yaml_file(path_to_conf_yaml) {
            Some(y) => {
                let conf_details: (String, Vec<SocketAddr>, EnvLogLevel) = read_config(&y)?;
                
                print_details(&conf_details);

                Ok(Environment {
                        listening_port: conf_details.0,
                        hosts: conf_details.1,
                        env_level: conf_details.2,
                    })
            },
            None => {
                panic!("Error while reading in YAML config file!");
            },
        }
    }
}

fn read_yaml_file(yaml_path: String) -> Option<String> {
    let contents = fs::read_to_string(yaml_path)
        .expect("Error reading in the YAML file!");
    Some(contents)
}

pub fn read_config(yaml_string: &String) -> Result<(String, Vec<SocketAddr>, EnvLogLevel), Box<dyn Error>> {
    let yaml_obj: yaml_rust::Yaml = YamlLoader::load_from_str(yaml_string)?[0].clone();

    let listening_port: String = yaml_obj["listening-port"]
        .as_i64()
        .expect("\nError reading in the listening port!\n")
        .to_string();

    let hosts_yaml: Vec<yaml_rust::Yaml> = yaml_obj["hosts"]
        .as_vec()
        .expect("\nError reading the hosts! Make sure hosts have been specified in your YAML file.\n")
        .clone();

    let mut hosts: Vec<SocketAddr> = vec![];
    for h in hosts_yaml {

        let host_string: String = h
            .as_str()
            .expect("\nError converting the hosts from YAML to String!\n")
            .to_string();

        hosts.push(host_string.to_socket_addrs()
            .unwrap()
            .next()
            .unwrap());
    }

    let log_level: EnvLogLevel = match yaml_obj["env"]
        .as_str()
        .expect("\nError reading the log level!\n") {
            "test" => EnvLogLevel::Test,
            "stage" => EnvLogLevel::Stage,
            "prod" => EnvLogLevel::Prod,
            _ => EnvLogLevel::Test,
        };

    Ok((listening_port, hosts, log_level))
}

fn print_details(env_details: &(String, Vec<SocketAddr>, EnvLogLevel)) {
    println!(
        "Listening on port {}\nHosts to balance {:?}\nEnvironment level: {}",
        env_details.0,
        env_details.1,
        env_details.2,
    );
}

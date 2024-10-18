use core::fmt;
use std::{
    error::Error,
    net::{SocketAddr, TcpStream}
};


#[derive(Debug, Clone, PartialEq)]
pub enum HostHealth {
    Active,
    Inactive,
}

impl fmt::Display for HostHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Host {
    pub hostname: SocketAddr,
    pub health: HostHealth,
}

impl Host {
    pub fn health_check(&mut self) {
        match TcpStream::connect(&self.hostname) {
            Ok(_x) => {
                self.health = HostHealth::Active;
            },
            Err(_x) => {
                self.health = HostHealth::Inactive;
            },
        }
    }
}


pub struct Hosts {
    pub hosts: Vec<Host>,

}

impl Hosts {
    pub fn init_client(hosts: &Vec<SocketAddr>) -> Result<Hosts, Box<dyn Error>> {
        let mut host_objects: Vec<Host> = vec![];
        for host_name in hosts {
            let mut new_host_obj: Host = Host {
                hostname: host_name.clone(),
                health: HostHealth::Inactive
            };
            new_host_obj.health_check();
            host_objects.push(new_host_obj);
        }

        Ok(Hosts {
                hosts: host_objects,
            })
    }
}
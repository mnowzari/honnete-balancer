use core::fmt;
use std::{error::Error, io::{Read, Write}, net::TcpStream};

use clap::builder::Str;

use crate::queue::Queue;

#[derive(Debug)]
pub enum HostHealth {
    Active,
    Inactive,
}

impl fmt::Display for HostHealth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Host {
    pub hostname: String,
    pub health: HostHealth,
}


pub struct Client {
    pub hosts: Vec<Host>,

}

impl Client {
    pub fn init_client(hosts: &Vec<String>) -> Result<Client, Box<dyn Error>> {
        let mut host_objects: Vec<Host> = vec![];
        for host_name in hosts {
            host_objects.push(
                Host {
                    hostname: host_name.clone(),
                    health: HostHealth::Inactive // assume hosts are inactive until proven otherwise
                });
        }

        Ok(Client {
                hosts: host_objects,
            })
    }

    pub fn health_check(&mut self) -> Result<(), Box<dyn Error>> {
        // intermittent health check to make sure hosts are up
        // println!("\nChecking health of hosts:\n");
        for host in &mut self.hosts {
            match TcpStream::connect(&host.hostname) {
                Ok(x) => {
                    host.health = HostHealth::Active;
                    // println!("{}", &host.health);
                },
                Err(x) => {
                    host.health = HostHealth::Inactive;
                    // println!("{}", &host.health);
                },
            }
        }
        Ok(())
    }

    // pub fn round_robin(&self, Queue: Arc<Mutex<Queue>>) {

    // }
}

fn print_init_details() {

}
mod threadpool;
mod queue;
mod client;
mod listener;
mod balancer;
mod env;

use std::{
    error::Error, sync::{Arc, Mutex}
};

use futures::executor::block_on;

use env::Environment;
use queue::Queue;
use client::Client;

use clap::Parser;


#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    propagate_version = true
)]
pub struct InputArguments {
    #[arg(long, required = true)]
    pub conf: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>>{
    println!("\nStarting Honnete load balancer\n");
    // get cli arguments
    let args: InputArguments = InputArguments::parse();
    // init env and logging and read in config settings
    println!("Initializing environment\n");
    let environ: Environment = Environment::init_env(args.conf.unwrap())?;
    // init in-memory requests queue
    let request_queue: Arc<Mutex<Queue>> = Queue::new()?;
    // init client instance
    let mut client_instance: Client = Client::init_client(&environ.hosts)?;
    client_instance.health_check();
    // init listener 
    let listener_future = listener::init_listeners(&request_queue, &environ.listening_port);

    // init request balancer

    block_on(listener_future);
    Ok(())
}

mod threadpool;
mod queue;
mod connections;
mod listener;
mod handler;
mod env;

use std::{
    error::Error,
    sync::{Arc, Mutex}
};

use env::Environment;
use queue::Queue;
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
    // init connections to hosts


    // init listener 
    listener::listen_for_requests(
        &request_queue,
        &environ.listening_port
    );

    Ok(())
}

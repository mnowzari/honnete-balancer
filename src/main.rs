use std::{error::Error, sync::{Arc, Mutex}};

use env::Environment;
use queue::Queue;
use clap::Parser;

mod threadpool;
mod queue;
mod connections;
mod listener;
mod handler;
mod env;


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
    // get cli arguments
    let args = InputArguments::parse();

    // init env and logging and read in config settings
    let environ: Environment = Environment::init_env(args.conf.unwrap())?;

    // init in-memory requests queue
    let request_queue = Arc::new(Mutex::new(Queue::new()?));

    // init listener 
    listener::listen_for_requests(&request_queue);

    // init connections

    Ok(())
}

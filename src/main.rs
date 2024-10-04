mod threadpool;
mod queue;
mod client;
mod listener;
mod balancer;
mod env;

use std::{
    error::Error, sync::{Arc, Mutex}
};

use env::Environment;
use queue::Queue;
use client::Client;

use clap::Parser;
use threadpool::ThreadPool;


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

    // initialize two threads, one for the listener and one for the balancer
    let main_thread_pool: ThreadPool = ThreadPool::new(2).unwrap();

    let req_q_arc = request_queue.clone();
    main_thread_pool.execute(move || {
        listener::init_listeners(
            &req_q_arc,
            &environ.listening_port
        );
    });

    let req_q_arc_two = request_queue.clone();
    main_thread_pool.execute(move || {
        balancer::lb_round_robin(&req_q_arc_two, &mut client_instance);
    });

    println!("...listeners active!\n\nCtrl-C to terminate this process.");
    Ok(())
}

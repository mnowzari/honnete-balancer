use std::{
    io::{BufRead,BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex}
};

// use regex::Regex;
use num_cpus;

use crate::{
    queue::Queue,
    threadpool::ThreadPool
};


fn validate_request_line(request: String) -> Option<String> {
    // ensure http request is valid
    // let re: Regex = Regex::new(r"")
    //     .unwrap();
    // if re.is_match(&request) {
    //     Some(request)
    // } else {
    //     None
    // }
    Some(request)
}

pub fn enqueue_requests(mut stream: TcpStream, request_queue: Arc<Mutex<Queue>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();
    
    match validate_request_line(request_line) {
        Some(r) => {
            // println!("{}", r);
            request_queue
                .lock()
                .unwrap()
                .enqueue(r);
        },
        None => {},
    }
}

pub fn listen_for_requests(request_queue: &Arc<Mutex<Queue>>, port: &String) {
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .expect("Error initializing TcpListener!");
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()/2).unwrap();

    for stream in listener.incoming() {

        let request_queue_arcref = request_queue.clone();
        pool.execute(move || {
            enqueue_requests(stream.unwrap(), request_queue_arcref);
        });
    }
}
use std::{
    io::{BufReader, BufRead, Write},
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
    // None
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

            println!("{}", r);
            
            request_queue
                .lock()
                .unwrap()
                .enqueue(r);
        },
        None => {
            // send 404 if incoming request is not valid
            let contents: String = String::from("Request is invalid!");
            let length: usize = contents.len();

            let response: String =
                format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: {length}\r\n\r\n{contents}");

            stream.write_all(response.as_bytes()).unwrap();
        },
    }
}

fn listener(request_queue: Arc<Mutex<Queue>>, port: String) {
    let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .expect("Error initializing TcpListener!");

    for stream in listener.incoming() {
        enqueue_requests(stream.unwrap(), request_queue.clone());
    }
}

pub async fn init_listeners(request_queue: &Arc<Mutex<Queue>>, port: &String) {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()/2).unwrap();

    let rq_arc_ref: Arc<Mutex<Queue>> = request_queue.clone();
    let port_number: String = port.clone();

    pool.execute(move || {
        listener(rq_arc_ref, port_number);
    });
}
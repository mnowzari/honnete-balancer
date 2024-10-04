use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use num_cpus;

use crate::{
    queue::{Queue, Request},
    threadpool::ThreadPool
};


fn validate_request_line(request: String) -> Option<String> {
    // ensure http request is valid...somehow
    Some(request)
    // None
}

// Streams will stay open as long as they live in the queue
// This makes sense because we want the stream to be alive
// until it is taken care of and a response is written
// back to it. The thread doesn't block, I don't think
pub fn enqueue_requests(mut stream: TcpStream, request_queue: Arc<Mutex<Queue>>) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line: String = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    match validate_request_line(request_line) {
        Some(r) => {

            let contents: String = String::from("");
            let length: usize = r.len();
            let request_to_send: String =
                format!("{}\r\nContent-Length: {length}\r\n\r\n{contents}", &r);
            
            println!("\n=== Incoming request: {}", &request_to_send);

            let request_obj: Request = Request {
                request_data: request_to_send,
                stream, // we need to save this stream to write back the request result to it
            };

            request_queue
                .lock()
                .unwrap()
                .enqueue(request_obj);

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

// ==============================================================================================================
// not sure which 'model' to use below:

// this function has a single TcpListener and kicks off
// many enqueue_requests threads as they come into a single listener
pub fn init_listeners(request_queue: &Arc<Mutex<Queue>>, port: &String) {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()/2).unwrap();
    
    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .expect("Error initializing TcpListener");

    for stream in listener.incoming() {
        let rq_arc_ref: Arc<Mutex<Queue>> = request_queue.clone();
        pool.execute(move || {
            enqueue_requests(stream.unwrap(), rq_arc_ref);
        });
    }
}

// these two functions work by kicking off multiple TcpListener threads
// so each thread has its own TcpListener on the specific port that will
// have its own enqueue_requests() function:

// fn listener(request_queue: Arc<Mutex<Queue>>, port: String) {
//     let listener: TcpListener = TcpListener::bind(format!("127.0.0.1:{}", port))
//         .expect("Error initializing TcpListener!");

//     for stream in listener.incoming() {
//         enqueue_requests(stream.unwrap(), request_queue.clone());
//     }
// }

// pub async fn init_listeners(request_queue: &Arc<Mutex<Queue>>, port: &String) {
//     let pool: ThreadPool = ThreadPool::new(num_cpus::get()/2).unwrap();

//     let rq_arc_ref: Arc<Mutex<Queue>> = request_queue.clone();
//     let port_number: String = port.clone();

//     pool.execute(move || {
//         listener(rq_arc_ref, port_number);
//     });
// }

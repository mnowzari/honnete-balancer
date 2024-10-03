use std::{error::Error, io::Write, sync::{Arc, Mutex}};

use crate::{client::Client, queue::{Queue, Request}, threadpool::ThreadPool};


// pub struct Balancer {
//     pub client_inst: Client,
// }

// impl Balancer {
//     fn new(client: &mut Client) -> Result<Balancer, Box<dyn Error>> {

//         Ok(Balancer {
//             client_inst: client,
//         })
//     }
// }

fn handler(request_object: &mut Request) {

    let contents: String = String::from("Request has been handled!");
    let length: usize = contents.len();
    let response: String =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

    request_object.stream.write_all(response.as_bytes()).unwrap();
}

pub fn test_balance(request_queue: &Arc<Mutex<Queue>>, client: &mut Client) {
    let pool: ThreadPool = ThreadPool::new(4).unwrap();

    loop {
        let request_object = request_queue.lock().unwrap().next();
        // println!("{}", request_object.as_ref().expect("BAD").request_data);
        match request_object {
            Some(mut r) => {
                pool.execute(move || {
                    handler(&mut r);
                });
            },
            None => {},
        }
    }
}
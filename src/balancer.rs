use core::time::Duration;
use std::{
    io::Write, ops::Deref, sync::{Arc, Mutex}, thread::sleep
};

use crate::{
    client::{Client, Host},
    queue::{Queue, Request},
    threadpool::ThreadPool
};

const THREAD_SLEEP: u64 = 3;

fn test_handler(request_object: &mut Request, host: &Host) {

    let contents: String = String::from("Request has been handled!");
    let length: usize = contents.len();
    let response: String =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

    request_object.stream.write_all(response.as_bytes()).unwrap();
}

// fn select_destination_host(hosts: &Vec<Host>) -> Result<(), Box<dyn Error>> {
    
//     Ok(())
// }

// this function will take in a request object
// as well as the chosen hosts' information
// so that it can handle the farside TcpStream
// and write the results back to the initial requests' stream
fn handler(request_object: &mut Request) {}

pub fn load_balancer(request_queue: &Arc<Mutex<Queue>>, client: &mut Client) {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()/2).unwrap();

    // let mut current_host_idx: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut current_host_idx: usize = 0;

    loop {
        sleep(Duration::from_millis(THREAD_SLEEP));
        // get the latest request object
        let request_object = request_queue
            .lock()
            .unwrap()
            .next();

        match request_object {
            Some(mut r) => {
                // determine which host to send the request to
                current_host_idx = current_host_idx % client.hosts.len();
                let selected_host: Host = client.hosts[current_host_idx].clone();
                current_host_idx += 1;

                pool.execute(move || {
                    test_handler(&mut r, &selected_host);
                });
            },
            None => {},
        }
    }
}

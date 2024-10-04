use core::time::Duration;
use std::{
    error::Error, io::{BufRead, BufReader, Read, Write}, net::TcpStream, sync::{Arc, Mutex}, thread::sleep
};

use crate::{
    client::{Client, Host, HostHealth},
    queue::{Queue, Request},
    threadpool::ThreadPool
};

const THREAD_SLEEP_MILLIS: u64 = 2;

fn test_handler(request_object: &mut Request, _host: &Host) {

    let contents: String = String::from("Request has been handled!");
    let length: usize = contents.len();
    let response: String =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

    request_object.stream.write_all(response.as_bytes()).unwrap();
}


fn get_request_content(stream: TcpStream) -> Result<String, Box<dyn Error>>{
    let mut reader: BufReader<TcpStream> = BufReader::new(stream.try_clone().unwrap());

    let mut response_string = String::new();
    loop {
        let r = reader.read_line(&mut response_string).unwrap();
            if r < 3 { //detect empty line
                break;
            }
    }

    let mut content_length_size = 0;
    let split_lines = response_string.split("\n");

    for line in split_lines {
        if line.starts_with("Content-Length") {

            let sizeplit = line.split(":");

            for s in sizeplit {
                if !(s.starts_with("Content-Length")) {
                    content_length_size = s.trim().parse::<usize>().unwrap();
                }
            }
        }
    }
    let mut buffer = vec![0; content_length_size]; 
    reader.read_exact(&mut buffer).unwrap();

    let content: String = String::from_utf8(buffer.clone())?;

    Ok(content)
}

// TODO - let's just use an HTTP library to handle response data^
fn handler(request_object: &mut Request, host: Host) -> Result<(), Box<dyn Error>> {
    println!("In handler()");
    // connect to farside host
    let mut farside_stream: TcpStream = TcpStream::connect_timeout(&host.hostname, Duration::from_secs(30))?;
    println!("Connected to farside host");

    // write request
    println!("Writing this request: {}", &request_object.request_data);
    farside_stream.write_all(&request_object.request_data.as_bytes())?;
    println!("Wrote to farside host");

    // get response and content from farside stream
    let content: String = get_request_content(farside_stream)?;

    println!("Farside content: {:?}", content);
    // write farside response to nearside stream
    request_object.stream.write_all(&content.as_bytes())?;
    println!("Wrote to nearside request stream");
    Ok(())
}

fn get_next_active_host(current_host_idx: &mut usize, client: &mut Client) -> Option<Host> {
    // first, update our idx counter and grab the host at this idx
    *current_host_idx = (*current_host_idx + 1) % client.hosts.len();
    let mut selected_host: Host = client.hosts[*current_host_idx].clone();

    // if this host happens to be inactive, iterate until we find an active host
    while selected_host.health == HostHealth::Inactive {
        *current_host_idx = (*current_host_idx + 1) % client.hosts.len();
        selected_host = client.hosts[*current_host_idx].clone();
    }
    println!("{}", selected_host.hostname.port());
    Some(selected_host)
}

fn run_health_checks(client: &mut Client) {
    for h in &mut client.hosts {
        h.health_check();
    }
}

pub fn lb_round_robin(request_queue: &Arc<Mutex<Queue>>, client: &mut Client) {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()/2).unwrap();

    let mut current_host_idx: usize = 0;
    loop {
        sleep(Duration::from_millis(THREAD_SLEEP_MILLIS));
        // get the latest request object
        let request_object = request_queue
            .lock()
            .unwrap()
            .next();

        match request_object {
            Some(mut req) => {
                // get which host to send the request to
                let selected_host: Host =
                    get_next_active_host(&mut current_host_idx, client).unwrap();

                pool.execute(move || {
                    let _ = handler(
                        &mut req,
                        selected_host
                    );
                });
            },
            None => {},
        }
    }
}

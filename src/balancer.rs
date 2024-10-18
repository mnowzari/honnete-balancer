use core::time::Duration;
use std::{
    error::Error,
    io::{BufRead, BufReader, Read, Write}, 
    net::TcpStream,
    sync::{Arc, Mutex},
    thread::sleep
};

use crate::{
    host::{Hosts, Host, HostHealth}, env::Environment, queue::{Queue, Request}, threadpool::ThreadPool
};

const THREAD_SLEEP_MILLIS: u64 = 2;
const TCP_STREAM_TIMEOUT: u64 = 60;

fn get_request_content(stream: &TcpStream) -> Result<Vec<String>, Box<dyn Error>>{
    let mut reader: BufReader<TcpStream> = BufReader::new(stream
        .try_clone()
        .unwrap());

    let mut response_string = String::new();
    loop {
        let r = reader.read_line(&mut response_string).unwrap();
            if r < 3 { //detect empty line
                break;
            }
    }

    let mut response_code: &str = "";
    let mut content_length_size: usize = 0;
    let split_lines = response_string.split("\n");

    for line in split_lines {
        if line.starts_with("HTTP/1.1") {
            response_code = line;
        }
        
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

    Ok(vec![response_code.to_string(), content])
}

fn _test_handler(request_object: &mut Request, _host: Host) {

    let contents: String = String::from("Request has been handled!");
    let length: usize = contents.len();
    let response: String =
        format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{contents}");

    request_object.stream.write_all(response.as_bytes()).unwrap();
}

// TODO - let's just use an HTTP library to handle response data^
fn handler(request_object: &mut Request, host: Host) -> Result<(), Box<dyn Error>> {
    // connect to farside host
    let upstream_connection: Result<TcpStream, std::io::Error> = TcpStream::connect_timeout(
        &host.hostname,
        Duration::from_secs(TCP_STREAM_TIMEOUT));
    
    match upstream_connection {
        Ok(mut upstream) => {
            // write request
            upstream.write_all(&request_object.request_data.as_bytes())?;
        
            // get response code and content from farside stream
            let response_content: Vec<String> = get_request_content(&upstream)?;
            let code: &String = &response_content[0];
            let content: &String = &response_content[1];

            // write farside response to nearside stream
            let length: usize = content.len();
            let response: String =
                format!("{code}\r\nContent-Length: {length}\r\n\r\n{content}");
            
            // println!("Incoming farside content: {:?}", response);

            request_object.stream.write_all(&response.as_bytes())?;
        },
        Err(_x) => {},
    }
    Ok(())
}

fn get_next_active_host(current_host_idx: &mut usize, hosts_instance: &mut Hosts) -> Option<Host> {
    // println!("Getting next active host");
    // first, update our idx counter and then grab the host at this idx
    *current_host_idx = (*current_host_idx + 1) % hosts_instance.hosts.len();
    let mut selected_host: Host = hosts_instance.hosts[*current_host_idx].clone();

    let max_retries: usize = hosts_instance.hosts.len() * 3;
    let mut retry_count: usize = 0;
    // if this host happens to be inactive, iterate until we find an active host
    while selected_host.health == HostHealth::Inactive {
        *current_host_idx = (*current_host_idx + 1) % hosts_instance.hosts.len();
        selected_host = hosts_instance.hosts[*current_host_idx].clone();

        retry_count += 1;
        if retry_count >= max_retries {
            break;
        }
    }
    // println!("{}", selected_host.hostname.port());
    match selected_host.health {
        HostHealth::Active => Some(selected_host),
        HostHealth::Inactive => None,
    }
}

fn run_health_checks(hosts: &mut Vec<Host>) {
    for h in hosts {
        // println!("{} => {}", h.hostname.port(), h.health);
        h.health_check();
    }
}

pub fn lb_round_robin(request_queue: &Arc<Mutex<Queue>>, client: &mut Hosts, num_cpu: &usize) {
    let pool: ThreadPool = ThreadPool::new(*num_cpu / 2).unwrap();

    let mut current_host_idx: usize = 0;
    let mut health_check_counter: u64 = 0;

    loop {

        health_check_counter += 1;
        // we want to make sure we check once every 10s regardless of tickrate
        if health_check_counter % ((1000/THREAD_SLEEP_MILLIS) * 10)  == 0 {
            // this could be a separate short-lived thread?
            run_health_checks(&mut client.hosts);
            health_check_counter = 0;
        }

        sleep(Duration::from_millis(THREAD_SLEEP_MILLIS));

        // get the latest request object from queue
        let request_object = request_queue
            .lock()
            .unwrap()
            .next();

        match request_object {
            Some(mut req) => {
                
                match get_next_active_host(&mut current_host_idx, client) {
                    Some(h) => {
                        // let len_of_q: usize = request_queue.lock().unwrap().len();
                        // if len_of_q > 0 {
                        //     println!("{}", len_of_q);
                        // }
                        pool.execute(move || {
                            let _ = handler(
                                &mut req,
                                h
                            );
                        });
                    },
                    None => {
                        // if selected_host comes back as None, we should
                        // append the request back into the queue
                        request_queue
                            .lock()
                            .unwrap()
                            .enqueue(req);
                    },
                }
            },
            None => {},
        }
    }
}

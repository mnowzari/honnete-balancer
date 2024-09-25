use std::{io::{BufRead, BufReader, Write}, net::TcpStream, thread, time::Duration};

pub fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    println!("{}", request_line);

    let status_line = match &request_line[..] {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK",

        "GET /request HTTP/1.1" => "HTTP/1.1 201 OK",

        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            "HTTP/1.1 200 OK"
        }

        _ => "HTTP/1.1 404 NOT FOUND",
    };

    // let contents = fs::read_to_string(filename).unwrap();
    let contents: String = String::from("CONTENT");
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
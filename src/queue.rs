use std::{
    error::Error, net::TcpStream, sync::{Arc, Mutex}
};

pub struct Request {
    pub request_data: String,
    pub stream: TcpStream,
}


pub struct Queue {
    pub requests_queue: Vec<Request>
}

impl Queue {
    pub fn new() -> Result<Arc<Mutex<Queue>>, Box<dyn Error>> {
        Ok(Arc::new(Mutex::new(Queue {requests_queue: vec![]})))
    }

    pub fn enqueue(&mut self, value: Request) {
        self.requests_queue.push(value);
    }

    pub fn next(&mut self) -> Option<Request> {
        if self.requests_queue.len() > 0 {
            Some(self.requests_queue.remove(0))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.requests_queue.len()
    }
}

#[cfg(test)]
mod tests {
    use std::net::TcpStream;
    use crate::queue::{Queue, Request};

    #[test]
    fn queue_test() {
        let q = Queue::new()
            .unwrap();

        // fake request objects
        // TODO - FIX THIS TEST FIX FIX FIX ASAP
        let request_one: Request = Request {request_data: String::from("r1"),
            stream: TcpStream::connect("0.0.0.0:80").unwrap()
        };

        let request_two: Request = Request {request_data: String::from("r2"),
            stream: TcpStream::connect("0.0.0.0:80").unwrap()
        };

        let request_three: Request = Request {request_data: String::from("r3"),
            stream: TcpStream::connect("0.0.0.0:80").unwrap()
        };

        q.lock().unwrap().enqueue(request_one);
        q.lock().unwrap().enqueue(request_two);
        q.lock().unwrap().enqueue(request_three);
            
        assert_eq!(q.lock().unwrap().next().unwrap().request_data, String::from("r1"));
        assert_eq!(q.lock().unwrap().next().unwrap().request_data, String::from("r2"));
        assert_eq!(q.lock().unwrap().next().unwrap().request_data, String::from("r3"));

    }
}
use std::{
    error::Error,
    sync::{Arc, Mutex}
};

pub struct Queue {
    pub requests_queue: Vec<String>
}

impl Queue {
    pub fn new() -> Result<Arc<Mutex<Queue>>, Box<dyn Error>> {
        Ok(Arc::new(Mutex::new(Queue {requests_queue: vec![]})))
    }

    pub fn enqueue(&mut self, value: String) {
        self.requests_queue.push(value);
    }

    pub fn next(&mut self) -> Option<String> {
        if self.requests_queue.len() > 0 {
            Some(self.requests_queue.remove(0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {    
    use crate::queue::Queue;

    #[test]
    fn queue_test() {
        let mut q = Queue::new()
            .unwrap();

        q.lock().unwrap().enqueue(String::from("5"));
        q.lock().unwrap().enqueue(String::from("7"));
        q.lock().unwrap().enqueue(String::from("9"));
            
        assert_eq!(q.lock().unwrap().next(), Some("5".to_string()));
        assert_eq!(q.lock().unwrap().next(), Some("7".to_string()));
        assert_eq!(q.lock().unwrap().next(), Some("9".to_string()));
        assert_eq!(q.lock().unwrap().next(), None);
    }
}
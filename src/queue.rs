use std::error::Error;

pub struct Queue {
    pub requests_queue: Vec<String>
}

impl Queue {
    pub fn new() -> Result<Queue, Box<dyn Error>> {
        Ok(Queue {
            requests_queue: vec![],
        })
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

        q.enqueue(String::from("5"));
        q.enqueue(String::from("7"));
        q.enqueue(String::from("9"));
            
        assert_eq!(q.next(), Some("5".to_string()));
        assert_eq!(q.next(), Some("7".to_string()));
        assert_eq!(q.next(), Some("9".to_string()));
        assert_eq!(q.next(), None);
    }
}
use tokio::sync::{Semaphore, SemaphorePermit};
pub struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl<'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("ticket freed");
    }
}
impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self {
        Self { permit }
    }
}
impl Museum {
    pub fn new(total: usize) -> Self {
        Self {
            remaining_tickets: Semaphore::new(total),
        }
    }
    pub fn get_ticket(&self) -> Option<Ticket> {
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_tickets.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let museum = Museum::new(50);
        let ticket = museum.get_ticket().unwrap();
        assert_eq!(museum.tickets(), 49);

        let _tickets: Vec<Ticket> = (0..49).map(|_i| museum.get_ticket().unwrap()).collect();
        assert_eq!(museum.tickets(), 0);

        assert!(museum.get_ticket().is_none());

        drop(ticket);
        {
            let ticket = museum.get_ticket().unwrap();
            println!("ticket: {:?}", ticket);
        }
        println!("--------------------------------")
        // assert!(museum.get_ticket().is_some());
    }

    #[test]
    fn move_copy() {
        let a = 10;
        let b = vec![1, 2, 3];

        let (ax, bx) = (a, b.clone());

        let c = a;
        let d = b;
    }

    #[test]
    fn drop_test() {
        struct MyString(String);
        struct MyBox<T>(Box<T>);
        struct MyVec<T>(Vec<T>);

        impl Drop for MyString {
            fn drop(&mut self) {
                println!("MyString {} dropped", self.0);
            }
        }

        impl<T> Drop for MyBox<T> {
            fn drop(&mut self) {
                println!("MyBox dropped");
            }
        }

        impl<T> Drop for MyVec<T> {
            fn drop(&mut self) {
                println!("MyVec dropped");
            }
        }

        let s1 = MyString("Hello, world".to_string());
        let s2 = MyString("Goodbye, world".to_string());
        let arr = MyVec(vec![MyBox(Box::new(s1)), MyBox(Box::new(s2))]);
    }
}

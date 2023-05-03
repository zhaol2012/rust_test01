use tokio::sync::{Semaphore, SemaphorePermit};
pub struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl <'a> Drop for Ticket<'a> {
    fn drop(&mut self) {
        println!("ticket freed");
    }
}
impl<'a> Ticket<'a> {
    pub fn new(permit: SemaphorePermit<'a>) -> Self { Self { permit } }
}
impl Museum {
    pub fn new(total: usize) -> Self { Self { remaining_tickets: Semaphore::new(total) } }
    pub fn get_ticket(&self) -> Option<Ticket>{
        match self.remaining_tickets.try_acquire() {
            Ok(permit) => Some(Ticket::new(permit)),
            Err(_) => None,
        }
    }

    pub fn tickets(&self) -> usize { self.remaining_tickets.available_permits() }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        let museum = Museum::new(50);
        let ticket = museum.get_ticket().unwrap();
        assert_eq!(museum.tickets(), 49);

        let tickets: Vec<Ticket> = (0..49).map(|_i|museum.get_ticket().unwrap()).collect();
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
}
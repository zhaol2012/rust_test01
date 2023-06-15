mod users;
mod strtok;
mod ticket;
mod fibonacci;
mod encoder;
mod actor;
mod pin;

use anyhow::Result;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>>;
}

pub struct Event<Id, Data> {
    id: Id,
    data: Data,
}

impl<Id, Data> Event<Id, Data> 
where
    Id: Encoder,
    Data: Encoder,
{
    pub fn new(id: Id, data: Data) -> Self {
        Self { id, data}
    }

    pub fn do_encode(&self) -> Result<Vec<u8>> {
        let mut result = self.id.encode()?;
        result.append(&mut self.data.encode()?);
        Ok(result)
    }

}

impl Encoder for u64 {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(vec![1,2,3,4,5,6,7,8])
    }
}

impl Encoder for String {
    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.as_bytes().to_vec())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
       let event = Event::new(1, "hello world".to_string());
       let _vec = event.do_encode().unwrap();
    }
}

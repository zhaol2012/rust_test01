use std::{marker::PhantomPinned, pin::Pin};

struct Data {
    data: usize,
}

struct SelfRef {
    data: Data,
    data_ref: *const Data,
    _p: PhantomPinned,
}

impl SelfRef {
    fn new(data: Data) -> Self {
        Self {
            data,
            data_ref: std::ptr::null(),
            _p: PhantomPinned::default(),
        }
    }

    fn init(self: Pin<&mut Self>) {
        let data_ref: *const Data = &self.data;
        let this = unsafe { self.get_unchecked_mut()};
        this.data_ref = data_ref;
    }

    fn print_into(&self) {
        println!(
            "data address: {:p} content: {}，data_ref：{:p} content: {}",
            &self.data,
            self.data.data,
            self.data_ref,
            unsafe { &*self.data_ref }.data
        );
    }
}

#[cfg(test)]
mod tests {
    use futures::pin_mut;

    use super::*;

    #[test]
    fn it_works() {
        let self_ref = SelfRef::new(Data{ data: 1});
        pin_mut!(self_ref);
        self_ref.as_mut().init();
        self_ref.print_into();

        let mut data = unsafe { self_ref.as_mut().map_unchecked_mut(|n|&mut n.data)};
        data.data = 2;
        self_ref.print_into();
    }
}
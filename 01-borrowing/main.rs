#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::borrow::{Borrow, BorrowMut};

struct View<T : Borrow<[u8]>> {
    buff: T
}

impl<T : Borrow<[u8]>> View<T> {
    fn get_size(&self) -> u8 {
        self.buff.borrow()[0]
    }
}

impl<T : BorrowMut<[u8]>> View<T> {
    fn set_size(&mut self, val : u8) {
        self.buff.borrow_mut()[0] = val
    }
}

fn main() {
    {
        let mut v = vec![0,1,2];
        {
            let b = View{buff:&v[..]};
        }
        {
            let mut b = View{buff:&mut v[..]};
        }
    }
    {
        let mut v = vec![0,1,2];
        {
            let b = View{buff:& mut v[..]};
        }
        {
            let b = View{buff:&v[..]};
        }
    }
    {

        let v = vec![0,1,2];
        let mut b = View{buff:v};
        b.set_size(1);

    }
}

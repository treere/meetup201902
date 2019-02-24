#[derive(Debug)]
struct View<T: AsRef<[u8]>> {
    b: T
}

impl<'a, T: AsRef<[u8]> + AsMut<[u8]> + ?Sized> View<&'a mut T> {
    fn get_buff_safe(& mut self) -> & mut [u8] {
        self.b.as_mut()
    }
    fn get_buff_unsafe<'b>(& 'b mut self) -> & 'b mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.b.as_mut().as_mut_ptr(), 4)
        }
    }
    fn get_buff_super_unsafe(& mut self) -> &'a mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.b.as_mut().as_mut_ptr(), 4)
        }
    }
}

fn main() {
    {
        println!("safe");
        let mut v: Vec<u8> = vec![4, 3, 2, 1, 0];
        {
            let mut b = View { b: & mut v[..] };
            {
                let t = b.get_buff_safe();
                // let t2 = b.get_buff_safe();
                t[0] = 1;
                // t2[0] = 4;
            }
            println!("get buf after {:?}", b);
        }

    }
    {
        println!("safe");
        let mut v: Vec<u8> = vec![4, 3, 2, 1, 0];
        {
            let mut b = View { b: & mut v[..] };
            {
                let t = b.get_buff_unsafe();
                // let t2 = b.get_buff_unsafe();
                t[0] = 1;
                // t2[0] = 4;
            }
            println!("get buf after {:?}", b);
        }

    }
    {
        println!("super unsafe");
        let mut v: Vec<u8> = vec![4, 3, 2, 1, 0];
        {
            let mut b = View { b: & mut v[..] };
            {
                let t = b.get_buff_super_unsafe();
                let t2 = b.get_buff_safe();
                t[0] = 1;
                t2[0] = 42;
            };
            println!("get buf after {:?}", b);
        }
    }
}

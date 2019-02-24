#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

struct View<T : AsRef<[u8]>> {
	buff: T
}

impl<T : AsRef<[u8]>> View<T> {
	fn get_size(&self) -> u8 {
		self.buff.as_ref()[0]
	}
}

impl<T : AsRef<[u8]> + AsMut<[u8]>> View<T> {
	fn set_size(&mut self, val : u8) {
		self.buff.as_mut()[0] = val
	}
}

fn main() {
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
        {
            let v = vec![0,1,2];
            let b = View{buff:v};
        }
    }
    {
        {
            let v = vec![0,1,2];
            let b = View{buff:&v};
        }
    }
}

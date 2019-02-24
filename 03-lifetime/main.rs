#![feature(specialization)]

#[derive(Debug)]
struct View<T: AsRef<[u8]>> {
    b: T
}

impl<T: AsRef<[u8]>> View<T> {
    fn read_size(&self) -> &[u8] {
        &self.b.as_ref()[0..1]
    }
}
trait GetBuff<'a,'b> {
    fn get_buff(&'b self) -> &'a [u8];
}

impl<'a, T: AsRef<[u8]> > GetBuff<'a,'a> for  View<T> {
    default fn get_buff(&'a self) -> &'a [u8] {
        self.b.as_ref()
    }
}

impl<'b, 'a, T: AsRef<[u8]> + ?Sized> GetBuff<'a,'b> for  View<&'a T> {
    fn get_buff(&'b self) -> &'a [u8] {
        self.b.as_ref()
    }
}

fn main() {
        let v: Vec<u8> = vec![4, 3, 2, 1, 0];
        let b = {
            let b = View { b: & v[..] };

            println!("b: {:?}", b);
            println!("size: {:?}", b.read_size());
            b.get_buff()
        };
        println!("get buf after {:?}", b);
}

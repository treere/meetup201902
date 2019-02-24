#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

struct View<'a> {
	buff: &'a[u8]
}

impl<'a> View<'a> {
	fn get_buff<'b>(&'b self) -> &'a[u8] {
		self.buff
	}
}

struct ViewMut<'a> {
	buff: &'a mut [u8]
}

impl<'a> ViewMut<'a> {
	fn get_buff<'b>(&'b self) -> &'b &'a mut [u8] {
		&self.buff
	}
}


fn main() {
	let mut v = vec![0,1,2];
	{
		let b = View{buff:& v[..]};
		//let b = View{buff:& mut v[..]};
	}
	{
		let b = ViewMut{buff:& mut v[..]};
		//let b = ViewMut{buff:&v[..]};
	}
}

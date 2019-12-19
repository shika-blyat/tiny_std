use std::mem::size_of;
use std::ptr;

pub struct Iter<T>{
	start: *const T,
	length: usize,
	count: usize,
}
impl<T> Iter<T>{
	pub fn new(start: *const T, length: usize) -> Self{
		Self {
			start,
			length,
			count:0
		}
	}
	fn get_index(&self, index: usize) -> *mut T {
        (self.start as usize + (size_of::<T>() as usize * index)) as *mut T
    }
}

impl<T> Iterator for Iter<T>{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.count += 1;
        if self.count <= self.length {
            Some(unsafe{ptr::read(self.get_index(self.count-1))})            
        } else {
            None
        }
    }
}
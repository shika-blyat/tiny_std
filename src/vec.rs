use std::alloc::{alloc, realloc, /*dealloc,*/ Layout};
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Index;
use std::ptr;
use crate::iter::Iter;
use std::slice;

#[derive(PartialEq)]
pub struct Vector<T: Sized> {
    layout: Layout,
    pointer: *mut u8,
    capacity: usize,
    length: usize,
    phantom: PhantomData<T>,
}

impl<T: Sized> Vector<T> {
    pub fn new() -> Self {
        let capacity = 500;
        let layout = Layout::new::<u32>();
        let pointer = unsafe { realloc(alloc(layout), layout, capacity) };
        let length = 0;
        Self {
            layout,
            pointer,
            capacity,
            length,
            phantom: PhantomData,
        }
    }
    pub fn append(&mut self, other: &mut Vector<T>){
        for i in other.iter(){
            self.push(i);
        }
        other.clear();
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T{
        self.pointer as *mut T
    }
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe{
            slice::from_raw_parts_mut(self.as_mut_ptr(), self.len())
        }
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T{
        self.pointer as *const T
    }
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe{
            slice::from_raw_parts(self.as_ptr(), self.len())
        }
    }
    pub fn capacity(&self) -> usize{
        self.capacity / size_of::<T>()
    }
    pub fn clear(&mut self) {
        self.length = 0;
    }
    pub fn dedup_by<F>(&mut self, same_bucket: F) 
    where
        F: FnMut(&mut T, &mut T) -> bool{
        let mut last;
        for i in self{
            last = i;
        }
    } 
    pub fn reserve(&mut self, add: usize){
        if self.capacity > add {
            return
        }
        self.realloc_exact(add);
    }
    pub fn iter(&self) -> Iter<T> {
        let start = self.as_ptr();
        Iter::new(start, self.length)
    }

    pub fn push(&mut self, v: T) {
        if self.capacity <= self.length {
            self.realloc();
        }
        let pointer = (self.pointer as usize + (size_of::<T>() as usize * self.length)) as *mut u8;
        self.length += 1;
        unsafe {
            *(pointer as *mut T) = v;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        unsafe { Some(ptr::read(self.get_index(self.length))) }
    }
    #[inline]
    fn get_index(&self, index: usize) -> *mut T {
        (self.pointer as usize + (size_of::<T>() as usize * index)) as *mut T
    }
    pub fn remove(&mut self, index: usize) {
        for i in index + 1..self.length {
            unsafe {
                ptr::swap(self.get_index(i - 1), self.get_index(i));
            }
        }
        self.length -= 1;
    }
    pub const fn len(&self) -> usize {
        self.length
    }
    #[inline]
    fn realloc(&mut self) {
        unsafe {
            self.capacity *= 2;
            self.pointer = realloc(self.pointer, self.layout, self.capacity);
        }
    }
    #[inline]
    fn realloc_exact(&mut self, add: usize) {
        unsafe {
            self.capacity += add;
            self.pointer = realloc(self.pointer, self.layout, self.capacity);
        }
    }
    pub fn from_vec(vec: Vec<T>) -> Vector<T> {
        let mut vector = Vector::new();
        for i in vec{
            vector.push(i);
        }
        vector
    }
}

impl<T: Clone> From<&[T]> for Vector<T> {
    fn from(s: &[T]) -> Vector<T> {
        Vector::from_vec(s.to_vec())
    }
}

impl<T> IntoIterator for &Vector<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Iter<T> {
        self.iter()
    }
}

impl<T> IntoIterator for &mut Vector<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Iter<T> {
        self.iter()
    }
}
impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    #[inline]
    fn into_iter(self) -> Iter<T> {
            self.iter()
    }
}
impl<T: Sized + fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (k, i) in (0..self.length).enumerate() {
            if k != self.length - 1 {
                write!(f, "{}, ", unsafe { ptr::read(self.get_index(i)) })?;
            } else {
                write!(f, "{}", unsafe { ptr::read(self.get_index(i)) })?;
            }
        }
        write!(f, "]")
    }
}

impl<T: Sized> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.length {
            panic!(
                "index out of bounds: the len is {} but the index is {}",
                self.length, index
            );
        }
        unsafe { &*((self.pointer as usize + (size_of::<T>() as usize * index)) as *mut T) }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec::Vector;
    #[test]
    fn push() {
        let mut vec = Vector::new();
        vec.push(15);
        vec.push(12);
        assert_eq!(15, vec[0]);
        assert_eq!(12, vec[1]);
    }
    #[test]
    fn pop() {
        let mut vec = Vector::new();
        vec.push(15);
        assert_eq!(15, vec.pop().unwrap());
        if let Some(_) = vec.pop() {
            panic!("Error occured during `pop` test");
        }
    }
    #[test]
    fn remove() {
        let mut vec = Vector::new();
        vec.push(15);
        vec.push(17);
        vec.push(19);
        vec.push(20);
        vec.push(21);
        vec.push(22);
        vec.remove(4);
        assert_eq!(22, vec[4]);
    }
    #[test]
    #[should_panic]
    fn out_of_range() {
        let mut vec = Vector::new();
        vec.push(15);
        vec.push(12);
        vec[5];
    }
    #[test]
    #[should_panic]
    fn pop_out_of_range() {
        let mut vec = Vector::new();
        vec.push(15);
        vec.push(12);
        vec.pop();
        vec[1];
    }
}

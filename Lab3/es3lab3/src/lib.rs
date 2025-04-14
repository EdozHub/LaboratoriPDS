use std::ops::{Deref, Index, IndexMut};


pub struct CircularBuffer<T>{
    pub buffer: Vec<Option<T>>,
    pub size: usize,
    pub head: usize,
    pub tail: usize
}

pub trait TryDeref<T>{
    type Output;
    fn try_deref(self: &Self) -> Option<&Self::Output>;
}

impl<T: Clone> CircularBuffer<T>{
    pub fn new(capacity: usize) -> Self{
        Self{
            buffer: vec![None; capacity],
            size: capacity,
            head: 0,
            tail: 0
        }
    }
    pub fn write(&mut self, item: Option<T>) -> Result<(),&str>{
        if self.head == self.tail && self.buffer[self.head].is_some() {
            Err("Circular buffer is full")
        }
        else {
            self.buffer[self.tail] = item;
            self.tail = (self.tail + 1) % self.size;
            Ok(())
        }
    }

    pub fn read(&mut self) -> Option<T>{
        if self.head == self.tail && self.buffer[self.head].is_none() {
            None
        }
        else {
            let item = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.size;
            item
        }
    }

    pub fn clear(&mut self){
        self.buffer = self.buffer.iter().map(|_| None).collect();
        self.head = 0;
        self.tail = 0;
    }

    pub fn size(&self) -> usize {
        if self.head == self.tail {
            0
        }
        else if self.tail > self.head {
            self.tail - self.head
        }
        else {
            self.size - self.head + self.tail
        }
    }
    pub fn overwrite(&mut self, item: Option<T>){
        if !(self.head == self.tail && self.buffer[self.head].is_some()){
            let res = self.write(item);
        }
        else{
            self.buffer[self.head] = item;
            self.head = (self.head + 1) % self.size;
            self.tail = (self.tail + 1) % self.size;
        }
    }
    pub fn make_contiguous(&mut self){
        if self.tail < self.head{
            let mut new_buffer: Vec<Option<T>> = vec![None; self.size];
            for i in 0 .. self.size{
                new_buffer[i] = self.buffer[(i+self.head) % self.size].take();
            }
            self.buffer = new_buffer;
            self.tail = (self.tail + self.head) % self.size;
            self.head = 0;
        }
    }
}

impl<T: Clone> Index<usize> for CircularBuffer<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        let real_index = (self.head + index) % self.size;
        &self.buffer[real_index]
    }
}
impl<T: Clone> IndexMut<usize> for CircularBuffer<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.size {
            panic!("Index out of bounds");
        }

        let real_index = (self.head + index) % self.size;
        &mut self.buffer[real_index]
    }
}

impl <T> Deref for CircularBuffer<T>{
    type Target = Vec<Option<T>>;
    fn deref(&self) -> &Self::Target{
        if self.tail < self.head{
            panic!("Vector is non continous")
        }
        &self.buffer
    }
}

//Prova commit per vedere se GitHub si aggiorna
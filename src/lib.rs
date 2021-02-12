#![feature(maybe_uninit_slice)]

use std::vec::Vec;
use libc::{c_char, c_void};
use std::mem::MaybeUninit;


pub trait CBuffer {
    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
}


impl CBuffer for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }

    fn capacity(&self) -> usize {
        self.capacity()
    }
}



impl<const N: usize> CBuffer for [u8; N] {
    fn len(&self) -> usize {
        N
    }

    fn capacity(&self) -> usize {
        N
    }
}


impl<const N: usize> CBuffer for [MaybeUninit<u8>; N] {
    fn len(&self) -> usize {
        N
    }

    fn capacity(&self) -> usize {
        N
    }
}





// an initialized buffer: array, slice or vec
pub trait TxBuffer: CBuffer {
    fn as_c_void (&self) -> (*const c_void, usize);
    fn as_c_char (&self) -> (*const c_char, usize);
}


impl TxBuffer for Vec<u8> {
   fn as_c_void (&self) -> (*const c_void, usize){
        (self.as_ptr() as *mut c_void, self.len())
    }

    fn as_c_char (&self) -> (*const c_char, usize){
        (self.as_ptr() as *mut c_char, self.len())
    }
}

impl<const N: usize> TxBuffer for [u8; N] {
    fn as_c_void (&self) -> (*const c_void, usize){
        (self.as_ptr() as *mut c_void, self.len())
    }

    fn as_c_char (&self) -> (*const c_char, usize){
        (self.as_ptr() as *mut c_char, self.len())
    }
}






// an possibly uninitialized: buffer array or vec
pub trait RxBuffer: CBuffer {
    fn as_c_void (&mut self) -> (*mut c_void, usize);
    fn as_c_char (&mut self) -> (*mut c_char, usize);
    fn rx_done(&mut self, len: usize) -> &[u8];
}



impl RxBuffer for Vec<u8> {
    fn as_c_void (&mut self) -> (*mut c_void, usize){
        unsafe {self.set_len(self.capacity())};
        (self.as_mut_ptr() as *mut c_void, self.len())
    }

    fn as_c_char (&mut self) -> (*mut c_char, usize){
        (self.as_mut_ptr() as *mut c_char, self.len())
    }

    fn rx_done(&mut self, len: usize) -> &[u8]{
        assert!(len <= self.capacity());
        unsafe {self.set_len(len)};
        &self[..len]
    }
}



impl<const N: usize> RxBuffer for [u8; N] {
    fn as_c_void (&mut self) -> (*mut c_void, usize){
        (self.as_mut_ptr() as *mut c_void, N)
    }

    fn as_c_char (&mut self) -> (*mut c_char, usize){
        (self.as_mut_ptr() as *mut c_char, N)
    }

    fn rx_done(&mut self, len: usize) -> &[u8]{
        &self[..len]
    }
}

impl<const N: usize> RxBuffer for [MaybeUninit<u8>; N] {
    fn as_c_void (&mut self) -> (*mut c_void, usize){
        (self.as_mut_ptr() as *mut c_void, N)
    }

    fn as_c_char (&mut self) -> (*mut c_char, usize){
        (self.as_mut_ptr() as *mut c_char, N)
    }

    fn rx_done(&mut self, len: usize) -> &[u8]{
        unsafe {MaybeUninit::slice_assume_init_ref(&self[..len])}
    }
}




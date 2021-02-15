#![feature(maybe_uninit_slice)]

use std::vec::Vec;
use libc::{c_char, c_void, size_t};
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
    fn as_c_void (&self) -> (*const c_void, size_t);

    fn as_c_char (&self) -> (*const c_char, size_t);
}


impl TxBuffer for Vec<u8> {
   fn as_c_void (&self) -> (*const c_void, size_t){
        (self.as_ptr() as *mut c_void, self.len())
    }

    fn as_c_char (&self) -> (*const c_char, size_t){
        (self.as_ptr() as *mut c_char, self.len())
    }
}

impl<const N: usize> TxBuffer for [u8; N] {
    fn as_c_void (&self) -> (*const c_void, size_t){
        (self.as_ptr() as *mut c_void, self.len())
    }

    fn as_c_char (&self) -> (*const c_char, size_t){
        (self.as_ptr() as *mut c_char, self.len())
    }
}






// an possibly uninitialized: buffer array or vec
pub trait RxBuffer: CBuffer {
    fn as_c_void (&mut self) -> (*mut c_void, size_t);

    fn as_c_char (&mut self) -> (*mut c_char, size_t);

    fn rx_done(&mut self, len: size_t) -> &[u8];
}



impl RxBuffer for Vec<u8> {
    fn as_c_void (&mut self) -> (*mut c_void, size_t){
        unsafe {self.set_len(self.capacity())};
        (self.as_mut_ptr() as *mut c_void, self.len())
    }

    fn as_c_char (&mut self) -> (*mut c_char, size_t){
        (self.as_mut_ptr() as *mut c_char, self.len())
    }

    fn rx_done(&mut self, len: usize) -> &[u8]{
        assert!(len <= self.capacity());
        unsafe {self.set_len(len)};
        &self[..len]
    }
}



impl<const N: usize> RxBuffer for [u8; N] {
    fn as_c_void (&mut self) -> (*mut c_void, size_t){
        (self.as_mut_ptr() as *mut c_void, N)
    }

    fn as_c_char (&mut self) -> (*mut c_char, size_t){
        (self.as_mut_ptr() as *mut c_char, N)
    }

    fn rx_done(&mut self, len: usize) -> &[u8]{
        &self[..len]
    }
}

impl<const N: usize> RxBuffer for [MaybeUninit<u8>; N] {
    fn as_c_void (&mut self) -> (*mut c_void, size_t){
        (self.as_mut_ptr() as *mut c_void, N)
    }

    fn as_c_char (&mut self) -> (*mut c_char, size_t){
        (self.as_mut_ptr() as *mut c_char, N)
    }

    fn rx_done(&mut self, len: usize) -> &[u8]{
        unsafe {MaybeUninit::slice_assume_init_ref(&self[..len])}
    }
}




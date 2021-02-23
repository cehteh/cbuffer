//! Low level interface to C-API's that expect a pointer/size reference to a buffer
//!
//! In many libc API's it is common to pass a pointer/size pair into a function.
//! This describes the location and length of data to be read or written.
//!
//! This library provides tools to generate such pointer/size pairs in rust.  In rust these
//! are either vectors or array of 'u8' values.  Buffers used for reading data in can be
//! defined as 'uninit_array!' to reduce the overhead of unnnecessary initialization.
//!
#![feature(maybe_uninit_slice)]
use libc::{c_char, c_void, size_t};
use std::mem::MaybeUninit;
use std::vec::Vec;

/// The 'TxBuffer' is a used when writing data out.
/// It's contents must be fully initialized.
pub trait TxBuffer {
    /// The used length
    fn len(&self) -> usize;

    /// Returns a const void*/size_t pair to be used in the C call.
    fn as_c_void(&self) -> (*const c_void, size_t);

    /// Returns a const char*/size_t pair to be used in the C call.
    fn as_c_char(&self) -> (*const c_char, size_t);
}

impl TxBuffer for Vec<u8> {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_c_void(&self) -> (*const c_void, size_t) {
        (self.as_ptr() as *mut c_void, self.len())
    }

    fn as_c_char(&self) -> (*const c_char, size_t) {
        (self.as_ptr() as *mut c_char, self.len())
    }
}

impl<const N: usize> TxBuffer for [u8; N] {
    fn len(&self) -> usize {
        N
    }

    fn as_c_void(&self) -> (*const c_void, size_t) {
        (self.as_ptr() as *mut c_void, self.len())
    }

    fn as_c_char(&self) -> (*const c_char, size_t) {
        (self.as_ptr() as *mut c_char, self.len())
    }
}

/// The 'RxBuffer' is a used when reading data in.
/// The contents of the buffer can be uninitialized.
pub trait RxBuffer {
    /// The allocated size
    fn capacity(&self) -> usize;

    /// Returns a void*/size_t pair to be used in the C call.
    fn as_c_void(&mut self) -> (*mut c_void, size_t);

    /// Returns a char*/size_t pair to be used in the C call.
    fn as_c_char(&mut self) -> (*mut c_char, size_t);

    /// After the reading operation is done the buffer must be sealed
    /// with the actual length of the data retrieved.
    /// This function returns a slice into the buffer containing the
    /// data.
    unsafe fn done(&mut self, len: size_t) -> &mut [u8];

    /// Get a 'Some(ResizeableBuffer)' when the underlying Buffer supports allocating more memory
    #[inline]
    fn can_set_capacity(&mut self) -> Option<&mut dyn ResizeableBuffer> {
        None
    }
}

impl RxBuffer for Vec<u8> {
    fn capacity(&self) -> usize {
        self.capacity()
    }

    fn as_c_void(&mut self) -> (*mut c_void, size_t) {
        (self.as_mut_ptr() as *mut c_void, self.capacity())
    }

    fn as_c_char(&mut self) -> (*mut c_char, size_t) {
        (self.as_mut_ptr() as *mut c_char, self.capacity())
    }

    unsafe fn done(&mut self, len: usize) -> &mut [u8] {
        assert!(len <= self.capacity());
        self.set_len(len);
        &mut self[..len]
    }

    #[inline]
    fn can_set_capacity(&mut self) -> Option<&mut dyn ResizeableBuffer> {
        Some(self)
    }
}

impl<const N: usize> RxBuffer for [u8; N] {
    fn capacity(&self) -> usize {
        N
    }

    fn as_c_void(&mut self) -> (*mut c_void, size_t) {
        (self.as_mut_ptr() as *mut c_void, N)
    }

    fn as_c_char(&mut self) -> (*mut c_char, size_t) {
        (self.as_mut_ptr() as *mut c_char, N)
    }

    unsafe fn done(&mut self, len: usize) -> &mut [u8] {
        &mut self[..len]
    }
}

impl<const N: usize> RxBuffer for [MaybeUninit<u8>; N] {
    fn capacity(&self) -> usize {
        N
    }

    fn as_c_void(&mut self) -> (*mut c_void, size_t) {
        (self.as_mut_ptr() as *mut c_void, N)
    }

    fn as_c_char(&mut self) -> (*mut c_char, size_t) {
        (self.as_mut_ptr() as *mut c_char, N)
    }

    unsafe fn done(&mut self, len: usize) -> &mut [u8] {
        MaybeUninit::slice_assume_init_mut(&mut self[..len])
    }
}

/// Implemented for Buffers which can be resized
pub trait ResizeableBuffer {
    /// Change buffer size (allocate more memory)
    fn set_capacity(&mut self, len: usize);
}

impl ResizeableBuffer for Vec<u8> {
    fn set_capacity(&mut self, len: usize) {
        // grow only
        if self.capacity() < len {
            self.reserve(len - self.len());
        };
    }
}

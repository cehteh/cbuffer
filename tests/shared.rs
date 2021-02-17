use cbuffer::{RxBuffer, TxBuffer};
use libc::size_t;

unsafe fn poke_foo(ptr: *mut u8, len: size_t) {
    let foo = b"foo\n";
    assert!(len >= foo.len());

    for i in 0..foo.len() {
        *ptr.add(i) = foo[i];
    }
}

unsafe fn assert_foo(ptr: *const u8, len: size_t) {
    let foo = b"foo\n";
    assert!(len >= foo.len());

    for i in 0..foo.len() {
        assert_eq!(*ptr.add(i), foo[i]);
    }
}

#[test]
fn foo_test() {
    let mut ary: [u8; 10] = [0; 10];
    unsafe {
        poke_foo(ary.as_mut_ptr(), 10);
        assert_foo(ary.as_ptr(), 10);
    }
}

#[test]
#[should_panic]
fn foo_fail_test() {
    let ary: [u8; 10] = [0; 10];
    unsafe {
        assert_foo(ary.as_ptr(), 10);
    }
}

pub fn write_cbuffer(buf: &dyn TxBuffer) {
    let (ptr, size) = buf.as_c_char();
    unsafe {
        assert_foo(ptr as *const u8, size);
    }
}

pub fn read_cbuffer(buf: &mut dyn RxBuffer) -> &[u8] {
    unsafe {
        let (ptr, size) = buf.as_c_void();
        poke_foo(ptr as *mut u8, size);
        assert_foo(ptr as *const u8, size);
        buf.done(4)
    }
}

pub fn read_cbuffer_resize(buf: &mut dyn RxBuffer) -> &[u8] {
    unsafe {
        if let Some(resize) = buf.can_set_capacity() {
            resize.set_capacity(4);
        }
        let (ptr, size) = buf.as_c_void();
        poke_foo(ptr as *mut u8, size);
        assert_foo(ptr as *const u8, size);
        buf.done(4)
    }
}

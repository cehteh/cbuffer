
use cbuffer::{TxBuffer, RxBuffer};


pub fn write_cbuffer_to_testfile(filename: &str, buf: &dyn TxBuffer) {

    let fd = unsafe {
        libc::creat(filename.as_ptr() as *const libc::c_char, libc::S_IRUSR|libc::S_IWUSR)
    };

    assert_ne!(fd, -1);

    let (ptr, size) = buf.as_c_void ();

    let written = unsafe {
        libc::write(fd, ptr, size)
    };

    assert_eq!(written as usize, size);

    unsafe {
        libc::close(fd)
    };
}


pub fn read_cbuffer_from_testfile<'a>(filename: &str, buf: &'a mut (dyn RxBuffer + 'a)) -> &'a[u8] {
    let fd = unsafe {
        libc::open(filename.as_ptr() as *const libc::c_char, libc::O_RDONLY)
    };

    assert_ne!(fd, -1);

    let read = unsafe {
        let (ptr, size) = buf.as_c_void ();
        libc::read(fd, ptr, size)
    };

    assert_eq!(read as usize, 4);

    unsafe {
        libc::close(fd)
    };

    buf.rx_done(read as usize)
}


pub fn delete_testfile(filename: &str) {
    unsafe {
        libc::unlink(filename.as_ptr() as *const libc::c_char)
    };
}


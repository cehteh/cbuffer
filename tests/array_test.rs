use uninit::{prelude::*};
mod shared;

#[test]
fn array_test() {
    let testfile = "tests/tmp/array_test\0";
    let write_buffer = *b"foo\n";

    shared::write_cbuffer_to_testfile(testfile, &write_buffer);

    let mut read_buffer: [u8; 10] = [0; 10];
    shared::read_cbuffer_from_testfile(testfile, &mut read_buffer);

    assert_eq!(&read_buffer[..4], b"foo\n");

    shared::delete_testfile(testfile);
}


#[test]
fn uninit_array_test() {
    let testfile = "tests/tmp/uninit_array_test\0";
    let write_buffer = *b"foo\n";

    shared::write_cbuffer_to_testfile(testfile, &write_buffer);

    let mut read_buffer = uninit_array![u8; 10];
    let got = shared::read_cbuffer_from_testfile(testfile, &mut read_buffer);

    println!("got {:?}", got);
    assert_eq!(got, b"foo\n");

    shared::delete_testfile(testfile);
}


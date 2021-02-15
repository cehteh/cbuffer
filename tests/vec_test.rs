mod shared;

#[test]
fn vec_test() {
    let testfile = "tests/tmp/vec_test\0";
    let write_buffer: Vec<u8> = b"foo\n".to_vec();

    shared::write_cbuffer_to_testfile(testfile, &write_buffer);

    let mut read_buffer: Vec<u8> = Vec::with_capacity(10);
    shared::read_cbuffer_from_testfile(testfile, &mut read_buffer);

    assert_eq!(read_buffer, b"foo\n");

    shared::delete_testfile(testfile);
}

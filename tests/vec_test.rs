mod shared;

#[test]
fn vec_tx_test() {
    shared::write_cbuffer(&b"foo\n".to_vec());
}

#[test]
fn vec_rx_test() {
    let mut read_buffer: Vec<u8> = Vec::with_capacity(10);
    let result = shared::read_cbuffer(&mut read_buffer);
    assert_eq!(result, b"foo\n");
}

#[test]
fn vec_rx_resize_test() {
    let mut read_buffer: Vec<u8> = Vec::new();
    let result = shared::read_cbuffer_resize(&mut read_buffer);
    assert_eq!(result, b"foo\n");
}

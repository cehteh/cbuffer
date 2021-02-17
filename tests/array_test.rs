use uninit::prelude::*;
mod shared;

#[test]
fn array_tx_test() {
    shared::write_cbuffer(&*b"foo\n");
}

#[test]
fn array_rx_test() {
    let mut read_buffer: [u8; 10] = [0; 10];
    let result = shared::read_cbuffer(&mut read_buffer);
    assert_eq!(result, b"foo\n");
}

#[test]
fn array_rx_uninit_test() {
    let mut read_buffer = uninit_array![u8; 10];
    let result = shared::read_cbuffer(&mut read_buffer);
    assert_eq!(result, b"foo\n");
}

#[test]
#[should_panic]
fn array_rx_cannotresize_test() {
    let mut read_buffer: [u8; 1] = [0; 1];
    let result = shared::read_cbuffer_resize(&mut read_buffer);
    assert_eq!(result, b"foo\n");
}

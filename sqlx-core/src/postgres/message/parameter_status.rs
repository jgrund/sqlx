use bytes::Bytes;

use crate::error::Error;
use crate::io::{BufExt, Decode};

#[derive(Debug)]
pub struct ParameterStatus {
    pub name: String,
    pub value: String,
}

impl Decode<'_> for ParameterStatus {
    fn decode_with(mut buf: Bytes, _: ()) -> Result<Self, Error> {
        let name = buf.get_str_nul()?;
        let value = buf.get_str_nul()?;

        Ok(Self { name, value })
    }
}

#[test]
fn test_decode_parameter_status() {
    const DATA: &[u8] = b"client_encoding\x00UTF8\x00";

    let m = ParameterStatus::decode(DATA.into()).unwrap();

    assert_eq!(&m.name, "client_encoding");
    assert_eq!(&m.value, "UTF8")
}

#[test]
fn test_decode_empty_parameter_status() {
    const DATA: &[u8] = b"\x00\x00";

    let m = ParameterStatus::decode(DATA.into()).unwrap();

    assert!(m.name.is_empty());
    assert!(m.value.is_empty());
}

#[cfg(all(test, not(debug_assertions)))]
#[bench]
fn bench_decode_parameter_status(b: &mut test::Bencher) {
    const DATA: &[u8] = b"client_encoding\x00UTF8\x00";

    b.iter(|| {
        ParameterStatus::decode(test::black_box(Bytes::from_static(DATA))).unwrap();
    });
}

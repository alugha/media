use super::*;
use std::io::Cursor;

#[test]
fn test_data_does_not_start_with_h264header() -> Result<()> {
    let test_function = |input: &[u8]| {
        let mut reader = H264Reader::new(Cursor::new(input));
        if let Err(err) = reader.next_nal() {
            assert_eq!(Error::ErrDataIsNotH264Stream, err);
        } else {
            assert!(false);
        }
    };

    test_function(&[2]);
    test_function(&[0, 2]);
    test_function(&[0, 0, 2]);
    test_function(&[0, 0, 2, 0]);
    test_function(&[0, 0, 0, 2]);

    Ok(())
}

#[test]
fn test_parse_header() -> Result<()> {
    let h264bytes = &[0x0, 0x0, 0x1, 0xAB];
    let mut reader = H264Reader::new(Cursor::new(h264bytes));

    let nal = reader.next_nal()?;

    assert_eq!(1, nal.data.len());
    assert!(nal.forbidden_zero_bit);
    assert_eq!(0, nal.picture_order_count);
    assert_eq!(1, nal.ref_idc);
    assert_eq!(NalUnitType::EndOfStream, nal.unit_type);

    Ok(())
}

#[test]
fn test_eof() -> Result<()> {
    let test_function = |input: &[u8]| {
        let mut reader = H264Reader::new(Cursor::new(input));
        if let Err(err) = reader.next_nal() {
            assert_eq!(Error::ErrIoEOF, err);
        } else {
            assert!(false);
        }
    };

    test_function(&[0, 0, 0, 1]);
    test_function(&[0, 0, 1]);
    test_function(&[]);

    Ok(())
}

#[test]
fn test_skip_sei() -> Result<()> {
    let h264bytes = &[
        0x0, 0x0, 0x0, 0x1, 0xAA, 0x0, 0x0, 0x0, 0x1, 0x6, // SEI
        0x0, 0x0, 0x0, 0x1, 0xAB,
    ];

    let mut reader = H264Reader::new(Cursor::new(h264bytes));

    let nal = reader.next_nal()?;
    assert_eq!(0xAA, nal.data[0]);

    let nal = reader.next_nal()?;
    assert_eq!(0xAB, nal.data[0]);

    Ok(())
}

#[test]
fn test_issue1734_next_nal() -> Result<()> {
    let tests: Vec<&[u8]> = vec![
        &[0x00, 0x00, 0x010, 0x00, 0x00, 0x01, 0x00, 0x00, 0x01],
        &[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x01],
    ];

    for test in tests {
        let mut reader = H264Reader::new(Cursor::new(test));

        // Just make sure it doesn't crash
        while reader.next_nal().is_ok() {
            //do nothing
        }
    }

    Ok(())
}

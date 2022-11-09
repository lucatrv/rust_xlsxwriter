// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxAlign, XlsxError};

mod common;

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_row_height(0, 30)?;

    let format1 = Format::new()
        .set_font_name("Arial")
        .set_bold()
        .set_locked()
        .set_rotation(0)
        .set_align(XlsxAlign::Left)
        .set_align(XlsxAlign::Bottom);

    let format2 = Format::new()
        .set_font_name("Arial")
        .set_bold()
        .set_locked()
        .set_rotation(90)
        .set_align(XlsxAlign::Center)
        .set_align(XlsxAlign::Bottom);

    worksheet.write_string(0, 0, "Foo", &format1)?;
    worksheet.write_string(0, 1, "Bar", &format2)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format02() {
    let test_runner = common::TestRunner::new()
        .set_name("format02")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

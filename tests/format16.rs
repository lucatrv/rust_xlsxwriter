// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError, XlsxPattern};

mod common;

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let pattern = Format::new().set_pattern(XlsxPattern::MediumGray);

    worksheet.write_blank(0, 0, &pattern)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format16() {
    let test_runner = common::TestRunner::new()
        .set_name("format16")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

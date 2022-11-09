// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to demonstrate setting the max column width.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_string_only(0, 0, "Foo")?;

    worksheet.set_column_width(0, 1000)?; // Should be limited to 255.

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap46_set_max_column_width() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap46")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test the creation of a simple rust_xlsxwriter file with repeat rows/cols.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string_only(0, 0, "Foo")?;

    worksheet.set_repeat_rows(0, 0)?;
    worksheet.set_repeat_columns(0, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_repeat03() {
    let test_runner = common::TestRunner::new()
        .set_name("repeat03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

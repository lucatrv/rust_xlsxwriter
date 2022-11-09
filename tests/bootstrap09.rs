// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format = Format::new().set_bold();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Hello", &format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap09_write_a_formatted_string() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap09")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

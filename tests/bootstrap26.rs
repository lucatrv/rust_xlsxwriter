// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with user defined column.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format1 = Format::new().set_bold();

    worksheet.write_string_only(2, 2, "Rust")?;
    worksheet.set_column_format(3, &format1)?;
    worksheet.set_column_format(2, &format1)?;
    worksheet.set_column_format(2, &format1)?; // Overwrite format.
    worksheet.set_column_format(3, &format1)?; // Overwrite format.

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap25_set_column_with_format() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap26")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxBorder, XlsxColor, XlsxDiagonalBorder, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with cell borders.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new()
        .set_border_diagonal(XlsxBorder::Thin)
        .set_border_diagonal_type(XlsxDiagonalBorder::BorderUp);

    let format2 = Format::new()
        .set_border_diagonal(XlsxBorder::Thin)
        .set_border_diagonal_type(XlsxDiagonalBorder::BorderDown);

    let format3 = Format::new()
        .set_border_diagonal(XlsxBorder::Thin)
        .set_border_diagonal_type(XlsxDiagonalBorder::BorderUpDown);

    let format4 = Format::new()
        .set_border(XlsxBorder::Thin)
        .set_border_diagonal(XlsxBorder::Thin)
        .set_border_diagonal_type(XlsxDiagonalBorder::BorderUpDown);

    let format5 = Format::new()
        .set_border(XlsxBorder::Thin)
        .set_border_color(XlsxColor::RGB(0x00B050))
        .set_border_diagonal(XlsxBorder::Thin)
        .set_border_diagonal_type(XlsxDiagonalBorder::BorderUpDown)
        .set_border_diagonal_color(XlsxColor::Red);

    let worksheet = workbook.add_worksheet();

    worksheet.write_blank(1, 1, &format1)?;
    worksheet.write_blank(3, 1, &format2)?;
    worksheet.write_blank(5, 1, &format3)?;
    worksheet.write_blank(7, 1, &format4)?;
    worksheet.write_blank(9, 1, &format5)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap33_diagonal_borders() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap33")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

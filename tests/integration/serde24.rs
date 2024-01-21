// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    CustomSerializeField, Format, SerializeFieldOptions, Table, TableColumn, Workbook, XlsxError,
};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_num_format("0.00;[Red]0.00");
    let format2 = Format::new().set_num_format("0.00_ ;\\-0.00\\ ");
    let format3 = Format::new().set_num_format("0.00_ ;[Red]\\-0.00\\ ");

    let worksheet = workbook.add_worksheet();

    worksheet.write(2, 2, "Foo")?;
    worksheet.write(3, 2, "Bar")?;
    worksheet.write(4, 2, "Baz")?;
    worksheet.write(5, 2, "Bop")?;

    worksheet.write(2, 3, 1234)?;
    worksheet.write(3, 3, 1256)?;
    worksheet.write(4, 3, 2234)?;
    worksheet.write(5, 3, 1324)?;

    worksheet.write(2, 4, 2000)?;
    worksheet.write(3, 4, 4000)?;
    worksheet.write(4, 4, 3000)?;
    worksheet.write(5, 4, 1000)?;

    worksheet.write(2, 5, 4321)?;
    worksheet.write(3, 5, 4320)?;
    worksheet.write(4, 5, 4332)?;
    worksheet.write(5, 5, 4333)?;

    // Manually set the indices to get the same order as the target file.
    worksheet.format_dxf_index(&format3);
    worksheet.format_dxf_index(&format2);
    worksheet.format_dxf_index(&format1);

    for col_num in 2..=5u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    let columns = vec![
        TableColumn::default(),
        TableColumn::new().set_format(&format1),
        TableColumn::new().set_format(&format2),
        TableColumn::new().set_format(&format3),
    ];

    let mut table = Table::new();
    table.set_columns(&columns);

    worksheet.add_table(1, 2, 5, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. Test Worksheet table.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let format1 = Format::new().set_num_format("0.00;[Red]0.00");
    let format2 = Format::new().set_num_format("0.00_ ;\\-0.00\\ ");
    let format3 = Format::new().set_num_format("0.00_ ;[Red]\\-0.00\\ ");

    let worksheet = workbook.add_worksheet();

    #[derive(Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct MyStruct<'a> {
        column1: &'a str,
        column2: u16,
        column3: u16,
        column4: u16,
    }

    let data = [
        MyStruct {
            column1: "Foo",
            column2: 1234,
            column3: 2000,
            column4: 4321,
        },
        MyStruct {
            column1: "Bar",
            column2: 1256,
            column3: 4000,
            column4: 4320,
        },
        MyStruct {
            column1: "Baz",
            column2: 2234,
            column3: 3000,
            column4: 4332,
        },
        MyStruct {
            column1: "Bop",
            column2: 1324,
            column3: 1000,
            column4: 4333,
        },
    ];

    // Manually set the indices to get the same order as the target file.
    worksheet.format_dxf_index(&format3);
    worksheet.format_dxf_index(&format2);
    worksheet.format_dxf_index(&format1);

    let columns = vec![
        TableColumn::default(),
        TableColumn::new().set_format(&format1),
        TableColumn::new().set_format(&format2),
        TableColumn::new().set_format(&format3),
    ];

    let mut table = Table::new();
    table.set_columns(&columns);

    let header_options = SerializeFieldOptions::new()
        .set_table(&table)
        .set_custom_headers(&[
            CustomSerializeField::new("Column1").set_column_width(10.288),
            CustomSerializeField::new("Column2").set_column_width(10.288),
            CustomSerializeField::new("Column3").set_column_width(10.288),
            CustomSerializeField::new("Column4").set_column_width(10.288),
        ]);

    worksheet.serialize_headers_with_options(1, 2, &data[0], &header_options)?;

    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde24_1() {
    let test_runner = common::TestRunner::new()
        .set_name("table14")
        .set_function(create_new_xlsx_file_1)
        .unique("serde24_1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde24_2() {
    let test_runner = common::TestRunner::new()
        .set_name("table14")
        .set_function(create_new_xlsx_file_2)
        .unique("serde24_2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{ExcelSerialize, Workbook, XlsxError};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write(1, 0, 1)?;
    worksheet.write(2, 0, 2)?;
    worksheet.write(3, 0, 3)?;

    worksheet.write(0, 1, "col2")?;
    worksheet.write(1, 1, 4)?;
    worksheet.write(2, 1, 5)?;
    worksheet.write(3, 1, 6)?;

    worksheet.write(0, 2, "col3")?;
    worksheet.write(1, 2, 7)?;
    worksheet.write(2, 2, "")?;
    worksheet.write(3, 2, 9)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization. With serde skip_serializing_if.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    fn should_skip(value: &u8) -> bool {
        *value == 8
    }

    // Create a serializable test struct.
    #[derive(Serialize, ExcelSerialize)]
    struct MyStruct {
        col1: u8,
        col2: u8,

        #[serde(skip_serializing_if = "should_skip")]
        col3: u8,
    }

    let data1 = MyStruct {
        col1: 1,
        col2: 4,
        col3: 7,
    };

    let data2 = MyStruct {
        col1: 2,
        col2: 5,
        col3: 8,
    };

    let data3 = MyStruct {
        col1: 3,
        col2: 6,
        col3: 9,
    };

    worksheet.set_serialize_headers::<MyStruct>(0, 0)?;

    worksheet.serialize(&data1)?;
    worksheet.serialize(&data2)?;
    worksheet.serialize(&data3)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde08_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde15")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde08_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde15")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
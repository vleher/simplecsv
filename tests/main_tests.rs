use simplecsv::parse_from_file;

#[test]
fn test_from_files() {
    test_from_file("tests/files/", "organizations-100000.csv", false);
    test_from_file("tests/files/", "test.csv", false);
}

fn test_from_file(path: &str, file_name: &str, has_header: bool) {
    let input_file_name = path.to_string() + file_name;
    let output_file_name = path.to_string() + "output-" + file_name;

    let csv = parse_from_file(&input_file_name, has_header).unwrap();
    let res = csv.save_to_file(&output_file_name);
    println!("{res:?}");
    assert!(res.is_ok());

    let outcsv = parse_from_file(&output_file_name, has_header).expect("Here");

    assert!(csv.header.is_empty(), "header is {:?}", csv.header);
    assert_eq!(csv.header, outcsv.header);
    assert_eq!(csv.data.len(), outcsv.data.len());
}

#[test]
fn test_from_files_with_header() {
    test_from_file_with_header("tests/files/", "organizations-100000.csv", true);
    test_from_file_with_header("tests/files/", "test.csv", true);
}

fn test_from_file_with_header(path: &str, file_name: &str, has_header: bool) {
    let input_file_name = path.to_string() + file_name;
    let output_file_name = path.to_string() + "output-" + file_name;

    let csv = parse_from_file(&input_file_name, has_header).unwrap();
    let res = csv.save_to_file(&output_file_name);

    assert!(res.is_ok());

    let outcsv = parse_from_file(&output_file_name, has_header).unwrap();

    assert!(!csv.header.is_empty());
    assert_eq!(csv.header, outcsv.header);
    assert_eq!(csv.data.len(), outcsv.data.len());
}

#[test]
fn test_get_values() {
    test_get_value("tests/files/organizations-100000.csv", true);
}

fn test_get_value(file_name: &str, has_header: bool) {
    let csv = parse_from_file(file_name, has_header).unwrap();

    let val00 = csv.get_value_by_index(0, 0).unwrap_or("EMPTY".to_string());
    let val12 = csv.get_value_by_index(1, 2).unwrap_or("ERROR".to_string());
    let val24 = csv.get_value_by_index(2, 4).unwrap_or("ERROR".to_string());
    let val55 = csv
        .get_value_by_index(100005, 5)
        .unwrap_or("ERROR".to_string());
    let val65 = csv
        .get_value_by_index(15, 500)
        .unwrap_or("ERROR".to_string());

    assert_eq!(val00, "1");
    assert_eq!(val12, "Walls-Mcdonald");
    assert_eq!(val24, "Tokelau");
    assert_eq!(val55, "ERROR");
    assert_eq!(val65, "ERROR");
}

#[test]
fn test_set_values() {
    test_set_value("tests/files/organizations-100000.csv", true);
}

fn test_set_value(file_name: &str, has_header: bool) {
    let mut csv = parse_from_file(file_name, has_header).unwrap();
    let flag = csv.set_value_by_index(0, 0, String::from("TestOne"));
    assert!(flag.is_ok());

    let flag = csv.set_value_by_index(1, 2, String::from("TestTwo"));
    assert!(flag.is_ok());

    let flag = csv.set_value_by_index(150000, 5, String::from("TestThree"));
    assert!(flag.is_err());

    let val00 = csv.get_value_by_index(0, 0).unwrap_or("EMPTY".to_string());
    let val12 = csv.get_value_by_index(1, 2).unwrap_or("ERROR".to_string());
    let val24 = csv.get_value_by_index(2, 4).unwrap_or("ERROR".to_string());
    let val55 = csv
        .get_value_by_index(5, 5000)
        .unwrap_or("ERROR".to_string());

    assert_eq!(val00, "TestOne");
    assert_eq!(val12, "TestTwo");
    assert_eq!(val24, "Tokelau");
    assert_eq!(val55, "ERROR");
}

#[test]
fn test_create_new_file() {
    let new_csv = simplecsv::new_csv_builder();

    let csv_file = new_csv
        .header(String::from("f1,f2"))
        .row(String::from("r1,r2"))
        .separator(',')
        .build();

    let res = csv_file.save_to_file("tests/files/new_file.csv");

    assert!(res.is_ok())
}

#[test]
fn test_new_file_with_custom() {
    let new_csv = simplecsv::new_csv_builder();

    let csv_file = new_csv
        .header(String::from("f1:f2,f3:f4"))
        .row(String::from("r1:r2-,-r:r5"))
        .separator(':')
        .build();

    let res = csv_file.save_to_file("tests/files/new_file.csv");

    assert!(res.is_ok())
}

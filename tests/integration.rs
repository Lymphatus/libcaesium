use libcaesium;

#[test]
fn unknown_file_type() {
    let output = "tests/samples/output/should_not_be_there";
    let params = libcaesium::initialize_parameters();
    let result = libcaesium::compress(String::from("tests/samples/output/.gitkeep"),
                      String::from(output),
                      params);
    assert!(result.is_err())
}

#[test]
fn unknown_file_type() {
    let output = "tests/samples/output/should_not_be_there";
    let params = caesium::initialize_parameters();
    let result = caesium::compress(String::from("tests/samples/output/.gitkeep"),
                      String::from(output),
                      params);
    assert!(result.is_err())
}

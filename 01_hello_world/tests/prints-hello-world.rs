use assert_cmd::Command;

#[test]
fn prints_hello_world() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("hello_world")? //build and run this binary
        .assert() //validate output
        .stdout("hello world!\n"); //assert that the stdout matches this text exactly
    Ok(())
}

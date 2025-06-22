use assert_cmd::Command;

#[test]
fn adds_numbers_to_6() -> Result<(), Box<dyn std::error::Error>> { 
    Command::cargo_bin("bin_files")? //build and run this binary
        .assert() //validate output
        .stdout("6\n"); //assert that the stdout matches this text exactly
    Ok(())
}

use std::{fs, error::Error};

use assert_cmd::Command;
use predicates::prelude::*;


type TestResult = Result<(),Box<dyn Error>>;

#[test]
fn dies_no_args() -> TestResult{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    
    Ok(())
}


fn run(args:&[&str],file:&str) -> TestResult{

    let expected = fs::read_to_string(file)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

#[test]
fn hello1(){
    let file = "tests/expected/hello1.txt";
    run(&["Hello there"], file);
}
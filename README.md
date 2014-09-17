# Basic Rust Test Runner ![build status](https://travis-ci.org/rgawdzik/rust_testrunner.svg?branch=master)
The philosophy is to KISS. One line tests that return either {nothing, something}, to indicate either
success or failure.

Anything you can test in bash, you can test in this Test Runner.

## Installation
 0. Make sure to have the latest rust and cargo with ```curl https://static.rust-lang.org/rustup.sh | sudo bash```.
 1. ```cargo build```
 2. ```sudo mv ./target/cs246test /usr/local/bin/```
 3. Make sure to have ```/usr/local/bin``` in your $PATH.

## Usage

Init the suite with ```cs246test init```. Then start writing tests:

 - ```fixtures/``` contain your test data, like myfile.txt
 - ```tests.sh``` contain your actual tests, all in one file.

```
#Dynamic diff syntax:
#diff <(actual command) <(output expected)

#Assignment n Question 1 tests:
diff <(egrep 'hi') <(cat mytest.txt) #=> (nothing) || error
diff <(egrep 'expression' input.txt) <(cat mytest.txt) #=> Either (nothing) || error
diff <(egrep 'expression' input.txt) <("should match") #=> (nothing) || error
egrep 'hi' myfile.txt #=> should not match
diff <(./test < input) <(echo 0) #=> should output 0
```

 Run ```cs246test``` to begin the test runner.
 Run ```cs246test -h``` or ```cs246test --help``` for more help.

### TODO
 - Should run recursively in the local directory too, which is useful if you have nested tests.

# Parameterized test framework for Rust

`p_test` uses a procedural macro to help you write parameterized tests.
If you have a function to test, you can write a parameterized test like the following:

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[p_test(
    test_sum,              // test module name: should be a valid module name
    (sum_1_1, (1, 1), 2),  // test case sum_1_1: (test_case_name, (arguments, ...), expected)
    (sum_2_3, (2, 3), 5),  // test case sum_2_3
    (sum_4_5, (4, 5), 9),  // test case sum_4_5
)]
fn test_sum((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

The first input to the attribute is test name, which will be translated to test module name.
After that, you need to provide a list of tuples containing test case name, arguments (also tuple), and expected value.

The output of the test run will be similar to:

```console
$ cargo test
...

running 3 tests
test test_sum::sum_1_1 ... ok
test test_sum::sum_2_3 ... ok
test test_sum::sum_4_5 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
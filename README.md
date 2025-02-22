# Parameterized test framework for Rust

`p_test` uses a procedural macro to help you write parameterized tests.
If you have a function to test, you can write a parameterized test like the following:

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[p_test(
    (sum_1_1, (1, 1), 2),  // test case sum_1_1: (test_case_name, (arguments, ...), expected)
    (sum_2_3, (2, 3), 5),  // test case sum_2_3
    (sum_4_5, (4, 5), 9),  // test case sum_4_5
)]
fn test_sum((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

The input of the `p_test` attribute is a list of tuples, where each tuple repreesnts
`(case_name, (argument_list), expected)`. Here, the `(argument_list)` is a tuple 
represents the argument of the function to test. The test function name `test_sum` will be 
translated to test module name.

So, the above example will be expanded like the following:

```rust
fn test_sum((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[cfg(test)]
mod test_sum {
    use super::*;
    #[test]
    fn sum_1_1() {
        test_sum((1, 1), 2);
    }
    #[test]
    fn sum_2_3() {
        test_sum((2, 3), 5);
    }
    #[test]
    fn sum_4_5() {
        test_sum((4, 5), 9);
    }
}
```

## Output
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

## Note
Before `0.1.3`, it was required to provide the test module name.

```rust
#[p_test(
    test_sum_mod, // test module name: should be a valid module name
    (sum_1_1, (1, 1), 2),
    (sum_2_3, (2, 3), 5),
    (sum_4_5, (4, 5), 9),
)]
fn test_sum((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

This may not be useful as we can use the function name as the module name.
You still can privide the module name, in case you want to use different name
for module and test function. But it is mainly left for backward compatibility.

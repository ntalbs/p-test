# P-test
This crate provides the `p_test` macro, which is to make writing
parameterized tests easier.

## Example
Suppose that you have a function.

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

You can write a parameterized test with `p_test` macro like this:

```rust
use p_test::p_test;

// Parameterized test
#[p_test(
    (sum_1_1, (1, 1), 2),  // test case sum_1_1: (test_case_name, (arguments, ...), expected)
    (sum_2_3, (2, 3), 5),  // test case sum_2_3
    (sum_4_5, (4, 5), 9),  // test case sum_4_5
)]
fn test_sum((a, b): (i32, i32), expected: i32) {
   assert_eq!(sum(a, b), expected);
}
```

The input for the `p_test` attribute is a list of tuples, where each
tuple represents a test case. The format of the test case tuple is
`(case_name, (argument_list), expected)`. The `(argument_list)` inside
the test case tuple is another tuple that represents the argument of
the function to test. The test function name `test_sum` will be used
to test module name. The above example will be expanded like the
following:

```rust
// This parameterized function is copied
fn test_sum((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}
// The macro expanded.
// Each of the case name become a test function
// which invokes parameterized function.
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

Each test case has their name, so you can find which test cases
failed.  This is especially useful when you have long list of test
cases.

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

This may not be useful as we can use the function name as the module
name. You still can provide the module name, in case you want to use
different name for module and test function. But it is mainly left for
backward compatibility.

# P-test
This crate provides the `p_test` macro, which is to make writing
parameterized tests easier.

## Basic Usage
Suppose that you have a function you want to test.

```rust
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
```

With `p_test` macro, you can write a parameterized test like this:

```rust
#[p_test(
    (1, 1, 2),
    (2, 3, 5),
    (4, 5, 9),
)]
fn test_sum(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

The above code will be expanded like the below:

```rust
fn test_sum(expected: i32, a: i32, b: i32) {
    assert_eq!(sum(a, b), expected);
}

#[cfg(test)]
mod test_sum {
    use super::*;
    #[test]
    fn case_1() {
        test_sum(2, 1, 1);
    }
    #[test]
    fn case_2() {
        test_sum(5, 2, 3);
    }
    #[test]
    fn case_3() {
        test_sum(9, 4, 5);
    }
}
```

As you can see, the case names are auto-generated with `case_{n}`
format. Note that the parameterized test function name `test_sum` is
used for the module name.  The test output will look like the
following:

```console
$ cargo test
...
test test_sum_no_name::case_1 ... ok
test test_sum_no_name::case_2 ... ok
test test_sum_no_name::case_3 ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This is a handy way of writing parameterized tests. However, when you
have a long list of parameters, finding a failed test would be
challenging.

## Use Arguments For Case Name

In any cases that the case name `case_{n}` is not good enough, you
case use `use_args_for_case_name=true`. This direct `p_test` macro to
use the values of the arguments for test case name generation.

```rust
#[p_test(
    use_args_for_case_name = true,
    (1, 1, 2),
    (2, 3, 5),
    (4, 5, 9),
)]
fn test_sum(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

The test case names will be generated with the arguments,
e.g. `_1__1__2`, so the output will look like the following:

```console
$ cargo test
...
test test_sum::_1__1__2 ... ok
test test_sum::_2__3__5 ... ok
test test_sum::_4__5__9 ... ok
...
```

This will make it a little easier to locate the failed test case.

## Specifying Explicit Case Names

If you prefer explicitness, you can specify each test case name like
this:

```rust
#[p_test(
    sum_1_1, (1, 1, 2), // test case sum_1_1
    sum_2_3, (2, 3, 5), // test case sum_2_3
    sum_4_5, (4, 5, 9), // test case sum_4_5
)]
fn test_sum(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

The first elements in each line (i.e., `sum_1_1`, `sum_2_3`, etc.)
will be used as a test function names when the macro is expanded. So,
they should be valid function names.

You can also use string literal for case name, like the following:

```rust
#[p_test(
    "sum(1,1)", (1, 1, 2),
    "sum(2,3)", (2, 3, 5),
    "sum(4,5)", (4, 5, 9),
)]
fn test_sum(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}
```

In this case, the case names will be transformed to a valid function
names by replacing non-alphanumeric characters with `_`. For example,
`"sum(1,1)"` will be converted to `sum_1_1_`.

The above code will be expanded like the below:

```rust
// This parameterized function is copied
fn test_sum(expected: i32, a: i32, b: i32) {
    assert_eq!(sum(a, b), expected);
}

// The macro expanded.
// Each of the case name become a test function
// which invokes parameterized function.
#[cfg(test)]
mod test_sum {
    use super::*;
    #[test]
    fn sum_1_1_() {
        test_sum(2, 1, 1);
    }
    #[test]
    fn sum_2_3_() {
        test_sum(5, 2, 3);
    }
    #[test]
    fn sum_4_5_() {
        test_sum(9, 4, 5);
    }
}
```

## Parameter Order

We set the expected value at the end of each test case. But the order
of arguments are totally up to you. You can use the first argument as
an expected value.

```rust
#[p_test(
    sum_1_1, (2, 1, 1),
    sum_2_3, (5, 2, 3),
    sum_4_5, (9, 4, 5),
)]
fn test_sum(expected: i32, a: i32, b: i32) {
    assert_eq!(sum(a, b), expected);
}
```

But the order should match with the parameter list of the test
function, `test_sum` in this example.

You can explicitly distinguish the argument list and the expected
value like the below code, if you like.

```rust
use p_test::p_test;

#[p_test(
    sum_1_1, ((1, 1), 2),  // test case sum_1_1
    sum_2_3, ((2, 3), 5),  // test case sum_2_3
    sum_4_5, ((4, 5), 9),  // test case sum_4_5
)]
fn test_sum((a, b): (i32, i32), expected: i32) {
   assert_eq!(sum(a, b), expected);
}
```

## Testing async function

`p_test` macro works well with `tokio::test`.

```rust
async fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[tokio::test]
#[p_test(
    sum_1_1, (1, 1, 2),
    sum_1_2, (1, 2, 3),
    sum_2_2, (2, 2, 4),
    sum_2_3, (2, 3, 5),
)]
async fn test_sum_async_tokio(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b).await, expected);
}
```

## Note
There are breaking changes in `1.0.0`.

- Before `1.0.0`, you could explicitly specify test module name. It
  was required in the early versions, but became optional from
  `0.1.3`. It is now removed for simplicity.
- The test case names used to be specified inside tuples. As of
  `0.1.8` it became possible to specify test case names by literal
  string. However, this caused an issue when the first parameter of
  the test function is string. Now, test case name should be specified
  before argument tuples.
- `use_args_for_case_name` is introduced.

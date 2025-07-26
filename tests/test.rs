use p_test::p_test;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[p_test(
    sum_1_1, (1, 1, 2),
    sum_1_2, (1, 2, 3),
    sum_2_2, (2, 2, 4),
    sum_2_3, (2, 3, 5),
)]
fn test_sum_case_name_ident(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[p_test(
    "sum(1,1)", (1, 1, 2),
    "sum(1,2)", (1, 2, 3),
    "sum(2,2)", (2, 2, 4),
    "sum(2,3)", (2, 3, 5),
)]
fn test_sum_case_name_litstr(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[p_test(
    sum_1_1, ((1, 1), 2),
    sum_1_2, ((1, 2), 3),
    sum_2_2, ((2, 2), 4),
    sum_2_3, ((2, 3), 5),
)]
fn test_sum_case_name_nested_tuple((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[p_test(
    (1, 1, 2),
    (1, 2, 3),
    (2, 2, 4),
    (2, 3, 5),
)]
fn test_sum_no_case_name(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[p_test(
    ((1, 1), 2),
    ((1, 2), 3),
    ((2, 2), 4),
    ((2, 3), 5),
)]
fn test_sum_no_case_name_nested_tuple((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[p_test(
    (1, 1, 2),
    (1, 2, 3),
    (2, 2, 4),
    (2, 3, 5),
    (5, 6, 11),
    (7, 4, 11),
    (8, 3, 11),
    (9, 2, 11),
    (10, 1, 11),
    (11, 0, 11),
    (12, -1, 11),
    (13, -2, 11),
)]
fn test_sum_more_than_10_cases_without_name(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// use_args_for_case_name is true
#[p_test(
    use_args_for_case_name = true,
    (1, 1, 2),
    (1, 2, 3),
    (2, 2, 4),
    (2, 3, 5),
)]
fn test_sum_use_args_for_case_name(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// use_args_for_case_name is true
/// use_args_for_case_name should be ignored if case name is provided
#[p_test(
    use_args_for_case_name = true,
    t1, (1, 1, 2),
    t2, (1, 2, 3),
    t3, (2, 2, 4),
    t4, (2, 3, 5),
)]
fn test_sum_use_args_for_case_name_but_case_name_specified(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// use_args_for_case_name is false
#[p_test(
    use_args_for_case_name = false,
    (1, 1, 2),
    (1, 2, 3),
    (2, 2, 4),
    (2, 3, 5),
)]
fn test_sum_use_args_for_case_name_false(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

#[p_test(
    ("hello", 'h'),
    ("world", 'w'),
)]
fn test_string_to_first_char(s: &str, expected: char) {
    assert_eq!(s.chars().next().unwrap(), expected);
}

#[p_test(
    use_args_for_case_name = true,
    ("hello", 'h'),
    ("world", 'w'),
)]
fn test_string_to_first_char_use_args_for_case_name(s: &str, expected: char) {
    assert_eq!(s.chars().next().unwrap(), expected);
}


/// Parameterized test with no arguments.
/// This is possible, but not useful.
#[p_test(
    use_args_for_case_name = true,
    (),
    (),
    (),
)]
fn dummy_test_with_no_args() {}

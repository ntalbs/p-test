use p_test::p_test;

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

/// - module name: None
/// - arguments: simple list
/// - case name: Ident
#[p_test(
    sum_1_1, (1, 1, 2),
    sum_1_2, (1, 2, 3),
    sum_2_2, (2, 2, 4),
    sum_2_3, (2, 3, 5),
)]
fn test_sum_no_without_module_name(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// - module name: None
/// - arguments: simple list
/// - case name: LitStr
#[p_test(
    "sum(1,1)", (1, 1, 2),
    "sum(1,2)", (1, 2, 3),
    "sum(2,2)", (2, 2, 4),
    "sum(2,3)", (2, 3, 5),
)]
fn test_sum_no_without_module_name_test_case_name_litstr(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// - module name: None
/// - arguments: tuple for function arguments, expected value is separeted.
/// - case name: Ident
#[p_test(
    sum_1_1, ((1, 1), 2),
    sum_1_2, ((1, 2), 3),
    sum_2_2, ((2, 2), 4),
    sum_2_3, ((2, 3), 5),
)]
fn test_sum_arg_tuple_without_module_name((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// - module name: None
/// - arguments: simple list
/// - case name: None
#[p_test(
    (1, 1, 2),
    (1, 2, 3),
    (2, 2, 4),
    (2, 3, 5),
)]
fn test_sum_no_without_module_name_no_case_name(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// - module name: None
/// - arguments: tuple for function arguments, expected value is separeted.
/// - case name: None
#[p_test(
    ((1, 1), 2),
    ((1, 2), 3),
    ((2, 2), 4),
    ((2, 3), 5),
)]
fn test_sum_arg_tuple_without_module_nameno_case_name((a, b): (i32, i32), expected: i32) {
    assert_eq!(sum(a, b), expected);
}

/// number of cases is more than 10 to check that the case name is generated in correct format.
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
fn test_sum_more_than_10_cases(a: i32, b: i32, expected: i32) {
    assert_eq!(sum(a, b), expected);
}

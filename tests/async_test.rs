use p_test::p_test;

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

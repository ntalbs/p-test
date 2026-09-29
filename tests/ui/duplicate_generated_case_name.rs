use p_test::p_test;

#[p_test(
    use_args_for_case_name = true,
    ("a", "b"),
    ("a", "b"),
)]
fn sample(s: &str, t: &str) {
    let _ = (s, t);
}

fn main() {}

use rstest::rstest;
use week_6::{service};
use week_6::service::NameRepository;

/// 설치 명령어: cargo add rstest
/// 문서: https://docs.rs/rstest/latest/rstest/
#[test]
fn add_test() {
    assert_eq!(3, service::add(1, 2))
}

#[rstest]
#[case(1, 1, 2)]
#[case(1, 2, 3)]
#[case("1", "2", 3)] /// MagicConversion: FromStr trait을 구현한 모든 타입에 사용 가능
fn parameterized_add_test(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(expected, service::add(a, b))
}

/// values의 모든 조합을 만들어서 테스
#[rstest]
fn combination_add_test(
    #[values(1, 2, 3)]
    a: i32,
    #[values(4, 5, 6)]
    b: i32
) {
    let expected = a + b;
    println!("a = {}, b = {}, expected = {}", a, b, expected);
    assert_eq!(expected, service::add(a, b))
}

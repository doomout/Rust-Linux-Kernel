//cargo new adder --lib 로 생성한 lib.rs
//main 함수가 없고 test 코드가 생성 되었다.
// cargo test 로 테스트 실행 해야 한다.

// add 함수는 두 개의 usize 타입의 인수를 받아 그 합을 반환
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(test)]는 이어지는 코드가 테스트 환경에서만 컴파일되도록 지시
#[cfg(test)]
mod tests {
    // 상위 스코프의 모든 항목을 현재 테스트 모듈로 가져온다.
    use super::*;

    // 테스트 함수다.
    #[test]
    fn it_works() {
        let result = add(2, 2); //add 함수에 2, 2를 인수로 전달
        assert_eq!(result, 4); //결과가 4와 같은지 확인
    }
}

/*실행 결과
running 1 test
test tests::it_works ... ok 
*/
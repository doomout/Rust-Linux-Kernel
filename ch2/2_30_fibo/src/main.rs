//피보나치 함수 만들기
use std::io;

fn main() {
    println!("n번째 수를 입력해주세요.");

    let mut n = String::new(); // 사용자 입력을 받을 변수 n 선언
    io::stdin().read_line(&mut n); // 표준 입력으로부터 한 줄을 읽어 n에 저장
    let n: i32 = n.trim().parse().unwrap(); // 입력된 문자열을 정수로 변환
    println!("입력 수: {}", n); // 입력된 수 출력

    let ret = fibo(n); // fibo 함수 호출하여 결과를 ret에 저장
    println!("결과: {}", ret); // 결과 출력
}

/* 실행결과
n번째 수를 입력해주세요.
6
입력 수: 6
1, 1, 2, 3, 5, 8,
결과: 8 
*/

// 피보나치 수열을 계산하는 함수
fn fibo(n: i32) -> i32 {
    let mut next = 0; // 다음 피보나치 수를 저장할 변수
    let mut t1 = 1; // 피보나치 수열의 첫 번째 수
    let mut t2 = 1; // 피보나치 수열의 두 번째 수
    let mut counter = 2; // 피보나치 수열의 현재 위치를 추적하는 카운터. 이미 2개의 수가 있으므로 2로 초기화
    
    print!("1, 1, ");   // 첫 번째와 두 번째 수를 출력

    // n번째 수까지 반복
    while counter < n {       
        next = t1 + t2;       // 다음 피보나치 수 계산
        t1 = t2;              // t1을 t2로 업데이트
        t2 = next;            // t2를 다음 피보나치 수로 업데이트
        print!("{}, ", next); // 다음 피보나치 수 출력

        counter += 1; // 카운터 증가
    }

    println!(""); // 줄 바꿈
    next //계산된 피보나치 수 반환. 반환값 뒤에는 세미콜론이 없음
}

//테스트 코드: cargo test 로 실행
#[test]
fn fibo_test() {
    assert_eq!(fibo(6), 8); // 6번째 수열은 8이여야 함
    assert_eq!(fibo(7), 13); // 7번째 수열은 13이여야 함
}

/* 테스트 실행 결과
running 1 test
test fibo_test ... ok
*/
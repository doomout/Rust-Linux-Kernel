// let if 예제
// if문은 조건에 따라 다른 값을 반환할 수 있습니다.

fn main() {
    let condition = true;

    // if문의 값을 ret에 저장
    let ret = if condition == true {
        String::from("조건이 참 입니다.") // ;을 붙이면 컴파일 오류 발생
    } else {
        String::from("조건이 거짓 입니다.") // ;을 붙이면 컴파일 오류 발생
    }; // 여기는 ;을 붙여야 한다.
    
    println!("ret={}", ret);
}

/*실행결과
ret=조건이 참 입니다.
*/
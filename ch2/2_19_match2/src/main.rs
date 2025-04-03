;//let- mathc 문
// match 문: 하나의 조건으로 여러 구문을 분기
// switch 문과 같다.
// match 문은 조건에 따라 여러 구문을 분기할 수 있습니다.
// match 문은 switch 문과 비슷합니다.
fn main() {
    let var = 1;
    let ret = match var {
        1 => String::from("하나"),
        2 => String::from("둘"),
        _ => String::from("기타"),
    }; // 세미콜론을 붙여야 합니다.
    
    println!("ret={}", ret);
}

/*실행결과
ret=하나
*/

// match 문: 하나의 조건으로 여러 구문을 분기
// switch 문과 같다.
fn main() {
    let var = 1;
    match var {
        1 => println!("하나"),
        2 => println!("둘"),
        _ => println!("기타"), // default 조건
    }
}
/*실행결과
하나
 */
fn main() {
    let var = 1;
    println!("var={}", var);
    //var = var + 1; // 컴파일 오류 발생
    let var = var + 1; // 기존의 var는 소멸되고 var이라는 새로운 변수가 생성(이것을 shadowing이라고 함)
    println!("var={}", var);
}

/*실행 결과
var=1
var=2
*/
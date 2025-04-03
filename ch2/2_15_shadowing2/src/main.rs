// 다른 자료형으로 섀도잉
// Rust에서는 같은 이름의 변수를 재정의할 수 있다.
// 이때, 기존 변수는 사라지고 새로운 변수가 생성된다.
// 이 과정을 섀도잉(shadowing)이라고 한다.
fn main() {
    let var = 1; 
    println!("var={}", var);
    let var = String::from("기존 var를 쉐도잉");
    println!("var={}", var);
}
/*실행 결과
var=1
var=기존 var를 쉐도잉
*/
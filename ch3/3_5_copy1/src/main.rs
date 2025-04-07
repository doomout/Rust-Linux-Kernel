// 데이터를 복제하는 예제
fn main() {
    let mut x = 10;
    let y = x; //clone 함수 사용하지 않아도 복사가 된다.

    println!("x: {} y: {}", x, y); //x: 10 y: 10
    
    x = 20; // x를 수정
    println!("x: {} y: {}", x, y); //x: 20 y: 10
}

/*실행결과
x: 10 y: 10
x: 20 y: 10
*/
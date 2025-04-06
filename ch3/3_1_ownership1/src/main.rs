// 소유권을 다른 변수로 이관하는 예제
fn main() {
    //새로운 문자열 변수 생성
    let s1 = String::from("Hello Rust!");

    // s2로 소유권을 이동
    let s2 = s1; 

   // s1은 소유권을 상실했기 때문에 s1에 접근하는 순간 컴파일 에러 발생
   println!("{}", s1); 
}

/* 실행 결과
error[E0382]: borrow of moved value: `s1`
10 |    println!("{}", s1); 
   |                   ^^ value borrowed here after move
   |

error: aborting due to 1 previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
*/
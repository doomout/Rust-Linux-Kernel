//move 키워드로 소유권을 클로저에 이전
fn main() {
    let s = String::from("Hello");
    let f = move || { // move클로저는 소유권을 이전합니다.
        println!("s: {}", s); // 여기서 s의 소유권을 가져갑니다.
    };

    f();
    println!("s: {}", s); // 컴파일 오류: s의 소유권이 없습니다.
}

/* 실행결과
error[E0382]: borrow of moved value: `s`
 --> main.rs:8:23
  |
2 |     let s = String::from("Hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     let f = move || { // move클로저는 소유권을 이전합니다.
  |             ------- value moved into closure here
4 |         println!("s: {}", s); // 여기서 s의 소유권을 가져갑니다.
  |                           - variable moved due to use in closure
...
8 |     println!("s: {}", s); // 컴파일 오류: s의 소유권이 없습니다.
  |                       ^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 1 previous error
 */
//버퍼 오버플로우 테스트
// Rust는 안전한 언어이기 때문에, 배열의 범위를 벗어난 인덱스에 접근하면 컴파일 에러가 발생합니다.
// 하지만, unsafe 블록을 사용하면 이러한 안전성 검사를 우회할 수 있습니다.
// 이 코드는 Rust의 안전성을 무시하고, 배열의 범위를 벗어난 인덱스에 접근하는 예시입니다.

fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5]; // i32타입을 가지는 5개 원소
    arr[6] = 7; // 6번 인덱스에 값을 기입

    println!("arr[6]={}", arr[6]);  // 6번 인덱스를 참조
}
/**실행결과
 * error: this operation will panic at runtime
 --> main.rs:8:5
  |
8 |     arr[6] = 7; // 6번 인덱스에 값을 기입
  |     ^^^^^^ index out of bounds: the length is 5 but the index is 6
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: this operation will panic at runtime
  --> main.rs:10:27
   |
10 |     println!("arr[6]={}", arr[6]);  // 6번 인덱스를 참조
   |                           ^^^^^^ index out of bounds: the length is 5 but the index is 6

error: aborting due to 2 previous errors

 */
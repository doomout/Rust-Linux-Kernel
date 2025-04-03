//러스트 기본적으로 불변 변수로 선언된다.
// 변수를 선언할 때 mut 키워드를 사용하여 가변 변수로 만들 수 있다.
// 가변 변수는 값을 변경할 수 있지만, 불변 변수는 값을 변경할 수 없다.
// 불변 변수는 기본적으로 안전하고, 가변 변수는 안전하지 않다.


fn main() {
    let var = 1; // 불변 변수 생성
    var = 2; // 컴파일 오류 발생
}
/**실행결과
 warning: variable `var` is assigned to, but never used
 --> main.rs:8:9
  |
8 |     let var = 1; // 불변 변수 생성
  |         ^^^
  |
  = note: consider using `_var` instead
  = note: `#[warn(unused_variables)]` on by default

warning: value assigned to `var` is never read
 --> main.rs:9:5
  |
9 |     var = 2; // 컴파일 오류 발생
  |     ^^^
  |
  = help: maybe it is overwritten before being read?
  = note: `#[warn(unused_assignments)]` on by default

error[E0384]: cannot assign twice to immutable variable `var`
 --> main.rs:9:5
  |
8 |     let var = 1; // 불변 변수 생성
  |         --- first assignment to `var`
9 |     var = 2; // 컴파일 오류 발생
  |     ^^^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
8 |     let mut var = 1; // 불변 변수 생성
  |         +++

error: aborting due to 1 previous error; 2 warnings emitted

For more information about this error, try `rustc --explain E0384`.

 */
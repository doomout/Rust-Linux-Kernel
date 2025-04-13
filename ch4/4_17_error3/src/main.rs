// 복구 불가능한 오류 에제
// 다양한 실행법으로 오류 표시 가능
fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let ret = div(1, 0);
    println!("ret: {}", ret);
}

/* vscode run 실행시
thread 'main' panicked at main.rs:4:5:
attempt to divide by zero
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace 
*/

/*터미널(PowerShell)에서 $env:RUST_BACKTRACE=1; cargo run 실행시
thread 'main' panicked at src\main.rs:4:5:
attempt to divide by zero
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library\std\src\panicking.rs:692
   1: core::panicking::panic_fmt
             at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library\core\src\panicking.rs:75
   2: core::panicking::panic_const::panic_const_div_by_zero
             at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181/library\core\src\panicking.rs:178
   3: error3::div
             at .\src\main.rs:4
   4: error3::main
             at .\src\main.rs:8
   5: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
             at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181\library\core\src\ops\function.rs:250
   6: core::hint::black_box
             at /rustc/4eb161250e340c8f48f66e2b929ef4a5bed7c181\library\core\src\hint.rs:475
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
 */


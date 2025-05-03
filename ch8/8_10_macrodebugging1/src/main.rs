macro_rules! S {
    ($e:expr) => {String::from($e)};
}

fn main() {
    let world = S!("World");
    println!("Hello, {}!", world);
}

/* 매크로 디버깅을 하기 위해서 사전 설치 해야 하는 것
 rustup toolchain install nightly

그 후에 아래 명령어를 실행하면
 rustc --pretty=expanded main.rs > main.rs.expanded
*/


/*매크로 디버깅 결과
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::rust_2015::*;
#[macro_use]
extern crate std;
macro_rules! S { ($e:expr) => {String::from($e)}; }

fn main() {
    let world = String::from("World");
    { ::std::io::_print(format_args!("Hello, {0}!\n", world)); };
} 
*/
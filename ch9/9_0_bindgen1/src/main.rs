/* Cargo.toml 에 의존성 추가
[build-dependencies]
bindgen = "0.64.0"
*/

//C와 호환되는 문자열을 다루기 위한 'CString' 타입을 사용한다.
use std::ffi::CString;

// 이전 단계에서 생성된 ''bindings.rs' 파일을 포함한다.
mod bindings {
    // 'OUT_DIR' 환경 변수를 사용하여 생성된 바인딩 파일을 포함한다.
    // 'OUT_DIR'은 Cargo가 빌드 아티팩트를 저장하는 디렉터리 경로를 나타낸다.
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    // C와 호환되는 문자열을 다루기 위한 'CString' 타입을 사용한다.
    // 'CString::new'는 Rust 문자열을 C 문자열로 변환한다.
    let c_to_print = CString::new("Hello Rust").expect("CString::new failed");

    // 'unsafe' 블록은 Rust의 안전성 규칙을 우회하는 코드 블록이다.
    // C 라이브러리의 함수를 호출할 때는 'unsafe' 블록이 필요하다.
    // 'bindings::hello'는 C 라이브러리에서 정의된 'hello' 함수를 호출한다.
    // 'c_to_print.as_ptr()'는 C 문자열의 포인터를 가져온다.
    // 'as_ptr()'는 'CString'의 내부 포인터를 반환한다.
    unsafe {
        bindings::hello(c_to_print.as_ptr());
    }
}

/*실행 명령어
LD_LIBRARY_PATH=. cargo run

실행 결과
from rust: Hello Rust
*/
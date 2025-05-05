/* 의존성 추가: libc 사용하기 위한
[dependencies]
libc = "0.2"
*/

// libc : C 언어의 표준 라이브러리
// C 언어의 표준 라이브러리인 libc를 사용하여 C 언어의 printf 함수를 호출하는 예제
fn main() {
    let message = "Hello, world!\0".as_ptr() as *const libc::c_char;
    unsafe {
        libc::printf(message);
    }
}

/*실행결과
Hello, world!
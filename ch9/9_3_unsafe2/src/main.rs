// 메모리 주소에 직접 접근 예제
use std::mem;

fn main() {
    // unsafe 블록을 사용하여 메모리 할당 및 해제를 수행
    // libc 크레이트를 사용하여 C 스타일의 메모리 할당 및 해제를 수행
    // libc 크레이트는 C 라이브러리의 기능을 Rust에서 사용할 수 있도록 해준다.
    unsafe {
        // libc::malloc은 C에서 메모리를 동적으로 할당하는 함수이다.
        let ptr: *mut i32 = libc::malloc(mem::size_of::<i32>()) as *mut i32;

        // 포인터가 null 인 경우 메모리 할당에 실패한 것이므로 프로그램 종료
        if ptr.is_null() {
            panic!("메모리 할당 실패");
        }

        let val: *mut i32 = ptr; // ptr과 동일한 주소를 갖는 변수 생성

        // val 포인터가 가리키는 메모리 위치에 정수 123을 저장
        // unsafe 블록 내에서만 포인터를 사용하여 메모리에 접근할 수 있다.
        // unsafe 블록 외부에서는 안전한 방법으로 메모리에 접근해야 한다.
        *val = 123;

        // 포인터를 사용하여 메모리에서 값을 읽어온다.
        println!("ptr={}", *ptr);

        // 메모리 해제를 위해 libc::free 함수를 사용
        // 메모리 해제를 하지 않으면 메모리 누수가 발생할 수 있다.
        libc::free(ptr as *mut libc::c_void);
    }
}

/*실행결과
ptr=123
*/
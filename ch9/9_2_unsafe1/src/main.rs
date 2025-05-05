//배열의 메모리를 복사하는 예제
fn main() {
    let src = [1, 2, 3]; // 원본 배열
    let mut dest = [0; 3]; // 대상 배열, 0으로 초기화

    // 안전하지 않은 작업을 수행하기 위해 unsafe 블록을 사용
    // src 배열의 메모리를 dest 배열로 복사
    // 이때 복사되는 길이(src.len())는 dest의 크기보다 작거나 같아야 한다.
    // 그렇지 않으면 메모리 안전성 문제가 발생할 수 있다.
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), dest.as_mut_ptr(), src.len());
    }

    println!("dest: {:?}", dest);
}

/*실행결과
dest: [1, 2, 3]
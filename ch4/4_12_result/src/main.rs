//파일 시스템 작업을 위한 File 구조체를 사용하기 위해
use std::fs::File;

fn main() {
    // "test.txt"라는 파일을 열려고 시도하며, 실패시 패닉을 일으킨다.
    // unwrap() 메서드는 Result 가 ok값이면 그 값을 반환하고, 
    // Err값이면 panic을 발생시킨다.
    let f = File::open("test.txt").unwrap();
    
    // 여기에 도달하면 파일 열기에 성공했음을 의미한다.
    println!("파일 열기 성공");
}
/*실행결과
파일 열기 성공 
*/
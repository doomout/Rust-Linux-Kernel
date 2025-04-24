use std::path::{Path, PathBuf};

fn main() {
    // Path : 참조용, 읽기만 가능
    let path = Path::new("/tmp/test.txt");

    // 경로의 파일명 추출
    if let Some(filename) = path.file_name() {
        println!("파일명: {:?}", filename);
    }

    // 경로의 확장자 추출
    if let Some(extension) = path.extension() {
        println!("확장자: {:?}", extension);
    }

    // PathBuf : 소유 및 수정 가능
    let mut path_buf = PathBuf::from("/tmp/foo");
    
    // 경로에 파일명 추가
    path_buf.push("example.txt");
    println!("전체 경로: {:?}", path_buf);  
}

/*실행 결과
파일명: "test.txt"
확장자: "txt"
전체 경로: "/tmp/foo\\example.txt" 
*/
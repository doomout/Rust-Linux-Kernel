extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // 현재 디렉터리를 컴파일러에 라이브러리 검색 경로를 알린다.
    println!("cargo:rustc-link-search=.");

    // hello 라이브러리를 링크한다.
    println!("cargo:rustc-link-lib=hello");

    // c_src/hello.h 파일이 변경되면 다시 빌드하도록 설정한다.
    println!("cargo:rerun-if-changed=c_src/hello.h");
    
    // 'bindgen'을 사용하여 C 헤더 파일에 대한 Rust 바인딩을 생성한다.
    // 'bindgen'은 C/C++ 헤더 파일을 Rust 코드로 변환하는 도구이다.
    let bindings = bindgen::Builder::default()
        .header("c_src/hello.h") //바인딩할 헤더 파일 설정
        .parse_callbacks(Box::new(bindgen::CargoCallbacks)) // Cargo 콜백 등록
        .generate() //바인딩 생성
        .expect("Unable to generate bindings");

    // 출력 경로를 환경 변수 OUT_DIR에서 가져온다.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // 생성된 바인딩을 Rust 파일로 저장한다.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
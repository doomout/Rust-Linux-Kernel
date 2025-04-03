fn main() {
    let s: &str = "  Hello Rust ";  
    println!("{}", s.trim());  // 앞뒤 공백제거
    println!("{}", s.to_lowercase()); //소문자로 변환
    println!("{}", s.to_uppercase()); //대문자로 변환
}

/**실행결과
 Hello Rust
  hello rust 
  HELLO RUST 
 */
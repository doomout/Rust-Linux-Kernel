// 객체가 메모리에서 벗어날 때 수행해야 할 작업 지정
struct Book {
    title: String,
}

impl Drop for Book {
    // Drop트레잇을 구현합니다.
    fn drop(&mut self) {
        println!("Book객체 해제: {}", self.title);
    }
}

fn main() {
    {
        let book = Book { title: String::from("러스트") };
    } // book의 Drop트레잇이 자동으로 호출됩니다.
}

/*실행결과
Book객체 해제: 러스트 
*/
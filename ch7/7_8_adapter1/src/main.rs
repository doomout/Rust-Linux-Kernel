//어댑터 패턴이란?
// 서로 호환되지 않는 인터페이스를 연결해주느 ㄴ패턴
// 기존 코드를 바꾸기 어렵지만 우리가 원하는 방식을 새 인터페이스로 감싸서 사용
struct Adaptee {}

impl Adaptee {
    fn new() -> Adaptee {
        Adaptee {}
    }

    fn vendor_specific_api(&self) {
        println!("벤더가 정의한 API") // 기존 시스템의 함수
    }
}

// 어댑터 패턴을 사용해 벤더가 정의한 API를 감싸는 구조체
// 어댑터는 벤더가 정의한 API를 호출하는 메서드를 제공
struct Adapter {}

impl Adapter {
    fn new() -> Adapter {
        Adapter {}
    }
    
    // 우리가 정의한 API
    fn call_api(&self) {
        Adaptee::new().vendor_specific_api(); // 내부적으로 기존 함수를 호출한다.
    }
}

fn main() {
    let adapter = Adapter::new();
    adapter.call_api(); // 어댑터를 통해 기존에 정의한 API를 호출
}
/*실행결과
벤더가 정의한 API */
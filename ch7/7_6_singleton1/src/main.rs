/* Cargo.toml 에 의존성 추가
[dependencies]
lazy_static = "1.4.0" */

// 싱글턴 패턴 : 시스템에서 하나의 인스턴스만 생성되도록 하는 디자인 패턴
// lazy_static : 정적 변수를 안전하게 사용할 수 있도록 도와주는 라이브러리
// lazy_static! 매크로를 사용하여 정적 변수를 선언하고 초기화할 수 있다.
#[macro_use]
extern crate lazy_static;

// 싱글턴 패턴을 구현하기 위해서는 정적 변수를 사용해야 한다.
struct MySingleton {
    name: String
}

// MySingleton 구조체는 name이라는 필드를 가지고 있다.
// 이 필드는 MySingleton 구조체의 인스턴스가 생성될 때 초기화된다.
// MySingleton 구조체는 new() 메서드를 가지고 있다.
impl MySingleton {
    fn new(name: String) -> MySingleton {
        MySingleton {
            name: name
        }
    }

    // methods of the singleton struct
    fn call(&self) {
        println!("my name is {}", self.name);
    }
}

// 정적 변수를 선언하고 초기화하는 부분
// lazy_static! 매크로를 사용하여 정적 변수를 선언하고 초기화할 수 있다.
// 이 매크로는 정적 변수를 안전하게 사용할 수 있도록 도와준다.
// INSTANCE는 MySingleton 구조체의 인스턴스를 가리키는 정적 변수이다.
// 이 변수는 프로그램이 시작될 때 초기화되며, 이후에는 프로그램 전체에서 공유된다.
// INSTANCE는 MySingleton 구조체의 인스턴스를 가리키는 정적 변수이다.
lazy_static! {
    static ref INSTANCE: MySingleton = {
        MySingleton::new(String::from("luna")) //인스턴스 생성시  "luna"라는 이름을 전달한다.
    };
}

fn main() {
    INSTANCE.call();
}

/*실행결과
my name is luna */
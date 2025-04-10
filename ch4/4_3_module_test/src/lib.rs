//pub 는 public 이라는 뜻
// 표현 계층
pub mod presentation {
    pub mod view {
        pub fn render() {
            println!("mysystem::presentation::view::render");
        }
    }
}

//비즈니스 계층
pub mod business {
    pub mod user {
        pub fn create() {
            println!("mysystem::business::user::create");
        }
    }
}

//데이터베이스 계층
pub mod database {
    pub mod user_dao {
        pub fn create() {
            println!("mysystem::database::user_dao::create");
        }
    }
}

#[test]
fn it_wors() {
    presentation::view::render();
    business::user::create();
    database::user_dao::create();
}

/*테스트 결과
running 1 test
test it_wors ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests module_test

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
*/
macro_rules! create_function { //함수를 생성하는 매크로
    ($func_name:ident) => { //함수 이름을 입력받아 $func_name에 저장
        fn $func_name() { //func_name이라는 이름의 함수를 생성
            println!("함수: {:?}()", stringify!($func_name));
        }
    };
}

// ident_func이라는 함수를 컴파일 시점에 생성
create_function!(ident_func);

fn main() {
    ident_func();
}

/*실행결과
함수: "ident_func"() */
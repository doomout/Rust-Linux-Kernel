// Cargo.toml 에 아래와 같은 의존성 추가
//[dependencies]
//adder = { path = "../4_1_adder" }

extern crate adder;

fn main() {
    let ret = adder::add(1, 2);
    println!("1+2={}", ret);
}

/*실행결과
1+2=3 
*/
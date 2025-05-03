// 두 수를 더하는 매크로 정의
macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    };
}

// 두 수를 곱하는 매크로 정의
macro_rules! multiply {
    ($x:expr, $y:expr) => {
        $x * $y
    };
}

// 네 개의 값을 받아서
// (a + b) * (c + d) 형태로 계산하는 매크로 정의
macro_rules! compute {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        multiply!(add!($a, $b), add!($c, $d))
        // => ($a + $b) * ($c + $d)
    };
}

fn main() {
    // compute!(1, 2, 1, 2)는 (1 + 2) * (1 + 2)와 같음
    // add!(1, 2) => 3
    // multiply!(3, 3) => 9
    let result = compute!(1, 2, 1, 2); 

    // 최종 결과 출력
    println!("(1+2)x(1+2)={}", result); // 출력: (1+2)x(1+2)=9
}

/*실행결과
(1+2)x(1+2)=9 */
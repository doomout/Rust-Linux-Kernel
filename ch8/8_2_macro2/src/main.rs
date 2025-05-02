// 두 개의 수를 더하느 ㄴ매크로 예제
// macro_rules! 은 매크로를 정의하는 키워드입니다.
macro_rules! add {
    // 패턴은 $a:expr, $b:expr로 정의되어 있습니다.
    // $a와 $b는 매크로가 호출될 때 전달되는 인자입니다.
    // :expr은 expr 타입의 인자를 받는다는 의미입니다.
    // expr은 표현식(expression)을 의미합니다.
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

fn main() {
    // add! 매크로를 호출하여 1과 2를 더합니다.
    // add! 매크로는 $a와 $b에 1과 2를 전달합니다.
    // 매크로는 $a + $b를 계산하여 결과를 반환합니다.
    let sum = add!(1, 2);
    println!("1+2={}", sum);
}

/*실행결과
1+2=3 */
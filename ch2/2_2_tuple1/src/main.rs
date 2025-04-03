fn main() {
    // i32, char, bool 타입의 튜플 x, y, z를 선언하고 각각의 값으로 1, 'a', true를 대입한다.
    // 튜플의 각 요소는 서로 다른 타입을 가질 수 있다.
    let (x, y, z) : (i32, char, bool) = (1, 'a', true);
    println!("x={}, y={}, z={}", x, y, z);
}

/*실행결과
 x=1, y=a, z=true
 */
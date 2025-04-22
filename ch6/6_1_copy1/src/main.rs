// Copy 예제 
// Copy 는  묵시적 복사를 수행하며 바이트 단위의 복제인 깊은 복사를 수행한다.
// 기본적인 데이터 타입에 대해 별도의 소유권 전달 없이 값을 복사한다.
// Copy 트레잇을 구현한 타입은 스택에 저장된 값이 복사되며, 힙에 저장된 값은 복사되지 않는다.
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn add_points(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };

    // add_point의 인자로 들어가는 a, b는 copy트레잇에 의해 복제됩니다.
    let result = add_points(a, b);

    println!("{:?}", a); // a에 접근 가능
    println!("{:?}", b); // b에 접근 가능
    println!("{:?}", result);
}
/*실행결과
Point { x: 1, y: 2 }
Point { x: 3, y: 4 }
Point { x: 4, y: 6 }
*/
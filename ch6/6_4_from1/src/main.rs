// 러스트는 타입에 관한 엄격한 안정성을 보장한다.
// 그래서 타입을 변환할 때는 명시적으로 변환을 해줘야 한다.
// From 은 한 자료형을 다른 자료형으로 변환하는 로직을 정의할 때 사용
// Into는 From을 통해 변환된 자료형을 사용.
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Point 구조체를 (i32, i32) 튜플로 변환하는 From trait 구현
impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

fn main() {
    let tuple = (1, 2);

    // 주어진 tuple을 바탕으로 Point객체를 생성
    let pt: Point = Point::from(tuple);
    
    println!("Point::from = {:?}", pt);
    
    // tuple을 기반으로 point를 생성합니다. 이때 Point::from이 호출됩니다.
    let pt: Point = tuple.into();

    println!("tuple.into = {:?}", pt);
}

/*실행결과
Point::from = Point { x: 1, y: 2 }
tuple.into = Point { x: 1, y: 2 } 
*/
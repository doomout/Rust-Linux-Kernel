// Pointable이라는 트레잇(인터페이스)을 정의합니다.
// 이 트레잇을 구현하는 구조체는 x()와 y() 메서드를 가져야 합니다.
trait Pointable {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

// Point 구조체는 x, y 좌표를 가지고 있습니다.
struct Point {
    x: i32,
    y: i32,
}

// Point 구조체에 대해 Pointable 트레잇을 구현합니다.
impl Pointable for Point {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

// ColorPoint 구조체는 색상(color)과 Point를 포함하는 구조체입니다.
struct ColorPoint {
    color: String,
    point: Point,
}

// ColorPoint에 메서드를 추가합니다.
impl ColorPoint {    
    // 생성자 메서드: 새로운 ColorPoint 인스턴스를 만듭니다.
    fn new(color: String, x: i32, y: i32) -> ColorPoint {
        ColorPoint {
            color: color,
            point: Point { x: x, y: y },
        }
    }

    // 색상을 반환하는 메서드
    fn color(&self) -> &String {
        &self.color
    }
}

// ColorPoint도 Pointable 트레잇을 구현합니다.
// 내부에 있는 Point의 x, y 값을 위임(delegate)하는 방식입니다.
impl Pointable for ColorPoint {
    fn x(&self) -> i32 {
        self.point.x
    }

    fn y(&self) -> i32 {
        self.point.y
    }
}

// Pointable 트레잇을 참조하는 함수. 모든 Pointable 타입을 받아서 좌표를 출력합니다.
fn print_pointable(pointable: &dyn Pointable) {
    println!("x: {} y: {}", pointable.x(), pointable.y());
}

fn main() {
    // ColorPoint 인스턴스를 생성합니다.
    let pt = ColorPoint::new(String::from("red"), 1, 2);
    
    // print_pointable 함수에 전달하면 Pointable로 동작합니다. (다형성)
    print_pointable(&pt);
}

/*실행결과
x: 1 y: 2 */
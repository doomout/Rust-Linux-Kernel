// 데이터 직렬화 예제
/* serde, serde_json 를 사용하려면 의존성 추가 해야 한다.
[dependencies]
serde = {version = "1.0", features = ["derive"]}
serde_json = "1.0" 
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let pt = Point { x: 10, y: 20 };
    //pt 를 json 형식으로 변환한다.
    let json = serde_json::to_string(&pt).unwrap();
    println!("json: {}", json);

    // json을 사용해서 Point를 생성한다.
    let pt: Point = serde_json::from_str(&json).unwrap();
    println!("point: [{}, {}]", pt.x, pt.y);
}

/*실행결과
json: {"x":10,"y":20}
point: [10, 20] 
*/
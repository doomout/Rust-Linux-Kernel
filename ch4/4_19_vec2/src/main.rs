//벡터 개별 요소 읽는 예제
fn main() {
    let v = vec![1, 2, 3]; //i32 타입의 벡터 v를 생성하고 1,2,3으로 설정
    let one = v[0]; //v의 첫 번째 요소를 one에 할당 

    // get() 함수는 해당 인덱스에 값이 있으면 Some(값)을 반환하고, 없으면 None을 반환한다.
    let two = v.get(1); //v의 두 번째 요소를 two에 할당, Option 타입을 반환하므로 결과는 some(2) 다.

    println!("One: {:?}, Two: {:?}", one, two);
}

/* 실행결과
One: 1, Two: Some(2)
*/
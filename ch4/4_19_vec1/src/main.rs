// Vec 란? 크기 조정이 가능한 배열
fn main() {
    // 빈 i32형 벡터를 생성
    //let mut v: Vec<i32> = Vec::new(); // 이 방법도 있고
    let mut v: Vec<i32> = vec![]; // 이 방법도 있다.

    for i in 1..10 {
        v.push(i); // i 값을 벡터 v에 삽입
    }

    for d in v { // 벡터 v의 각 요소에 대해 반복
        print!("{},", d);
    }
}

/* 실행결과
1,2,3,4,5,6,7,8,9, 
*/
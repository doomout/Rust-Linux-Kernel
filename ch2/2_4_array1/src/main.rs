fn main() {
    // 배열은 하나의 자료형만 선언 가능하며 크기가 고정이다.
    let arr = [1, 2, 3, 4, 5];

    for a in arr {
        print!("{},", a);
    }
    // println! 은 다음 라인으로 이동
    println!("");
}

/* 실행결과
1,2,3,4,5,
 */
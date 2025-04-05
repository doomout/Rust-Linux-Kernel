// 데이터(필드)만 정의
struct Score {
    score: i32,
}

// Score에 관련된 동작(메서드) 정의
impl Score {
    // 점수를 등급으로 변환하는 메서드
    fn get_grade(&self) -> String {
        if self.score >= 90 {
            String::from("A")
        } else if self.score >= 80 {
            String::from("B")
        } else {
            String::from("C")
        }
    }
    // 정적 메서드 (Rust의 from() 느낌)
    fn from(score: i32) -> Score {
        Score { score: score }
    }
}


fn main() {

}

#[test]
fn test_get_grade() {
    assert_eq!(Score::from(100).get_grade(), "A");
    assert_eq!(Score::from(80).get_grade(), "B");
}

/* 테스트 결과
running 1 test
test test_get_grade ... ok
*/
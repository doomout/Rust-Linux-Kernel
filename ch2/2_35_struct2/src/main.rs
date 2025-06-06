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
}


fn main() {

}

#[test]
fn test_get_grade() {
    // Score 구조체의 인스턴스를 생성
    let score = Score { score: 100 };

    // score.get_grade()가 "A"를 반환하는지 검사
    assert_eq!(score.get_grade(), "A");
    
    // Score 구조체의 인스턴스를 생성
    let score = Score { score: 80 };
    
    // score.get_grade()가 "B"를 반환하는지 검사
    assert_eq!(score.get_grade(), "B");
}

/* 테스트 결과
running 1 test
test test_get_grade ... ok
*/
// 햄버거를 표현하는 구조체
struct Burger {
    bun: String,           // 빵 종류
    patties: i32,          // 패티 개수
    sauce: String,         // 소스 종류
    extra: Vec<String>,    // 추가 재료들 (예: 양상추, 토마토 등)
}

impl Burger {
    // Burger 객체의 정보를 문자열로 변환하여 반환하는 메서드
    fn to_string(&self) -> String {
        let mut txt = format!("{} 위에 순 쇠고기 패티 {}장 {} 소스 ", 
            self.bun, self.patties, self.sauce);

        // 추가 재료들을 문자열에 덧붙임
        for ex in self.extra.iter() {
            txt = format!("{} {} ", txt, ex);
        }

        txt
    }
}

// Burger 객체를 만들기 위한 빌더 구조체
struct BurgerBuilder {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl BurgerBuilder {
    // 기본값으로 초기화된 빌더 생성자
    fn new() -> BurgerBuilder {
        BurgerBuilder {
            bun: String::from(""),
            patties: 0,
            sauce: String::from(""),
            extra: Vec::<String>::new()
        }
    }

    // bun 설정 메서드 (self를 소유권으로 받고 반환하여 메서드 체이닝 가능)
    fn bun(mut self, bun: String) -> BurgerBuilder {
        self.bun = bun;
        self
    }

    // patties 설정 메서드
    fn patties(mut self, patties: i32) -> BurgerBuilder {
        self.patties = patties;
        self
    }

    // sauce 설정 메서드
    fn sauce(mut self, sauce: String) -> BurgerBuilder {
        self.sauce = sauce;
        self
    }

    // extra 재료 추가 메서드
    fn add_extra(mut self, val: String) -> BurgerBuilder {
        self.extra.push(val);
        self
    }

    // 모든 값이 설정된 Burger 객체를 최종적으로 생성
    fn build(self) -> Burger {
        Burger {
            bun: self.bun,
            patties: self.patties,
            sauce: self.sauce,
            extra: self.extra,
        }
    }
}

fn main() {
    // 빌더 패턴을 사용해 햄버거 객체 생성 (메서드 체이닝 사용)
    let burger = BurgerBuilder::new()
        .bun(String::from("참깨빵"))
        .patties(2)
        .sauce(String::from("특별한"))
        .add_extra(String::from("양상추"))
        .build(); // 최종 Burger 객체 생성

    // 햄버거 정보 출력
    println!("{}", burger.to_string());
}

/*실행결과
참깨빵 위에 순 쇠고기 패티 2장 특별한 소스  양상추  
*/
use std::collections::HashMap;
use crate::database::file_db;

// 외부에서 접근 가능한 구조체
pub struct User {
    pub id: i32,
    pub age: i32,
    pub name: String,
}

//User 구조체를 관리하기 위한 구조체 함수
pub struct UserManager {
    user_map: HashMap<i32, User> //사용자 id와 사용자 정보를 매칭하는 HashMap
}

// UserManager 구조체의 메서드
impl UserManager {
    // UserManager 구조체의 생성자
    pub fn new() -> UserManager {
        let mgr = UserManager {
            user_map:  HashMap::new() // 빈 HashMap으로 초기화
        };
        mgr
    }

    // 사용자 추가 메서드
    pub fn add_user(&mut self, id: i32, age: i32, name: String) -> bool {
        let mut user = User {
            id: id,
            age: age,
            name: name,
        };

        // id가 존재하면 갱신, 존재하지 않으면 추가
        self.user_map.entry(user.id).or_insert(user);
        true // id가 존재하면 true 리턴
    }
    
    // 사용자 삭제 메서드
    // id가 존재하지 않으면 false 리턴
    
    pub fn remove_user(&mut self, id: i32) -> bool {
        // id가 존재하지 않으면 false 리턴
        if self.user_map.contains_key(&id) == false {
            return false;
        }

        self.user_map.remove(&id);
        true // id가 존재하면 true 리턴
    }
    
    // 특정 사용자 정보 조회 메서드
    pub fn get_user(&mut self, id: i32) -> Option<&User> {
        self.user_map.get(&id) // id로 사용자 정보 반환
    }
    
    // 모든 사용자 정보를 반환하는 메서드
    pub fn get_all(&mut self) -> Vec<&User> {
        let mut v : Vec<&User> = Vec::new();

        //모든 사용자를 순회하며 벡터에 추가
        for u in self.user_map.values() {
            v.push(&u);
        }

        // 사용자 참조를 담은 벡터 반환
        return v;
    }

    // 사용자 정보를 파일에 저장하는 메서드
    // 파일 이름은 "file.db"로 고정
    pub fn save(&mut self) {
        file_db::save(String::from("file.db"), self.get_all());
    }

    // 사용자 정보를 파일에서 불러오는 메서드
    // 파일 이름은 "file.db"로 고정
    pub fn load(&mut self) {
        let user_vec = file_db::load(String::from("file.db"));
        // user_map 초기화
        self.user_map = HashMap::new();

        // 사용자 정보를 순회하며 user_map에 추가
        for user in user_vec.iter() {
            self.add_user(user.id, user.age, user.name.clone());
        }
    }
}
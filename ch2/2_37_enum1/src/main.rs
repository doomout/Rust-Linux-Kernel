struct ElementarySchool {
    room: String,
}

struct MiddleSchool {
    teacher: String,
}

struct HighSchool {
    id: i32,
}

enum SchoolKind {
    Elementary(ElementarySchool), //Elementary 에는 ElementarySchool 구조체가 들어감
    Middle(MiddleSchool), // Middle 에는 MiddleSchool 구조체가 들어감
    High(HighSchool) // High 에는 HighSchool 구조체가 들어감
}


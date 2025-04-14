// into_iter() 메서드는 소유권을 이동하는 반복자를 생성한다.
// 이로 인해 원본 데이터는 더 이상 사용 불가능하다.
// into_iter() 메서드는 소유권을 이동하는 반복자를 생성한다.
// 이로 인해 원본 데이터는 더 이상 사용 불가능하다.
n main() {
    let vec = vec![1, 2, 3];
    for item in vec.into_iter() {
        // vec의 소유권은 이동되었으므로 이후에 vec를 사용할 수 없음
        println!("{}", item);
    }

    println!("{:?}", vec); // vec 접근 불가능
}

/*실행결과
error: expected one of `!` or `::`, found `main`
 --> main.rs:5:3
  |
5 | n main() {
  |   ^^^^ expected one of `!` or `::`
  |
help: there is a keyword `fn` with a similar name
  |
5 | fn main() {
  | ~~

error: aborting due to 1 previous error
 */
/* 의존성 추가
[dependencies]
dbus = "*"
*/

// DBus 클라이언트 예제
// DBus 서버에서 제공하는 Hello 메서드를 호출합니다.
use dbus::blocking::Connection;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 새로운 DBus 세션 연결을 생성합니다.
    let conn = Connection::new_session()?;
    
    // DBus 프록시를 생성합니다. 
    // com.example.dbustest 서비스의 /hello 경로에 접근합니다.
    let proxy = conn.with_proxy("com.example.dbustest", "/hello", Duration::from_millis(5000));

    // Hello를 호출합니다.
    let (hello,): (String,) = proxy.method_call("com.example.dbustest", "Hello", ("luna",))?;
    
    println!("수신: {}", hello);

    Ok(())
}

/*실행 결과 : 서버 실행 후 실행 할 때마다 횟수가 오른다.
수신: 안녕 luna! API호출 횟수: 1
수신: 안녕 luna! API호출 횟수: 2
수신: 안녕 luna! API호출 횟수: 3
*/
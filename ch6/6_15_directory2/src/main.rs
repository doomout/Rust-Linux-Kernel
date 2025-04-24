// read_dir() 메서드는 디렉터리의 모든 내용을 읽어오는 메서드
use std::fs;

fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }
    Ok(())
}
/*실행결과
".\\main.exe"
".\\main.pdb"
".\\main.rs" 
*/
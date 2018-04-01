fn main() {
    
    let s = String::from("hello");
    takes_ownership(s); // s에 대한 소유권이 takes_ownership 함수로 넘어갔으므로 다음줄에서 s를 사용하면 에러
    
    let x = 4;
    makes_copy(x); // x에 대한 소유권이 makes_copy에 넘어갔지만 i32 타입의 경우에는 stack에 쌓이고, copy된다.

}

fn takes_ownership(some_string:String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer:i32) {
    println!("{}", some_integer);
}

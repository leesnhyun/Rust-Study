fn main() {
    
    let mut s = String::from("Hello");
    change(&mut s);

    // mutable reference (&mut) 는 스코프에서 1개만 가질 수 있음

    {
        // mutable reference 를 쓰려면 이렇게 스코프를 한번 더 만들자. (아니면 immutable
        // reference만 쓰던지... )
        change(&mut s);
    }

}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

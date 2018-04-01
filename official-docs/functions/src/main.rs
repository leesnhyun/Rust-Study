fn cel_to_fah( cel:f32 ) -> f32 {
    (cel * 1.8) + 32.0
}

fn fah_to_cel( fah:f32 ) -> f32 {
    ( fah - 32.0 ) / 1.8
}

fn main() {
    println!("섭씨 {}도는 화씨 {}도", 32.0, cel_to_fah(32.0));
    println!("화씨 {}도는 섭씨 {}도", 89.6, fah_to_cel(89.6));
}

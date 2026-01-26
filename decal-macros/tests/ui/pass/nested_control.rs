use decal::decal;

fn flag() -> bool {
    true
}

fn main() {
    let _ = decal! {
        Block {
            if flag() {
                for i in 0..3 {
                    while flag() {
                        loop {
                            match i {
                                0 => Text("zero"),
                                _ => Text("some"),
                            }
                            break;
                        }
                        break;
                    }
                }
            }
        }
    };
}

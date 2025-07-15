use decal::decal;

fn flag() -> bool {
    true
}

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
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

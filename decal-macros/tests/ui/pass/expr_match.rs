use decal::decal;

fn val() -> i32 {
    1
}

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            match val() {
                0 => Text("zero"),
                1 | 2 if val() > 1 => Column() { Text("yes") },
                _ if false => Row() { Text("guard only") },
                _ => Text("default"),
            }
        }
    };
}

use decal::decal;

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            match 1 {
                0 => Text("zero")
                1 => Text("one"),
            }
        }
    };
}

use decal::decal;

fn main() {
    let _ = decal! {
        Block {
            match 1 {
                0 => Text("zero")
                1 => Text("one"),
            }
        }
    };
}

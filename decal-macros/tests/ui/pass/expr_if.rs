use decal::decal;

fn condition() -> bool {
    true
}

fn main() {
    let _ = decal! {
        Block {
            if condition() {
                Column {}
            } else if !condition() {
                Row {}
            } else {
                Snippet {}
            }
        }
    };
}

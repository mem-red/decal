use decal::decal;

fn condition() -> bool {
    true
}

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
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

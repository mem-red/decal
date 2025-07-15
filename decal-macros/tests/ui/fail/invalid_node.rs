use decal::decal;

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            Spaghetti {
                Text("invalid")
            }
        }
    };
}

use decal::decal;

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            Root(32.0, 32.0) {
                Text("nested")
            }
        }
    };
}

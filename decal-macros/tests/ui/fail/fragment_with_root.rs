use decal::fragment;

fn main() {
    let _ = fragment! {
        Root(32.0, 32.0) {
            Text("invalid")
        }
    };
}

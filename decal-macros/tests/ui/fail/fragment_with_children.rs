use decal::fragment;

fn main() {
    let frag = fragment! {
        Column {}
    };

    let _ = fragment! {
        Fragment(frag) {
            Text("child")
        }
    };
}

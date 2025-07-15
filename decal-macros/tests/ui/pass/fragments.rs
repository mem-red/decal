use decal::{decal, fragment};

fn main() {
    let frag = fragment! {
        Column() {
            Text("content")
        }
    };

    let _ = decal! {
        Root(64.0, 64.0) {
            Column {
                Fragment(frag)
            }
        }
    };
}

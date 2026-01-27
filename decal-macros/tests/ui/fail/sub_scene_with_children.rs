use decal::decal;

fn main() {
    let frag = decal! {
        Column {}
    };

    let _ = decal! {
        Block {
            Scene(frag) {
                Text("child")
            }
        }
    };
}

use decal::decal;

fn main() {
    let scene = decal! {
        Column {
            Text("content")
        }
    };

    let _ = decal! {
        Column {
            Scene(scene)
        }
    };
}

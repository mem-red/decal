use decal::decal;

fn main() {
    let frag = decal! {
        Column {
            Text("content")
        }
    };

    let _ = decal! {
        Column {
            Fragment(frag)
        }
    };
}

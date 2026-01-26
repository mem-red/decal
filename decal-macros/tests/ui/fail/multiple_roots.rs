use decal::decal;

fn main() {
    let _ = decal! {
        Row {
            Text("one")
        }

        Column {
            Text("two")
        }
    };
}

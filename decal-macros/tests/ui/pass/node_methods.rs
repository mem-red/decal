use decal::decal;

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            Column {
                Text("method chain")
            }.background(None)
        }
    };
}

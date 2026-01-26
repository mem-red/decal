use decal::decal;

fn main() {
    let _ = decal! {
        Column {
            Text("method chain")
        }
          .background(None)
          .padding(32)
    };
}

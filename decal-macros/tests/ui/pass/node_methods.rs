use decal::decal;

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            Column {
                Text("method chain")
            }
        }.set_background(None)
    };
}

// TODO: Allow this kind of parsing:
// Column().spacing(10).padding(5) {
//     Text("method chain")
// }

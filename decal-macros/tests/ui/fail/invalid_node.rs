use decal::decal;

fn main() {
    let _ = decal! {
        Block {
            Spaghetti {
                Text("invalid")
            }
        }
    };
}

use decal::decal;

fn main() {
    let _ = decal! {
        Block {
            Snippet {
                let s = "DECAL";
            }

            if s == "DECAL" {
                Row {}
            }
        }
    };
}

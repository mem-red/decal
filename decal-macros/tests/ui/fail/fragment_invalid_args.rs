use decal::decal;

fn main() {
    let _ = decal! {
        Fragment()
    };

    let _ = decal! {
        Fragment(1, 2)
    };
}

use decal::decal;

fn main() {
    let _ = decal! {
        Scene()
    };

    let _ = decal! {
        Scene(1, 2)
    };
}

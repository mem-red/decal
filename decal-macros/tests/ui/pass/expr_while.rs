use decal::decal;

fn cond() -> bool {
    false
}

fn main() {
    let _ = decal! {
        Block {
            while cond() {
                Text("looping")
            }

            'test_loop: while cond() {
                Text("loop with label")
                break 'test_loop;
            }
        }
    };
}

use decal::decal;

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            Snippet { let mut i = 0; }

            loop {
                Text("looping")

                Snippet {
                    i += 1;

                    if i >= 5 {
                        break;
                    }
                }
            }

            'test_loop: loop {
                Text("loop with label")
                break 'test_loop;
            }
        }
    };
}

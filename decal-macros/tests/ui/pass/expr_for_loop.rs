use decal::decal;

fn vec_3() -> Vec<i32> {
    vec![1, 2, 3]
}

fn main() {
    let _ = decal! {
        Root(64.0, 64.0) {
            for x in vec_3() {
                Text(format!("loop with x: {x}"))
            }

            'outer: for (a, b) in [(1, 2)] {
                Text("outer loop")

                'inner: for (c, d) in [(3, 4)] {
                    Text("inner loop")

                    if a == b + c {
                        break 'outer;
                    } else if c == d + a {
                        break 'inner;
                    }
                }
            }
        }
    };
}

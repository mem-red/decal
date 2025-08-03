use decal::prelude::*;
use decal_macros::{decal, fragment};
use taffy::{Size, prelude::TaffyMaxContent};

fn row_with_text(vertical_pos: &str) -> Node {
    fragment! {
        Row {
            Text(format!("{vertical_pos}_left"))
            Text(format!("{vertical_pos}_right"))
        }
    }
}

fn main() {
    let mut decal_1 = decal! {
            Root(1200, 630) {
                Column {
                    Fragment(row_with_text("top"))
                    Fragment(row_with_text("bottom"))
                    Row {
                        Column {
                            Row {
                                Text("abcdb")
                            }.reverse(false)
                        }.reverse(true)
                    }.reverse(false)
                }
            }
    };

    decal_1.compute_layout(Size::MAX_CONTENT);
}

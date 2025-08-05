use decal::prelude::*;
use decal_macros::{decal, fragment};
use taffy::{Size, prelude::TaffyMaxContent};

fn another() -> Decal {
    fragment! {
        Row {
            Text("another?")
        }
    }
}

fn row_with_text(vertical_pos: &str) -> Decal {
    fragment! {
        Row {
            Text(format!("{vertical_pos}_left"))
            Fragment(another())
            Text(format!("{vertical_pos}_right"))
        }.reverse(true)
    }
}

fn main() {
    let mut dcl = decal! {
            Root(1200, 630) {
                Column {
                    Fragment(row_with_text("top"))
                    Row {
                        Column {
                                Text("abcdb")
                        }
                    }.reverse(false)
                }
            }
    };

    dcl.compute_layout(Size::MAX_CONTENT, true);
    dcl.print_tree();
}

use decal::prelude::*;
use decal_macros::{decal, fragment};

fn another() -> Decal {
    fragment! {
        Row {
            Text("another?")
            Block {
                Image("https://t.co/img.jpeg")
            }
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
                .padding(*Padding::new().top(Length::Percent(0.65)))
                .padding(None)
            }
    };

    dcl.compute_layout(true);
    dcl.print_tree();
}

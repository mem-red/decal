use std::sync::Arc;

use decal::prelude::*;
use decal_macros::{decal, fragment};
use resvg::render;
use tiny_skia::Pixmap;
use usvg::{Options, Transform, Tree, fontdb::Database};

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
            // Fragment(another())
            Text(format!("{vertical_pos}_right"))
        }.reverse(true)
    }
}

fn main() {
    let mut dcl = decal! {
            Root(1200, 630) {
                Column {
                    Text("hello world!")
                    Text("second line 🥹")
                }.padding([pix(0)])
                // Column {
                //     // Fragment(row_with_text("123000000"))
                //     // Row {
                //     //     Column {
                //     //             Text("abcdb")
                //     //     }
                //     // }.reverse(false)
                // }
                // .padding([pix(100)])
                // .background(Fill::Color(Color {
                //     r: 25,
                //     g: 30,
                //     b: 105,
                //     a: 100
                // }))
            }
    };

    dcl.compute_layout(true);
    dcl.print_tree();
    let svg = dcl.to_svg();
    println!("{}\n\n\n", svg);

    let tree = Tree::from_str(
        &svg,
        &Options {
            fontdb: Arc::new(Database::new()),
            ..Default::default()
        },
    )
    .unwrap();
    let mut pixmap = Pixmap::new(1200, 630).unwrap();
    render(&tree, Transform::default(), &mut pixmap.as_mut());
    pixmap.save_png("./output.png").unwrap();
}

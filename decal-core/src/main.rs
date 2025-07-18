use decal::prelude::*;
use taffy::prelude::*;
use decal_macros::{decal, fragment};

fn row_with_text(vertical_pos: &str) -> DecalFragment {
    fragment! {
        Row {
            Text(format!("{vertical_pos}_left"))
            Text(format!("{vertical_pos}_right"))
        }
    }
}

// fn main() {
//     let _item = decal! {
//             Root(1200.0, 630.0) {
//                 Column {
//                     Fragment(row_with_text("top"))
//                     Fragment(row_with_text("bottom"))
//                 }
//                 .spacing(12.0)
//                 .width(Dimension::AUTO)
//             }
//     };
// }

fn main() -> Result<(), taffy::TaffyError> {
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    let child = taffy.new_leaf(Style {
        size: Size { width: Dimension::from_percent(0.5), height: Dimension::AUTO },
        ..Default::default()
    })?;
    
    let node = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::from_length(100.0), height: Dimension::from_length(100.0) },
            justify_content: Some(JustifyContent::Center), 
            ..Default::default()
        },
        &[child],
    )?;

    println!("Compute layout with 100x100 viewport:");
    taffy.compute_layout(
        node,
        Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
    )?;
    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    println!("Compute layout with undefined (infinite) viewport:");
    taffy.compute_layout(node, Size::MAX_CONTENT)?;
    println!("node: {:#?}", taffy.layout(node)?);
    println!("child: {:#?}", taffy.layout(child)?);

    Ok(())
}
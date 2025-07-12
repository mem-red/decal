use atree::{Arena, Token};
use decal_macros::{decal, decal_fragment};

fn print_tree<T: std::fmt::Debug>(arena: &Arena<T>, node: Token, depth: usize) {
    let data = &arena[node].data;
    println!("{}{:?}", "  ".repeat(depth), data);
    for child in node.children(arena) {
        print_tree(arena, child.token(), depth + 1);
    }
}

fn main() {
    // TODO: Check fragment nesting (cyclic loops)

    // let fragment = {
    //     decal_fragment! {
    //         Row {
    //             Column {
    //                 Row {}
    //             }
    //         }
    //     }
    // };

    // let item = {
    //     decal! {
    //         Root(1200.0, 630.0) {
    //             Column {
    //                  'loop1: for  _ in 0..4 {
    //                     Row {
    //                         for zed in 0..5 {
    //                             for b in 1..5 {
    //                                 Row {}
    //                             }
    //                             Column {
    //                                 Row {
    //                                     Fragment(fragment)

    //                                     if 0 == zed || 3 == 4 || 5 != 5 {
    //                                         for c in 0..1 {
    //                                             Column {
    //                                                 Row {}
    //                                             }
    //                                         }
    //                                     } else if 2 == 10 {
    //                                         Snippet {
    //                                             break 'loop1;
    //                                         }
    //                                     } else {
    //                                         Column {}
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //             .set_background(Some(Fill::Color))
    //             .set_background(None)
    //     }
    // };

    println!("__PRINT__");

    let (fragment, frag_root) = {
        {
            use decal::prelude::*;
            let mut row_node_1 = Row::new();
            let (mut fragment, row_tkn_1) = DecalFragment::new(NodeKind::Row(row_node_1));
            let mut column_node_1 = Column::new();
            let column_tkn_1 = row_tkn_1.append(
                fragment.arena_mut(),
                Node::new(NodeKind::Column(column_node_1)),
            );
            (fragment, row_tkn_1)
        }
    };
    let (item, root) = {
        use decal::prelude::*;
        let mut root_node_1 = Root::new(1200.0, 630.0);
        root_node_1.set_background(Some(Fill::Color));
        // root_node_1.set_background(None);
        let (mut decal, root_tkn_1) = Decal::new(root_node_1);
        let mut column_node_1 = Column::new();
        let column_tkn_1 = root_tkn_1.append(
            decal.arena_mut(),
            Node::new(NodeKind::Column(column_node_1)),
        );

        {
            let mut fragment_node_1 = Fragment::new(fragment.clone());
            decal.arena.copy_and_append_subtree(
                column_tkn_1,
                fragment_node_1.get_arena(),
                frag_root,
            );
        }

        (decal, root_tkn_1)
    };

    print_tree(&item.arena, root, 0);
}

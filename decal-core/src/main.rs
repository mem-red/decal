use decal_macros::{decal, decal_partial};

fn main() {
    // let partial = {
    //     decal_partial! {
    //         Row {
    //             Column {
    //                 Row {
    //                     Column {}
    //                 }
    //             }
    //         }
    //     }
    // };

    let item = {
        decal! {
            Root(1200.0, 630.0) {
                Column {
                     'loop1: for  _ in 0..4 {
                        Row {
                     for _ in 0..5 {
                                Column {
                                    Row {}
                                }
                            }
                        }
                    }
                }
            }
                .set_background(Some(Fill::Color))
                .set_background(None)
        }
    };

    println!("__PRINT__")
}

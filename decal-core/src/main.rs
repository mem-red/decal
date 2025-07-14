use decal_macros::{decal, decal_fragment};

fn main() {
    let fragment = decal_fragment! {
            Row {
                Column {
                    Row {}
                }
            }.set_spacing(None)
    };

    let cond = "";
    let item = decal! {
            Root(1200.0, 630.0) {
                Column {
                     'loop1: for  _ in 0..4 {
                        Row {
                            for zed in 0..5 {
                                for b in 1..5 {
                                    Row {}
                                }
                                Column {
                                    Row {
                                        Fragment(fragment)

                                        if 0 == zed || 3 == 4 || 5 != 5 {
                                            for c in 0..1 {
                                                Column {
                                                    Row {}
                                                }
                                            }
                                        } else if 2 == 10 {
                                            Snippet {
                                                break 'loop1;
                                            }
                                        } else {
                                            Column {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                match cond {
                    "col" => Column {
                        Row {
                            Column {
                                Snippet {
                                    let my_const: usize = 45;
                                }

                                Row {
                                    Snippet {
                                        let another: usize = 10;
                                        println!("{}", my_const);
                                    }
                                }
                            }
                        }
                    },
                    x if x == "cond" => Snippet {
                      let a = 5;
                    },
                    y =>  for e in 0..10 {
                        Row() {}
                    },
                    "c" => Snippet { if b == c {

                        } let b = c; },
                    "d" => Snippet {},
                    "e" => Snippet { let c = 5; },
                    "f" => Fragment(fragment),
                    _ => Row {},
                }

                'myloop: loop {
                    Row {}

                    if cond == "c" {
                        Snippet { break 'myloop; }
                    }
                }
                
                loop {
                    Row {}

                    if cond == "c" {
                        Snippet { break 'myloop; }
                    }
                }
                
                Row {
                    'myloop: while 1 == 2 {
                        Row {}
    
                        if cond == "c" {
                            Snippet { break 'myloop; }
                        }
                    }
                    
                    while 3 == 4 {
                        Row {}
    
                        if cond == "c" {
                            Snippet { break 'myloop; }
                        }
                    } 
                }
            }
                .set_background(Some(Fill::Color))
                .set_background(None)
    };
}

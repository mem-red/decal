use decal::prelude::*;

fn header(owner: &str, repo: &str) -> Decal {
    fragment! {
        Row {
            Text(text! {
                owner,
                "/",
                (repo, { weight: FontWeight::Bold })
             })
        }
            .font_size(80.0)
            .line_height(80.0)
    }
}

fn statistic(value: &str, label: &str, icon: &str) -> Decal {
    fragment! {
        Column {
            Row {
                Image(ImageSource::svg(format!(r##"<svg width="32" height="32" viewBox="0 0 16 16" fill="#6e7681">{icon}</svg>"##)), 32.0, 32.0)
                Text(value).color(rgb(0x0))
            }
                .font_size(32.0)
                .line_height(32.0)
                .align_items(AlignItems::Center)
                .gap(18)
            Text(label)
                .color(rgb(0x6e7681))
                .font_size(24.0)
                .line_height(24.0)
                .margin_left(50)
        }
            .gap(12)
    }
}

fn main() {
    let mut engine = Engine::new(EngineOptions {
        fonts: FontRegistry::new()
            .load_font("Mona Sans", include_bytes!("../fonts/MonaSans-Regular.ttf"))
            .load_font(
                "Mona Sans Bold",
                include_bytes!("../fonts/MonaSans-Bold.ttf"),
            ),
        ..Default::default()
    });

    let mut gh_card = decal! {
        Root(1200.0, 600.0) {
            Column {
                Row {
                    Column {
                        Fragment(header("nasa", "fprime"))
                        Text("F' - A flight software and embedded systems framework")
                            .color(rgb(0x6e7681))
                    }
                        .gap(48)
                        .width(pct(70))
                    Image("https://avatars.githubusercontent.com/u/848102?s=200", 200.0, 200.0)
                }
                    .justify_content(JustifyContent::SpaceBetween)
                    .gap(64)
                    .flex_grow(1.0)

                Row {
                    Fragment(statistic("83", "Contributors", r#"<path d="M2 5.5a3.5 3.5 0 1 1 5.898 2.549 5.508 5.508 0 0 1 3.034 4.084.75.75 0 1 1-1.482.235 4 4 0 0 0-7.9 0 .75.75 0 0 1-1.482-.236A5.507 5.507 0 0 1 3.102 8.05 3.493 3.493 0 0 1 2 5.5ZM11 4a3.001 3.001 0 0 1 2.22 5.018 5.01 5.01 0 0 1 2.56 3.012.749.749 0 0 1-.885.954.752.752 0 0 1-.549-.514 3.507 3.507 0 0 0-2.522-2.372.75.75 0 0 1-.574-.73v-.352a.75.75 0 0 1 .416-.672A1.5 1.5 0 0 0 11 5.5.75.75 0 0 1 11 4Zm-5.5-.5a2 2 0 1 0-.001 3.999A2 2 0 0 0 5.5 3.5Z"></path>"#))
                    Fragment(statistic("53", "Issues", r#"<path d="M8 9.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3Z"></path><path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM1.5 8a6.5 6.5 0 1 0 13 0 6.5 6.5 0 0 0-13 0Z"></path>"#))
                    Fragment(statistic("6", "Discussions", r#"<path d="M1.75 1h8.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0 1 10.25 10H7.061l-2.574 2.573A1.458 1.458 0 0 1 2 11.543V10h-.25A1.75 1.75 0 0 1 0 8.25v-5.5C0 1.784.784 1 1.75 1ZM1.5 2.75v5.5c0 .138.112.25.25.25h1a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h3.5a.25.25 0 0 0 .25-.25v-5.5a.25.25 0 0 0-.25-.25h-8.5a.25.25 0 0 0-.25.25Zm13 2a.25.25 0 0 0-.25-.25h-.5a.75.75 0 0 1 0-1.5h.5c.966 0 1.75.784 1.75 1.75v5.5A1.75 1.75 0 0 1 14.25 12H14v1.543a1.458 1.458 0 0 1-2.487 1.03L9.22 12.28a.749.749 0 0 1 .326-1.275.749.749 0 0 1 .734.215l2.22 2.22v-2.19a.75.75 0 0 1 .75-.75h1a.25.25 0 0 0 .25-.25Z"></path>"#))
                    Fragment(statistic("8k", "Stars", r#"<path d="M8 .25a.75.75 0 0 1 .673.418l1.882 3.815 4.21.612a.75.75 0 0 1 .416 1.279l-3.046 2.97.719 4.192a.751.751 0 0 1-1.088.791L8 12.347l-3.766 1.98a.75.75 0 0 1-1.088-.79l.72-4.194L.818 6.374a.75.75 0 0 1 .416-1.28l4.21-.611L7.327.668A.75.75 0 0 1 8 .25Zm0 2.445L6.615 5.5a.75.75 0 0 1-.564.41l-3.097.45 2.24 2.184a.75.75 0 0 1 .216.664l-.528 3.084 2.769-1.456a.75.75 0 0 1 .698 0l2.77 1.456-.53-3.084a.75.75 0 0 1 .216-.664l2.24-2.183-3.096-.45a.75.75 0 0 1-.564-.41L8 2.694Z"></path>"#))
                    Fragment(statistic("950", "Forks", r#"<path d="M5 5.372v.878c0 .414.336.75.75.75h4.5a.75.75 0 0 0 .75-.75v-.878a2.25 2.25 0 1 1 1.5 0v.878a2.25 2.25 0 0 1-2.25 2.25h-1.5v2.128a2.251 2.251 0 1 1-1.5 0V8.5h-1.5A2.25 2.25 0 0 1 3.5 6.25v-.878a2.25 2.25 0 1 1 1.5 0ZM5 3.25a.75.75 0 1 0-1.5 0 .75.75 0 0 0 1.5 0Zm6.75.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Zm-3 8.75a.75.75 0 1 0-1.5 0 .75.75 0 0 0 1.5 0Z"></path>"#))
                    // GitHub logo
                    Row {
                        Image(
                            ImageSource::svg(
                                r##"<svg viewBox="0 0 16 16" width="46" height="46" fill="#8492af"><path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path></svg>"##
                            ),
                            46.0,
                            46.0
                        )
                    }
                        .justify_content(JustifyContent::End)
                        .flex_grow(1.0)

                }
                    .gap(46)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::SpaceBetween)

                // Languages
                Row {
                    Block {}.background(rgb(0xf34b7d)).width(pct(61.2))
                    Block {}.background(rgb(0xe34c25)).width(pct(17.5))
                    Block {}.background(rgb(0x3572a5)).width(pct(8.9))
                    Block {}.background(rgb(0x3d6118)).width(pct(5.1))
                    Block {}.background(rgb(0xb07219)).width(pct(3.2))
                    Block {}.background(rgb(0xecdebe)).width(pct(2.1))
                    Block {}.background(rgb(0x555555)).width(pct(1.3))
                    Block {}.background(rgb(0xf0e05a)).width(pct(0.7))
                }
                    .height(24)
                    .margin_x(-80)
                    .margin_bottom(-80)
            }
                .padding(80)
                .align_items(AlignItems::Stretch)
                .gap(52)
                .background(rgb(0xffffff))
                .size(pct(100))
        }
            .font_size(32.0)
            .line_height(46.0)
            .color(rgb(0x2f363d))
            .font_family("Mona Sans")
            .text_wrap(TextWrap::Word)
    };

    engine
        .rasterize(&mut gh_card, &RasterizeOptions::default())
        .unwrap()
        .save_png("./output.png")
        .unwrap();
}

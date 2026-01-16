use crate::filters::Filter;

// https://www.w3.org/TR/filter-effects-1/#grayscaleEquivalent

pub fn grayscale(amount: f32) -> Filter {
    let x = 1.0 - amount.clamp(0.0, 1.0);

    Filter::new(|ctx| {
        ctx.color_matrix()
            .matrix([
                [
                    0.2126 + 0.7874 * x,
                    0.7152 - 0.7152 * x,
                    0.0722 - 0.0722 * x,
                    0.0,
                    0.0,
                ],
                [
                    0.2126 - 0.2126 * x,
                    0.7152 + 0.2848 * x,
                    0.0722 - 0.0722 * x,
                    0.0,
                    0.0,
                ],
                [
                    0.2126 - 0.2126 * x,
                    0.7152 - 0.7152 * x,
                    0.0722 + 0.9278 * x,
                    0.0,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0, 0.0],
            ])
            .finish();
    })
}

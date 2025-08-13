use material_colors::{
    dynamic_color::Variant,
    image::{FilterType, ImageReader},
    theme::ThemeBuilder,
};

fn main() {
    let img_path: String = "../../../Pictures/Screenshots/Screenshot From 2025-07-10 21-24-02.png".to_string();
    let mut data = ImageReader::open(img_path).expect("failed to read image");

    // Lancsoz3 takes a little longer, but provides the best pixels for color extraction.
    // However, if you don't like the results, you can always try other FilterType values.
    data.resize(128, 128, FilterType::Lanczos3);

    let theme = ThemeBuilder::with_source(ImageReader::extract_color(&data))
        .variant(Variant::Expressive)
        .build();

    println!("\x1b[48;2;{};{};{}m   \x1b[0m {:#?}", 
        theme.schemes.dark.surface.red,
        theme.schemes.dark.surface.green,
        theme.schemes.dark.surface.blue,
        theme.schemes.dark.surface)
}

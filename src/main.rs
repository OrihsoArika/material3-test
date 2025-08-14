use clap::{Args, Parser, Subcommand};
use std::{
    env::{self, args},
    fs::File,
    io::Write,
    path::PathBuf,
    process::Command,
};

use material_colors::{
    color::Argb,
    dynamic_color::Variant,
    image::{FilterType, ImageReader},
    theme::{Theme, ThemeBuilder},
};
use serde_json;

#[derive(Parser, Debug)]
#[command(name = "Material Design color generator")]
#[command(about = "A program for generating Googles Material Design 3 colorshemes.")]
#[command(version = "0.0.1", long_about = None)]
struct Arguments {
    // path to image
    #[arg(value_name = "IMAGE")]
    image_path: PathBuf,

    #[arg(short, long)]
    variant: Option<String>,

    #[arg(short, long)]
    scheme: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    let username = env::var("USER").expect("couldn't read environment variable `$USER`.");

    let is_dark = true;
    let variant = Variant::TonalSpot;
    let waybar_config_dir = "/home/".to_owned() + &username.to_owned();

    println!("args {:?}", args);

    if is_image(&args.image_path).unwrap() {
        run(
            args.image_path.display().to_string(),
            waybar_config_dir,
            is_dark,
            variant,
        );
    } else {
        println!("not an image");
    }
}

fn is_image(path: &PathBuf) -> Result<bool, &str> {
    if path.is_dir() {
        Err("the given path is a directory")
    } else if !path.exists() {
        Err("the given path is a does not exist")
    } else {
        match path.extension() {
            Some(ext) => {
                if ext == "jpg" || ext == "jpeg" {
                    Ok(true)
                } else if ext == "png" || ext == "webp" {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => {
                Err("couldn't parse file extension, is either empty or something else happened")
            }
        }
    }
}

fn parse_variant(variant_string: String) -> Result<Variant, &'static str> {
    match variant_string.as_str() {
        "content" => Ok(Variant::Content),
        "expresive" => Ok(Variant::Expressive),
        "monochrome" => Ok(Variant::Monochrome),
        "neutral" => Ok(Variant::Neutral),
        "tonal_spot" => Ok(Variant::TonalSpot),
        "vibrant" => Ok(Variant::Vibrant),
        "fidelity" => Ok(Variant::Fidelity),
        "fruit_salad" => Ok(Variant::FruitSalad),
        "rainbow" => Ok(Variant::Rainbow),
        _ => Err("no variant was given"),
    }
}

fn check_args(exec_arguments: Vec<String>) {
    let username = env::var("USER").unwrap();
    let home_dir =
        env::var("HOME").unwrap_or_else(|_| String::from("/home/".to_owned() + &username));

    if exec_arguments.len() == 1 {
        println!("No argument was given");
    } else if exec_arguments[1].to_lowercase() != "generate" {
        println!("No `generate` argument was given");
    } else {
        let mut img_path =
            String::from("../../../Pictures/Screenshots/Screenshot From 2025-07-10 21-24-02.png");
        let mut is_dark = true;
        let mut variant = Variant::TonalSpot;
        let waybar_config_dir = String::from(home_dir + "/.config/waybar");

        if exec_arguments.len() < 3 {
            eprintln!("no image was given");
        } else if exec_arguments.len() >= 3 {
            img_path = exec_arguments[2].clone();
            for (i, argument) in exec_arguments.iter().enumerate().skip(3) {
                let arg_lower = argument.to_lowercase();
                match arg_lower.as_str() {
                    "variant" => match exec_arguments[i + 1].to_lowercase().as_str() {
                        "content" => variant = Variant::Content,
                        "expresive" => variant = Variant::Expressive,
                        "monochrome" => variant = Variant::Monochrome,
                        "neutral" => variant = Variant::Neutral,
                        "tonal_spot" => variant = Variant::TonalSpot,
                        "vibrant" => variant = Variant::Vibrant,
                        "fidelity" => variant = Variant::Fidelity,
                        "fruit_salad" => variant = Variant::FruitSalad,
                        "rainbow" => variant = Variant::Rainbow,
                        _ => {
                            eprintln!("no variant was given")
                        }
                    },
                    "scheme" => match exec_arguments[i + 1].to_lowercase().as_str() {
                        "light" => is_dark = false,
                        "dark" => is_dark = true,
                        _ => {
                            eprintln!("no scheme was given")
                        }
                    },
                    _ => {
                        println!("`{}` argument not avaliable", argument);
                    }
                }
            }
            run(img_path, waybar_config_dir, is_dark, variant);
        }
    }
}

fn run(img_path: String, waybar_config_dir: String, is_dark: bool, variant: Variant) {
    let mut img_data = ImageReader::open(img_path).expect("failed to read image");

    img_data.resize(128, 128, FilterType::Triangle);

    let theme = ThemeBuilder::with_source(ImageReader::extract_color(&img_data))
        .variant(variant)
        .build();

    let theme_json = serde_json::to_string_pretty(&theme).unwrap();

    let mut json_file = File::create("theme.json").unwrap();
    json_file.write_all(theme_json.as_bytes()).unwrap();

    let css_string = write_to_waybar_css(&theme, is_dark).unwrap();
    let mut css_file = File::create(waybar_config_dir + "/colors.css").unwrap();
    css_file.write_all(css_string.as_bytes()).unwrap();
}

fn print_color(color: Argb) {
    println!(
        "\x1b[48;2;{};{};{}m   \x1b[0m {} {} {}",
        color.red, color.green, color.blue, color.red, color.green, color.blue,
    )
}

fn write_to_waybar_css(theme: &Theme, is_dark: bool) -> Result<String, ()> {
    let mut css_buf = String::new();

    if is_dark {
        css_buf += &("@define-color".to_string()
            + " primary #"
            + &theme.schemes.dark.primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary #"
            + &theme.schemes.dark.on_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " primary_container #"
            + &theme.schemes.dark.primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary_container #"
            + &theme.schemes.dark.on_primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " inverse_primary #"
            + &theme.schemes.dark.inverse_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " primary_fixed #"
            + &theme.schemes.dark.primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " primary_fixed_dim #"
            + &theme.schemes.dark.primary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary_fixed #"
            + &theme.schemes.dark.on_primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary_fixed_variant #"
            + &theme.schemes.dark.on_primary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary #"
            + &theme.schemes.dark.secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary #"
            + &theme.schemes.dark.on_secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary_container #"
            + &theme.schemes.dark.secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary_container #"
            + &theme.schemes.dark.on_secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary_fixed #"
            + &theme.schemes.dark.secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary_fixed_dim #"
            + &theme.schemes.dark.secondary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary_fixed #"
            + &theme.schemes.dark.on_secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary_fixed_variant #"
            + &theme.schemes.dark.on_secondary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary #"
            + &theme.schemes.dark.tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary #"
            + &theme.schemes.dark.on_tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary_container #"
            + &theme.schemes.dark.tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary_container #"
            + &theme.schemes.dark.on_tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary_fixed #"
            + &theme.schemes.dark.tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary_fixed_dim #"
            + &theme.schemes.dark.tertiary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary_fixed #"
            + &theme.schemes.dark.on_tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary_fixed_variant #"
            + &theme.schemes.dark.on_tertiary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " error #"
            + &theme.schemes.dark.error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_error #"
            + &theme.schemes.dark.on_error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " error_container #"
            + &theme.schemes.dark.error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_error_container #"
            + &theme.schemes.dark.on_error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_dim #"
            + &theme.schemes.dark.surface_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface #"
            + &theme.schemes.dark.surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_tint #"
            + &theme.schemes.dark.surface_tint.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_bright #"
            + &theme.schemes.dark.surface_bright.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_lowest #"
            + &theme.schemes.dark.surface_container_lowest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_low #"
            + &theme.schemes.dark.surface_container_low.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container #"
            + &theme.schemes.dark.surface_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_high #"
            + &theme.schemes.dark.surface_container_high.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_highest #"
            + &theme.schemes.dark.surface_container_highest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_surface #"
            + &theme.schemes.dark.on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_surface_variant #"
            + &theme.schemes.dark.on_surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " outline #"
            + &theme.schemes.dark.outline.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " outline_variant #"
            + &theme.schemes.dark.outline_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " inverse_surface #"
            + &theme.schemes.dark.inverse_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " inverse_on_surface #"
            + &theme.schemes.dark.inverse_on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_variant #"
            + &theme.schemes.dark.surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " background #"
            + &theme.schemes.dark.background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_background #"
            + &theme.schemes.dark.on_background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " shadow #"
            + &theme.schemes.dark.shadow.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " scrim #"
            + &theme.schemes.dark.scrim.to_hex()
            + ";\n");
    } else {
        css_buf += &("@define-color".to_string()
            + " primary #"
            + &theme.schemes.light.primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary #"
            + &theme.schemes.light.on_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " primary_container #"
            + &theme.schemes.light.primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary_container #"
            + &theme.schemes.light.on_primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " inverse_primary #"
            + &theme.schemes.light.inverse_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " primary_fixed #"
            + &theme.schemes.light.primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " primary_fixed_dim #"
            + &theme.schemes.light.primary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary_fixed #"
            + &theme.schemes.light.on_primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_primary_fixed_variant #"
            + &theme.schemes.light.on_primary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary #"
            + &theme.schemes.light.secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary #"
            + &theme.schemes.light.on_secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary_container #"
            + &theme.schemes.light.secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary_container #"
            + &theme.schemes.light.on_secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary_fixed #"
            + &theme.schemes.light.secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " secondary_fixed_dim #"
            + &theme.schemes.light.secondary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary_fixed #"
            + &theme.schemes.light.on_secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_secondary_fixed_variant #"
            + &theme.schemes.light.on_secondary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary #"
            + &theme.schemes.light.tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary #"
            + &theme.schemes.light.on_tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary_container #"
            + &theme.schemes.light.tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary_container #"
            + &theme.schemes.light.on_tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary_fixed #"
            + &theme.schemes.light.tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " tertiary_fixed_dim #"
            + &theme.schemes.light.tertiary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary_fixed #"
            + &theme.schemes.light.on_tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_tertiary_fixed_variant #"
            + &theme.schemes.light.on_tertiary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " error #"
            + &theme.schemes.light.error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_error #"
            + &theme.schemes.light.on_error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " error_container #"
            + &theme.schemes.light.error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_error_container #"
            + &theme.schemes.light.on_error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_dim #"
            + &theme.schemes.light.surface_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface #"
            + &theme.schemes.light.surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_tint #"
            + &theme.schemes.light.surface_tint.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_bright #"
            + &theme.schemes.light.surface_bright.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_lowest #"
            + &theme.schemes.light.surface_container_lowest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_low #"
            + &theme.schemes.light.surface_container_low.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container #"
            + &theme.schemes.light.surface_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_high #"
            + &theme.schemes.light.surface_container_high.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_container_highest #"
            + &theme.schemes.light.surface_container_highest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_surface #"
            + &theme.schemes.light.on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_surface_variant #"
            + &theme.schemes.light.on_surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " outline #"
            + &theme.schemes.light.outline.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " outline_variant #"
            + &theme.schemes.light.outline_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " inverse_surface #"
            + &theme.schemes.light.inverse_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " inverse_on_surface #"
            + &theme.schemes.light.inverse_on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " surface_variant #"
            + &theme.schemes.light.surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " background #"
            + &theme.schemes.light.background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " on_background #"
            + &theme.schemes.light.on_background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " shadow #"
            + &theme.schemes.light.shadow.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_string()
            + " scrim #"
            + &theme.schemes.light.scrim.to_hex()
            + ";\n");
    }
    Ok(css_buf)
}

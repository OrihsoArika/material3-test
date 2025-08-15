use clap::Parser;
use std::{
    env::{self},
    fs::File,
    io::Write,
    path::PathBuf,
    process::Command,
};

use material_colors::{
    color::Argb,
    dynamic_color::{Variant, variant},
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
    #[arg(short, long, value_name = "IMAGE")]
    image: PathBuf,

    #[arg(short, long)]
    variant: Option<String>,

    #[arg(short, long)]
    scheme: Option<String>,

    #[arg(short, long, value_name = "CONFIG_DIR")]
    waybar_conf_dir: Option<String>,
}

fn main() {
    let args = Arguments::parse();

    check_args(args);

    sigusr2_waybar();
}
fn sigusr2_waybar() {
    Command::new("pkill")
        .arg("-USR2")
        .arg("waybar")
        .spawn()
        .expect("error while restarting Waybar");
}
fn check_args(arguments: Arguments) {
    let username = env::var("USER").expect("couldn't read environment variable `$USER`.");

    let mut is_dark = true;
    let mut variant = Variant::TonalSpot;
    let mut config_dir = "/home/".to_owned() + &username + "/.config";

    config_dir = match arguments.waybar_conf_dir {
        Some(path) => path,
        None => config_dir,
    };

    variant = match arguments.variant {
        Some(var) => parse_variant(var),
        None => variant,
    };

    is_dark = match arguments.scheme {
        Some(scheme) => {
            if scheme == "light" {
                false
            } else {
                true
            }
        }
        None => is_dark,
    };

    if is_image(&arguments.image).unwrap() {
        run(
            arguments.image.display().to_string(),
            config_dir,
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

fn parse_variant(variant_string: String) -> Variant {
    println!("{}", &variant_string);
    match variant_string.as_str() {
        "content" => Variant::Content,
        "expresive" => Variant::Expressive,
        "monochrome" => Variant::Monochrome,
        "neutral" => Variant::Neutral,
        "tonal_spot" => Variant::TonalSpot,
        "vibrant" => Variant::Vibrant,
        "fidelity" => Variant::Fidelity,
        "fruit_salad" => Variant::FruitSalad,
        "rainbow" => Variant::Rainbow,
        _ => {
            println!(
                "no variant was given or didn't recognize the variant.\nusing default (Tonal Spot)"
            );
            Variant::TonalSpot
        }
    }
}

fn run(img_path: String, config_dir: String, is_dark: bool, variant: Variant) {
    let mut img_data = ImageReader::open(img_path).expect("failed to read image");

    img_data.resize(128, 128, FilterType::Triangle);

    let theme = ThemeBuilder::with_source(ImageReader::extract_color(&img_data))
        .variant(variant)
        .build();

    let theme_json = serde_json::to_string_pretty(&theme).unwrap();

    let mut json_file = File::create("theme.json").unwrap();
    json_file.write_all(theme_json.as_bytes()).unwrap();

    create_waybar_css(&theme, is_dark, &(config_dir + "/waybar/colors.css"));
    create_css(
        &theme,
        is_dark,
        &"/home/oshiro/.config/vesktop/themes/Actual Material design colors/colors.css".to_owned(),
    );

    println!("{:?}", theme.source);
}
fn create_css(theme: &Theme, is_dark: bool, config_file_path: &String) {
    let css_string = theme_to_css(&theme, is_dark).unwrap();
    let mut css_file = File::create(config_file_path.to_owned()).unwrap();
    css_file.write_all(css_string.as_bytes()).unwrap();
}
fn create_waybar_css(theme: &Theme, is_dark: bool, config_file_path: &String) {
    let css_string = theme_to_waybar_css(&theme, is_dark).unwrap();
    let mut css_file = File::create(config_file_path.to_owned()).unwrap();
    css_file.write_all(css_string.as_bytes()).unwrap();
}

fn print_color(color: Argb) {
    println!(
        "\x1b[48;2;{};{};{}m   \x1b[0m {} {} {}",
        color.red, color.green, color.blue, color.red, color.green, color.blue,
    )
}

fn theme_to_css(theme: &Theme, is_dark: bool) -> Result<String, ()> {
    let mut css_buf = String::new();

    css_buf += &("# Source: ".to_owned() + &theme.source.to_hex() + "\n");
    css_buf += ":root {\n";
    if is_dark {
        css_buf += &("  --".to_owned() + "primary: #" + &theme.schemes.dark.primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary: #" + &theme.schemes.dark.on_primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "primary_container: #" + &theme.schemes.dark.primary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary_container: #" + &theme.schemes.dark.on_primary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "inverse_primary: #" + &theme.schemes.dark.inverse_primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "primary_fixed: #" + &theme.schemes.dark.primary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "primary_fixed_dim: #" + &theme.schemes.dark.primary_fixed_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary_fixed: #" + &theme.schemes.dark.on_primary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary_fixed_variant: #" + &theme.schemes.dark.on_primary_fixed_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary: #" + &theme.schemes.dark.secondary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary: #" + &theme.schemes.dark.on_secondary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary_container: #" + &theme.schemes.dark.secondary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary_container: #" + &theme.schemes.dark.on_secondary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary_fixed: #" + &theme.schemes.dark.secondary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary_fixed_dim: #" + &theme.schemes.dark.secondary_fixed_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary_fixed: #" + &theme.schemes.dark.on_secondary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary_fixed_variant: #" + &theme.schemes.dark.on_secondary_fixed_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary: #" + &theme.schemes.dark.tertiary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary: #" + &theme.schemes.dark.on_tertiary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary_container: #" + &theme.schemes.dark.tertiary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary_container: #" + &theme.schemes.dark.on_tertiary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary_fixed: #" + &theme.schemes.dark.tertiary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary_fixed_dim: #" + &theme.schemes.dark.tertiary_fixed_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary_fixed: #" + &theme.schemes.dark.on_tertiary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary_fixed_variant: #" + &theme.schemes.dark.on_tertiary_fixed_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "error: #" + &theme.schemes.dark.error.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_error: #" + &theme.schemes.dark.on_error.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "error_container: #" + &theme.schemes.dark.error_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_error_container: #" + &theme.schemes.dark.on_error_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_dim: #" + &theme.schemes.dark.surface_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface: #" + &theme.schemes.dark.surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_tint: #" + &theme.schemes.dark.surface_tint.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_bright: #" + &theme.schemes.dark.surface_bright.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_lowest: #" + &theme.schemes.dark.surface_container_lowest.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_low: #" + &theme.schemes.dark.surface_container_low.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container: #" + &theme.schemes.dark.surface_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_high: #" + &theme.schemes.dark.surface_container_high.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_highest: #" + &theme.schemes.dark.surface_container_highest.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_surface: #" + &theme.schemes.dark.on_surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_surface_variant: #" + &theme.schemes.dark.on_surface_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "outline: #" + &theme.schemes.dark.outline.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "outline_variant: #" + &theme.schemes.dark.outline_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "inverse_surface: #" + &theme.schemes.dark.inverse_surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "inverse_on_surface: #" + &theme.schemes.dark.inverse_on_surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_variant: #" + &theme.schemes.dark.surface_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "background: #" + &theme.schemes.dark.background.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_background: #" + &theme.schemes.dark.on_background.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "shadow: #" + &theme.schemes.dark.shadow.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "scrim: #" + &theme.schemes.dark.scrim.to_hex() + ";\n");
    } else {
        css_buf += &("  --".to_owned() + "primary: #" + &theme.schemes.light.primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary: #" + &theme.schemes.light.on_primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "primary_container: #" + &theme.schemes.light.primary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary_container: #" + &theme.schemes.light.on_primary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "inverse_primary: #" + &theme.schemes.light.inverse_primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "primary_fixed: #" + &theme.schemes.light.primary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "primary_fixed_dim: #" + &theme.schemes.light.primary_fixed_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary_fixed: #" + &theme.schemes.light.on_primary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_primary_fixed_variant: #" + &theme.schemes.light.on_primary_fixed_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary: #" + &theme.schemes.light.secondary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary: #" + &theme.schemes.light.on_secondary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary_container: #" + &theme.schemes.light.secondary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary_container: #" + &theme.schemes.light.on_secondary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary_fixed: #" + &theme.schemes.light.secondary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "secondary_fixed_dim: #" + &theme.schemes.light.secondary_fixed_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary_fixed: #" + &theme.schemes.light.on_secondary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_secondary_fixed_variant: #" + &theme.schemes.light.on_secondary_fixed_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary: #" + &theme.schemes.light.tertiary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary: #" + &theme.schemes.light.on_tertiary.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary_container: #" + &theme.schemes.light.tertiary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary_container: #" + &theme.schemes.light.on_tertiary_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary_fixed: #" + &theme.schemes.light.tertiary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "tertiary_fixed_dim: #" + &theme.schemes.light.tertiary_fixed_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary_fixed: #" + &theme.schemes.light.on_tertiary_fixed.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_tertiary_fixed_variant: #" + &theme.schemes.light.on_tertiary_fixed_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "error: #" + &theme.schemes.light.error.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_error: #" + &theme.schemes.light.on_error.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "error_container: #" + &theme.schemes.light.error_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_error_container: #" + &theme.schemes.light.on_error_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_dim: #" + &theme.schemes.light.surface_dim.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface: #" + &theme.schemes.light.surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_tint: #" + &theme.schemes.light.surface_tint.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_bright: #" + &theme.schemes.light.surface_bright.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_lowest: #" + &theme.schemes.light.surface_container_lowest.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_low: #" + &theme.schemes.light.surface_container_low.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container: #" + &theme.schemes.light.surface_container.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_high: #" + &theme.schemes.light.surface_container_high.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_container_highest: #" + &theme.schemes.light.surface_container_highest.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_surface: #" + &theme.schemes.light.on_surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_surface_variant: #" + &theme.schemes.light.on_surface_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "outline: #" + &theme.schemes.light.outline.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "outline_variant: #" + &theme.schemes.light.outline_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "inverse_surface: #" + &theme.schemes.light.inverse_surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "inverse_on_surface: #" + &theme.schemes.light.inverse_on_surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "surface_variant: #" + &theme.schemes.light.surface_variant.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "background: #" + &theme.schemes.light.background.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "on_background: #" + &theme.schemes.light.on_background.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "shadow: #" + &theme.schemes.light.shadow.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "scrim: #" + &theme.schemes.light.scrim.to_hex() + ";\n");
    }
    css_buf += "}";
    Ok(css_buf)
}
fn theme_to_waybar_css(theme: &Theme, is_dark: bool) -> Result<String, ()> {
    let mut css_buf = String::new();

    if is_dark {
        css_buf += &("@define-color".to_owned()
            + " primary #"
            + &theme.schemes.dark.primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary #"
            + &theme.schemes.dark.on_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " primary_container #"
            + &theme.schemes.dark.primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary_container #"
            + &theme.schemes.dark.on_primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " inverse_primary #"
            + &theme.schemes.dark.inverse_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " primary_fixed #"
            + &theme.schemes.dark.primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " primary_fixed_dim #"
            + &theme.schemes.dark.primary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary_fixed #"
            + &theme.schemes.dark.on_primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary_fixed_variant #"
            + &theme.schemes.dark.on_primary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary #"
            + &theme.schemes.dark.secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary #"
            + &theme.schemes.dark.on_secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary_container #"
            + &theme.schemes.dark.secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary_container #"
            + &theme.schemes.dark.on_secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary_fixed #"
            + &theme.schemes.dark.secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary_fixed_dim #"
            + &theme.schemes.dark.secondary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary_fixed #"
            + &theme.schemes.dark.on_secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary_fixed_variant #"
            + &theme.schemes.dark.on_secondary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary #"
            + &theme.schemes.dark.tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary #"
            + &theme.schemes.dark.on_tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary_container #"
            + &theme.schemes.dark.tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary_container #"
            + &theme.schemes.dark.on_tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary_fixed #"
            + &theme.schemes.dark.tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary_fixed_dim #"
            + &theme.schemes.dark.tertiary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary_fixed #"
            + &theme.schemes.dark.on_tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary_fixed_variant #"
            + &theme.schemes.dark.on_tertiary_fixed_variant.to_hex()
            + ";\n");
        css_buf +=
            &("@define-color".to_owned() + " error #" + &theme.schemes.dark.error.to_hex() + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_error #"
            + &theme.schemes.dark.on_error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " error_container #"
            + &theme.schemes.dark.error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_error_container #"
            + &theme.schemes.dark.on_error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_dim #"
            + &theme.schemes.dark.surface_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface #"
            + &theme.schemes.dark.surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_tint #"
            + &theme.schemes.dark.surface_tint.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_bright #"
            + &theme.schemes.dark.surface_bright.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_lowest #"
            + &theme.schemes.dark.surface_container_lowest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_low #"
            + &theme.schemes.dark.surface_container_low.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container #"
            + &theme.schemes.dark.surface_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_high #"
            + &theme.schemes.dark.surface_container_high.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_highest #"
            + &theme.schemes.dark.surface_container_highest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_surface #"
            + &theme.schemes.dark.on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_surface_variant #"
            + &theme.schemes.dark.on_surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " outline #"
            + &theme.schemes.dark.outline.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " outline_variant #"
            + &theme.schemes.dark.outline_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " inverse_surface #"
            + &theme.schemes.dark.inverse_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " inverse_on_surface #"
            + &theme.schemes.dark.inverse_on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_variant #"
            + &theme.schemes.dark.surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " background #"
            + &theme.schemes.dark.background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_background #"
            + &theme.schemes.dark.on_background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " shadow #"
            + &theme.schemes.dark.shadow.to_hex()
            + ";\n");
        css_buf +=
            &("@define-color".to_owned() + " scrim #" + &theme.schemes.dark.scrim.to_hex() + ";\n");
    } else {
        css_buf += &("@define-color".to_owned()
            + " primary #"
            + &theme.schemes.light.primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary #"
            + &theme.schemes.light.on_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " primary_container #"
            + &theme.schemes.light.primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary_container #"
            + &theme.schemes.light.on_primary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " inverse_primary #"
            + &theme.schemes.light.inverse_primary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " primary_fixed #"
            + &theme.schemes.light.primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " primary_fixed_dim #"
            + &theme.schemes.light.primary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary_fixed #"
            + &theme.schemes.light.on_primary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_primary_fixed_variant #"
            + &theme.schemes.light.on_primary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary #"
            + &theme.schemes.light.secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary #"
            + &theme.schemes.light.on_secondary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary_container #"
            + &theme.schemes.light.secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary_container #"
            + &theme.schemes.light.on_secondary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary_fixed #"
            + &theme.schemes.light.secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " secondary_fixed_dim #"
            + &theme.schemes.light.secondary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary_fixed #"
            + &theme.schemes.light.on_secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_secondary_fixed_variant #"
            + &theme.schemes.light.on_secondary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary #"
            + &theme.schemes.light.tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary #"
            + &theme.schemes.light.on_tertiary.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary_container #"
            + &theme.schemes.light.tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary_container #"
            + &theme.schemes.light.on_tertiary_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary_fixed #"
            + &theme.schemes.light.tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " tertiary_fixed_dim #"
            + &theme.schemes.light.tertiary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary_fixed #"
            + &theme.schemes.light.on_tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_tertiary_fixed_variant #"
            + &theme.schemes.light.on_tertiary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " error #"
            + &theme.schemes.light.error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_error #"
            + &theme.schemes.light.on_error.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " error_container #"
            + &theme.schemes.light.error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_error_container #"
            + &theme.schemes.light.on_error_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_dim #"
            + &theme.schemes.light.surface_dim.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface #"
            + &theme.schemes.light.surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_tint #"
            + &theme.schemes.light.surface_tint.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_bright #"
            + &theme.schemes.light.surface_bright.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_lowest #"
            + &theme.schemes.light.surface_container_lowest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_low #"
            + &theme.schemes.light.surface_container_low.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container #"
            + &theme.schemes.light.surface_container.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_high #"
            + &theme.schemes.light.surface_container_high.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_container_highest #"
            + &theme.schemes.light.surface_container_highest.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_surface #"
            + &theme.schemes.light.on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_surface_variant #"
            + &theme.schemes.light.on_surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " outline #"
            + &theme.schemes.light.outline.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " outline_variant #"
            + &theme.schemes.light.outline_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " inverse_surface #"
            + &theme.schemes.light.inverse_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " inverse_on_surface #"
            + &theme.schemes.light.inverse_on_surface.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " surface_variant #"
            + &theme.schemes.light.surface_variant.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " background #"
            + &theme.schemes.light.background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " on_background #"
            + &theme.schemes.light.on_background.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " shadow #"
            + &theme.schemes.light.shadow.to_hex()
            + ";\n");
        css_buf += &("@define-color".to_owned()
            + " scrim #"
            + &theme.schemes.light.scrim.to_hex()
            + ";\n");
    }
    Ok(css_buf)
}

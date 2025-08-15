mod templates;
mod argument_parse;

use crate::{ argument_parse::*, templates::* };

use clap::Parser;
use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    process::Command,
};

use material_colors::{
    color::Argb,
    dynamic_color::{Variant},
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
fn run(img_path: String, config_dir: String, is_dark: bool, variant: Variant) {
    let mut img_data = ImageReader::open(img_path).expect("failed to read image");

    img_data.resize(128, 128, FilterType::Triangle);

    let theme = ThemeBuilder::with_source(ImageReader::extract_color(&img_data))
        .variant(variant)
        .build();

    let theme_json = serde_json::to_string_pretty(&theme).unwrap();

    let mut json_file = File::create("theme.json").unwrap();
    json_file.write_all(theme_json.as_bytes()).unwrap();

    create_waybar_css(&theme, is_dark, &(config_dir.clone() + "/waybar/colors.css"));
    create_css(
        &theme,
        is_dark,
        &(config_dir.clone() + "/vesktop/themes/Actual Material design colors/colors.css"),
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



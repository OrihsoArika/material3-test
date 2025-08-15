use material_colors::dynamic_color::Variant;
use std::{env, path::PathBuf};

use crate::{Arguments, run};

pub fn check_args(arguments: Arguments) {
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
        let image_path = arguments.image.display().to_string();
        run(image_path, config_dir, is_dark, variant);
    } else {
        println!("not an image");
    }
}

pub fn is_image(path: &PathBuf) -> Result<bool, &str> {
    if path.is_dir() {
        Err("the given path is a directory")
    } else if !path.exists() {
        Err("the given path is a does not exist")
    } else {
        match path.extension() {
            Some(ext) => {
                match ext.to_str().unwrap() {
                    "jpg" => Ok(true),
                    "jpeg" => Ok(true),
                    "png" => Ok(true),
                    _ => Ok(false)
                }
            }
            None => {
                Err("couldn't parse file extension, is either empty or something else happened")
            }
        }
    }
}

pub fn parse_variant(variant_string: String) -> Variant {
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

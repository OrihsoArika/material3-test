use material_colors::theme::Theme;

pub fn theme_to_css(theme: &Theme, is_dark: bool) -> Result<String, ()> {
    let mut css_buf = String::new();

    css_buf += &("# Source: ".to_owned() + &theme.source.to_hex() + "\n");
    css_buf += ":root {\n";
    if is_dark {
        css_buf +=
            &("  --".to_owned() + "primary: #" + &theme.schemes.dark.primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary: #"
            + &theme.schemes.dark.on_primary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "primary_container: #"
            + &theme.schemes.dark.primary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary_container: #"
            + &theme.schemes.dark.on_primary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "inverse_primary: #"
            + &theme.schemes.dark.inverse_primary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "primary_fixed: #"
            + &theme.schemes.dark.primary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "primary_fixed_dim: #"
            + &theme.schemes.dark.primary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary_fixed: #"
            + &theme.schemes.dark.on_primary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary_fixed_variant: #"
            + &theme.schemes.dark.on_primary_fixed_variant.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "secondary: #" + &theme.schemes.dark.secondary.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary: #"
            + &theme.schemes.dark.on_secondary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "secondary_container: #"
            + &theme.schemes.dark.secondary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary_container: #"
            + &theme.schemes.dark.on_secondary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "secondary_fixed: #"
            + &theme.schemes.dark.secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "secondary_fixed_dim: #"
            + &theme.schemes.dark.secondary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary_fixed: #"
            + &theme.schemes.dark.on_secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary_fixed_variant: #"
            + &theme.schemes.dark.on_secondary_fixed_variant.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "tertiary: #" + &theme.schemes.dark.tertiary.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary: #"
            + &theme.schemes.dark.on_tertiary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "tertiary_container: #"
            + &theme.schemes.dark.tertiary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary_container: #"
            + &theme.schemes.dark.on_tertiary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "tertiary_fixed: #"
            + &theme.schemes.dark.tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "tertiary_fixed_dim: #"
            + &theme.schemes.dark.tertiary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary_fixed: #"
            + &theme.schemes.dark.on_tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary_fixed_variant: #"
            + &theme.schemes.dark.on_tertiary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned() + "error: #" + &theme.schemes.dark.error.to_hex() + ";\n");
        css_buf +=
            &("  --".to_owned() + "on_error: #" + &theme.schemes.dark.on_error.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "error_container: #"
            + &theme.schemes.dark.error_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_error_container: #"
            + &theme.schemes.dark.on_error_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_dim: #"
            + &theme.schemes.dark.surface_dim.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "surface: #" + &theme.schemes.dark.surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_tint: #"
            + &theme.schemes.dark.surface_tint.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_bright: #"
            + &theme.schemes.dark.surface_bright.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_lowest: #"
            + &theme.schemes.dark.surface_container_lowest.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_low: #"
            + &theme.schemes.dark.surface_container_low.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container: #"
            + &theme.schemes.dark.surface_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_high: #"
            + &theme.schemes.dark.surface_container_high.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_highest: #"
            + &theme.schemes.dark.surface_container_highest.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_surface: #"
            + &theme.schemes.dark.on_surface.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_surface_variant: #"
            + &theme.schemes.dark.on_surface_variant.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "outline: #" + &theme.schemes.dark.outline.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "outline_variant: #"
            + &theme.schemes.dark.outline_variant.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "inverse_surface: #"
            + &theme.schemes.dark.inverse_surface.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "inverse_on_surface: #"
            + &theme.schemes.dark.inverse_on_surface.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_variant: #"
            + &theme.schemes.dark.surface_variant.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "background: #"
            + &theme.schemes.dark.background.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_background: #"
            + &theme.schemes.dark.on_background.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned() + "shadow: #" + &theme.schemes.dark.shadow.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "scrim: #" + &theme.schemes.dark.scrim.to_hex() + ";\n");
    } else {
        css_buf +=
            &("  --".to_owned() + "primary: #" + &theme.schemes.light.primary.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary: #"
            + &theme.schemes.light.on_primary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "primary_container: #"
            + &theme.schemes.light.primary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary_container: #"
            + &theme.schemes.light.on_primary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "inverse_primary: #"
            + &theme.schemes.light.inverse_primary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "primary_fixed: #"
            + &theme.schemes.light.primary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "primary_fixed_dim: #"
            + &theme.schemes.light.primary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary_fixed: #"
            + &theme.schemes.light.on_primary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_primary_fixed_variant: #"
            + &theme.schemes.light.on_primary_fixed_variant.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "secondary: #" + &theme.schemes.light.secondary.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary: #"
            + &theme.schemes.light.on_secondary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "secondary_container: #"
            + &theme.schemes.light.secondary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary_container: #"
            + &theme.schemes.light.on_secondary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "secondary_fixed: #"
            + &theme.schemes.light.secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "secondary_fixed_dim: #"
            + &theme.schemes.light.secondary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary_fixed: #"
            + &theme.schemes.light.on_secondary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_secondary_fixed_variant: #"
            + &theme.schemes.light.on_secondary_fixed_variant.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "tertiary: #" + &theme.schemes.light.tertiary.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary: #"
            + &theme.schemes.light.on_tertiary.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "tertiary_container: #"
            + &theme.schemes.light.tertiary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary_container: #"
            + &theme.schemes.light.on_tertiary_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "tertiary_fixed: #"
            + &theme.schemes.light.tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "tertiary_fixed_dim: #"
            + &theme.schemes.light.tertiary_fixed_dim.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary_fixed: #"
            + &theme.schemes.light.on_tertiary_fixed.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_tertiary_fixed_variant: #"
            + &theme.schemes.light.on_tertiary_fixed_variant.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned() + "error: #" + &theme.schemes.light.error.to_hex() + ";\n");
        css_buf +=
            &("  --".to_owned() + "on_error: #" + &theme.schemes.light.on_error.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "error_container: #"
            + &theme.schemes.light.error_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_error_container: #"
            + &theme.schemes.light.on_error_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_dim: #"
            + &theme.schemes.light.surface_dim.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "surface: #" + &theme.schemes.light.surface.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_tint: #"
            + &theme.schemes.light.surface_tint.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_bright: #"
            + &theme.schemes.light.surface_bright.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_lowest: #"
            + &theme.schemes.light.surface_container_lowest.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_low: #"
            + &theme.schemes.light.surface_container_low.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container: #"
            + &theme.schemes.light.surface_container.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_high: #"
            + &theme.schemes.light.surface_container_high.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_container_highest: #"
            + &theme.schemes.light.surface_container_highest.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_surface: #"
            + &theme.schemes.light.on_surface.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_surface_variant: #"
            + &theme.schemes.light.on_surface_variant.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "outline: #" + &theme.schemes.light.outline.to_hex() + ";\n");
        css_buf += &("  --".to_owned()
            + "outline_variant: #"
            + &theme.schemes.light.outline_variant.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "inverse_surface: #"
            + &theme.schemes.light.inverse_surface.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "inverse_on_surface: #"
            + &theme.schemes.light.inverse_on_surface.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "surface_variant: #"
            + &theme.schemes.light.surface_variant.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "background: #"
            + &theme.schemes.light.background.to_hex()
            + ";\n");
        css_buf += &("  --".to_owned()
            + "on_background: #"
            + &theme.schemes.light.on_background.to_hex()
            + ";\n");
        css_buf +=
            &("  --".to_owned() + "shadow: #" + &theme.schemes.light.shadow.to_hex() + ";\n");
        css_buf += &("  --".to_owned() + "scrim: #" + &theme.schemes.light.scrim.to_hex() + ";\n");
    }
    css_buf += "}";
    Ok(css_buf)
}
pub fn theme_to_waybar_css(theme: &Theme, is_dark: bool) -> Result<String, ()> {
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

mod common;

use common::{
  contrast_ratio, extract_hex_colors, is_valid_hex, sublime_color_scheme_global, sublime_color_scheme_name,
  sublime_color_scheme_rule,
};

const DARK: &str = include_str!("../sublime/Warm Burnout Dark.sublime-color-scheme");
const LIGHT: &str = include_str!("../sublime/Warm Burnout Light.sublime-color-scheme");
const DARK_UI: &str = include_str!("../sublime/Warm Burnout Dark.sublime-theme");
const LIGHT_UI: &str = include_str!("../sublime/Warm Burnout Light.sublime-theme");

fn parse_json(src: &str) -> serde_json::Value {
  serde_json::from_str(src).expect("invalid Sublime Text color scheme JSON")
}

#[test]
fn dark_is_valid_json() {
  parse_json(DARK);
}

#[test]
fn light_is_valid_json() {
  parse_json(LIGHT);
}

#[test]
fn ui_themes_are_valid_json() {
  parse_json(DARK_UI);
  parse_json(LIGHT_UI);
}

#[test]
fn ui_themes_extend_sublimes_default_theme() {
  for (name, src, style) in [("dark", DARK_UI, "dark"), ("light", LIGHT_UI, "light")] {
    let theme = parse_json(src);
    assert_eq!(theme["extends"].as_str(), Some("Default.sublime-theme"), "{name}");
    assert_eq!(theme["rules"][0]["class"].as_str(), Some("title_bar"), "{name}");
    assert_eq!(theme["rules"][0]["style"].as_str(), Some(style), "{name}");
  }
}

#[test]
fn dark_name_is_warm_burnout_dark() {
  assert_eq!(sublime_color_scheme_name(DARK), "Warm Burnout Dark");
}

#[test]
fn light_name_is_warm_burnout_light() {
  assert_eq!(sublime_color_scheme_name(LIGHT), "Warm Burnout Light");
}

#[test]
fn dark_all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(DARK) {
    assert!(is_valid_hex(hex), "dark line {line}: invalid hex: {hex}");
  }
}

#[test]
fn light_all_hex_colors_are_valid() {
  for (line, hex) in extract_hex_colors(LIGHT) {
    assert!(is_valid_hex(hex), "light line {line}: invalid hex: {hex}");
  }
}

#[test]
fn ui_theme_hex_colors_are_valid() {
  for (name, src) in [("dark", DARK_UI), ("light", LIGHT_UI)] {
    for (line, hex) in extract_hex_colors(src) {
      assert!(is_valid_hex(hex), "{name} UI line {line}: invalid hex: {hex}");
    }
  }
}

#[test]
fn ui_theme_core_colors_match_palette() {
  for (name, src, text, ui_background, accent) in [
    ("dark", DARK_UI, "#bfbdb6", "#14120f", "#ffb454"),
    ("light", LIGHT_UI, "#3a3630", "#EDE6DA", "#855700"),
  ] {
    let theme = parse_json(src);
    let variables = theme["variables"].as_object().expect("missing variables object");
    assert_eq!(variables["text_fg"].as_str(), Some(text), "{name} text");
    assert_eq!(variables["ui_bg"].as_str(), Some(ui_background), "{name} UI background");
    assert_eq!(variables["link_fg"].as_str(), Some(accent), "{name} link");
    assert_eq!(
      variables["progress_bar_fg"].as_str(),
      Some("#b8522e"),
      "{name} progress bar"
    );
  }
}

#[test]
fn dark_globals_match_palette() {
  for (key, expected) in [
    ("background", "#1a1510"),
    ("foreground", "#bfbdb6"),
    ("caret", "#f5c56e"),
    ("selection", "#33393a"),
    ("line_highlight", "#222018"),
    ("gutter", "#1a1510"),
  ] {
    assert_eq!(sublime_color_scheme_global(DARK, key), expected, "dark {key}");
  }
}

#[test]
fn light_globals_match_palette() {
  for (key, expected) in [
    ("background", "#f5ede0"),
    ("foreground", "#3a3630"),
    ("caret", "#8a6600"),
    ("selection", "#e5e8e2"),
    ("line_highlight", "#e2dace"),
    ("gutter", "#f5ede0"),
  ] {
    assert_eq!(sublime_color_scheme_global(LIGHT, key), expected, "light {key}");
  }
}

#[test]
fn dark_syntax_colors_match_palette() {
  for (scope, expected) in [
    ("keyword", "#ff8f40"),
    ("keyword.operator", "#f29668"),
    ("entity.name.function", "#ffb454"),
    ("support.function.builtin", "#ec9878"),
    ("entity.name.type", "#90aec0"),
    ("string", "#b4bc78"),
    ("string.regexp", "#96b898"),
    ("constant", "#d4a8b8"),
    ("entity.name.tag", "#dc9e92"),
    ("variable.other.member", "#ec9878"),
    ("comment", "#b4a89c"),
    ("invalid", "#f49090"),
    ("meta.decorator", "#e6c08a"),
    ("support.type.property-name.css", "#deb074"),
  ] {
    assert_eq!(
      sublime_color_scheme_rule(DARK, scope, "foreground"),
      expected,
      "dark {scope}"
    );
  }
}

#[test]
fn light_syntax_colors_match_palette() {
  for (scope, expected) in [
    ("keyword", "#924800"),
    ("keyword.operator", "#8f4418"),
    ("entity.name.function", "#855700"),
    ("support.function.builtin", "#883850"),
    ("entity.name.type", "#285464"),
    ("string", "#4d5c1a"),
    ("string.regexp", "#286a48"),
    ("constant", "#7e4060"),
    ("entity.name.tag", "#8e4632"),
    ("variable.other.member", "#883850"),
    ("comment", "#544c40"),
    ("invalid", "#b03434"),
    ("meta.decorator", "#7a5a1c"),
    ("support.type.property-name.css", "#74501c"),
  ] {
    assert_eq!(
      sublime_color_scheme_rule(LIGHT, scope, "foreground"),
      expected,
      "light {scope}"
    );
  }
}

#[test]
fn dark_font_styles_preserve_three_tiers() {
  for (scope, expected) in [
    ("keyword", "bold"),
    ("entity.name.tag", "bold"),
    ("entity.name.type", "italic"),
    ("comment", "italic"),
    ("meta.decorator", "italic"),
    ("support.type.property-name.css", "italic"),
    ("keyword.operator", ""),
  ] {
    assert_eq!(
      sublime_color_scheme_rule(DARK, scope, "font_style"),
      expected,
      "dark {scope}"
    );
  }
}

#[test]
fn light_font_styles_preserve_three_tiers() {
  for (scope, expected) in [
    ("keyword", "bold"),
    ("entity.name.tag", "bold"),
    ("entity.name.type", "italic"),
    ("comment", "italic"),
    ("meta.decorator", "italic"),
    ("support.type.property-name.css", "italic"),
    ("keyword.operator", ""),
  ] {
    assert_eq!(
      sublime_color_scheme_rule(LIGHT, scope, "font_style"),
      expected,
      "light {scope}"
    );
  }
}

#[test]
fn syntax_tokens_meet_contrast_requirements() {
  for (name, src, background, minimum) in [("dark", DARK, "#1a1510", 7.0), ("light", LIGHT, "#f5ede0", 4.5)] {
    for scope in [
      "keyword",
      "keyword.operator",
      "entity.name.function",
      "support.function.builtin",
      "entity.name.type",
      "string",
      "string.regexp",
      "constant",
      "entity.name.tag",
      "variable.other.member",
      "comment",
      "invalid",
      "meta.decorator",
      "support.type.property-name.css",
    ] {
      let foreground = sublime_color_scheme_rule(src, scope, "foreground");
      let contrast = contrast_ratio(&foreground, background);
      assert!(
        contrast >= minimum,
        "{name} {scope} ({foreground}) contrast {contrast:.2}:1 < {minimum:.1}:1"
      );
    }
  }
}

#[test]
fn backgrounds_are_not_extreme() {
  assert_ne!(sublime_color_scheme_global(DARK, "background"), "#000000");
  assert_ne!(sublime_color_scheme_global(LIGHT, "background"), "#ffffff");
}

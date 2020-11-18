use std::default::Default;
use std::path::Path;

use lazy_static::lazy_static;
use serde_derive::Deserialize;

use crate::util;

lazy_static! {
    pub static ref SLICK: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#424242")).to_owned(),
        idle_fg: Some(String::from("#ffffff")).to_owned(),
        info_bg: Some(String::from("#2196f3")).to_owned(),
        info_fg: Some(String::from("#ffffff")).to_owned(),
        good_bg: Some(String::from("#8bc34a")).to_owned(),
        good_fg: Some(String::from("#000000")).to_owned(),
        warning_bg: Some(String::from("#ffc107")).to_owned(),
        warning_fg: Some(String::from("#000000")).to_owned(),
        critical_bg: Some(String::from("#f44336")).to_owned(),
        critical_fg: Some(String::from("#ffffff")).to_owned(),
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: Some(String::from("#111111")).to_owned(),
    };

    pub static ref SOLARIZED_DARK: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#002b36")).to_owned(),      // base03
        idle_fg: Some(String::from("#93a1a1")).to_owned(),      // base1
        info_bg: Some(String::from("#268bd2")).to_owned(),      // blue
        info_fg: Some(String::from("#002b36")).to_owned(),      // base03
        good_bg: Some(String::from("#859900")).to_owned(),      // green
        good_fg: Some(String::from("#002b36")).to_owned(),      // base03
        warning_bg: Some(String::from("#b58900")).to_owned(),   // yellow
        warning_fg: Some(String::from("#002b36")).to_owned(),   // base03
        critical_bg: Some(String::from("#dc322f")).to_owned(),  // red
        critical_fg: Some(String::from("#002b36")).to_owned(),  // base03
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref SOLARIZED_LIGHT: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#fdf6e3")).to_owned(),      // base3
        idle_fg: Some(String::from("#586e75")).to_owned(),      // base01
        info_bg: Some(String::from("#268bd2")).to_owned(),      // blue
        info_fg: Some(String::from("#fdf6e3")).to_owned(),      // base3
        good_bg: Some(String::from("#859900")).to_owned(),      // green
        good_fg: Some(String::from("#fdf6e3")).to_owned(),      // base3
        warning_bg: Some(String::from("#b58900")).to_owned(),   // yellow
        warning_fg: Some(String::from("#fdf6e3")).to_owned(),   // base3
        critical_bg: Some(String::from("#dc322f")).to_owned(),  // red
        critical_fg: Some(String::from("#fdf6e3")).to_owned(),  // base3
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref MODERN: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#222D32")).to_owned(),
        idle_fg: Some(String::from("#CFD8DC")).to_owned(),
        info_bg: Some(String::from("#449CDB")).to_owned(),
        info_fg: Some(String::from("#1D1F21")).to_owned(),
        good_bg: Some(String::from("#99b938")).to_owned(),
        good_fg: Some(String::from("#1D1F21")).to_owned(),
        warning_bg: Some(String::from("#FE7E29")).to_owned(),
        warning_fg: Some(String::from("#1D1F21")).to_owned(),
        critical_bg: Some(String::from("#ff5252")).to_owned(),
        critical_fg: Some(String::from("#1D1F21")).to_owned(),
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref PLAIN: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#000000")).to_owned(),
        idle_fg: Some(String::from("#93a1a1")).to_owned(),
        info_bg: Some(String::from("#000000")).to_owned(),
        info_fg: Some(String::from("#93a1a1")).to_owned(),
        good_bg: Some(String::from("#000000")).to_owned(),
        good_fg: Some(String::from("#859900")).to_owned(),
        warning_bg: Some(String::from("#000000")).to_owned(),
        warning_fg: Some(String::from("#b58900")).to_owned(),
        critical_bg: Some(String::from("#000000")).to_owned(),
        critical_fg: Some(String::from("#dc322f")).to_owned(),
        separator: "|".to_owned(),
        separator_bg: Some(String::from("#000000")).to_owned(),
        separator_fg: Some(String::from("#a9a9a9")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref BAD_WOLF: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#444444")).to_owned(),
        idle_fg: Some(String::from("#f5f5f5")).to_owned(),
        info_bg: Some(String::from("#626262")).to_owned(),
        info_fg: Some(String::from("#ffd680")).to_owned(),
        good_bg: Some(String::from("#afff00")).to_owned(),
        good_fg: Some(String::from("#000000")).to_owned(),
        warning_bg: Some(String::from("#ffaf00")).to_owned(),
        warning_fg: Some(String::from("#000000")).to_owned(),
        critical_bg: Some(String::from("#d70000")).to_owned(),
        critical_fg: Some(String::from("#000000")).to_owned(),
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref GRUVBOX_LIGHT: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#fbf1c7")).to_owned(),
        idle_fg: Some(String::from("#3c3836")).to_owned(),
        info_bg: Some(String::from("#458588")).to_owned(),
        info_fg: Some(String::from("#fbf1c7")).to_owned(),
        good_bg: Some(String::from("#98971a")).to_owned(),
        good_fg: Some(String::from("#fbf1c7")).to_owned(),
        warning_bg: Some(String::from("#d79921")).to_owned(),
        warning_fg: Some(String::from("#fbf1c7")).to_owned(),
        critical_bg: Some(String::from("#cc241d")).to_owned(),
        critical_fg: Some(String::from("#fbf1c7")).to_owned(),
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref GRUVBOX_DARK: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#282828")).to_owned(),
        idle_fg: Some(String::from("#ebdbb2")).to_owned(),
        info_bg: Some(String::from("#458588")).to_owned(),
        info_fg: Some(String::from("#ebdbb2")).to_owned(),
        good_bg: Some(String::from("#98971a")).to_owned(),
        good_fg: Some(String::from("#ebdbb2")).to_owned(),
        warning_bg: Some(String::from("#d79921")).to_owned(),
        warning_fg: Some(String::from("#ebdbb2")).to_owned(),
        critical_bg: Some(String::from("#cc241d")).to_owned(),
        critical_fg: Some(String::from("#ebdbb2")).to_owned(),
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref SPACE_VILLAIN: Theme = Theme {
        native_separators: false.to_owned(),
        idle_bg: Some(String::from("#06060f")).to_owned(), //Rich black
        idle_fg: Some(String::from("#c1c1c1")).to_owned(), //Silver
        info_bg: Some(String::from("#00223f")).to_owned(), //Maastricht Blue
        info_fg: Some(String::from("#c1c1c1")).to_owned(), //Silver
        good_bg: Some(String::from("#394049")).to_owned(), //Arsenic
        good_fg: Some(String::from("#c1c1c1")).to_owned(), //Silver
        warning_bg: Some(String::from("#2d1637")).to_owned(), //Dark Purple
        warning_fg: Some(String::from("#c1c1c1")).to_owned(), //Silver
        critical_bg: Some(String::from("#c1c1c1")).to_owned(), //Silver
        critical_fg: Some(String::from("#2c1637")).to_owned(), //Dark Purple
        separator: "\u{e0b2}".to_owned(),
        separator_bg: Some(String::from("auto")).to_owned(),
        separator_fg: Some(String::from("auto")).to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

    pub static ref SEMI_NATIVE: Theme = Theme {
        native_separators: true.to_owned(),
        idle_bg: None.to_owned(),
        idle_fg: Some(String::from("#93a1a1")).to_owned(),
        info_bg: None.to_owned(),
        info_fg: Some(String::from("#93a1a1")).to_owned(),
        good_bg: None.to_owned(),
        good_fg: Some(String::from("#859900")).to_owned(),
        warning_bg: None.to_owned(),
        warning_fg: Some(String::from("#b58900")).to_owned(),
        critical_bg: None.to_owned(),
        critical_fg: Some(String::from("#dc322f")).to_owned(),
        separator: "".to_owned(),
        separator_bg: None.to_owned(),
        separator_fg: None.to_owned(),
        alternating_tint_bg: None.to_owned(),
    };

}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Theme {
    pub native_separators: bool,
    pub idle_bg: Option<String>,
    pub idle_fg: Option<String>,
    pub info_bg: Option<String>,
    pub info_fg: Option<String>,
    pub good_bg: Option<String>,
    pub good_fg: Option<String>,
    pub warning_bg: Option<String>,
    pub warning_fg: Option<String>,
    pub critical_bg: Option<String>,
    pub critical_fg: Option<String>,
    pub separator: String,
    pub separator_bg: Option<String>,
    pub separator_fg: Option<String>,
    pub alternating_tint_bg: Option<String>,
}

impl Default for Theme {
    fn default() -> Self {
        PLAIN.clone()
    }
}

impl Theme {
    pub fn from_name(name: &str) -> Option<Theme> {
        match name {
            "slick" => Some(SLICK.clone()),
            "solarized-dark" => Some(SOLARIZED_DARK.clone()),
            "solarized-light" => Some(SOLARIZED_LIGHT.clone()),
            "plain" => Some(PLAIN.clone()),
            "modern" => Some(MODERN.clone()),
            "bad-wolf" => Some(BAD_WOLF.clone()),
            "gruvbox-light" => Some(GRUVBOX_LIGHT.clone()),
            "gruvbox-dark" => Some(GRUVBOX_DARK.clone()),
            "space-villain" => Some(SPACE_VILLAIN.clone()),
            "semi-native" => Some(SEMI_NATIVE.clone()),
            "native" => Some(NATIVE.clone()),
            _ => None,
        }
    }

    pub fn from_file(file: &str) -> Option<Theme> {
        let full_path = Path::new(file);
        let xdg_path = util::xdg_config_home()
            .join("i3status-rust/themes")
            .join(file);
        let share_path = Path::new(util::USR_SHARE_PATH).join("themes").join(file);

        if full_path.exists() {
            util::deserialize_file(full_path.to_str().unwrap()).ok()
        } else if xdg_path.exists() {
            util::deserialize_file(xdg_path.to_str().unwrap()).ok()
        } else if share_path.exists() {
            util::deserialize_file(share_path.to_str().unwrap()).ok()
        } else {
            None
        }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct ThemeOverrides {
    idle_bg: Option<String>,
    idle_fg: Option<String>,
    info_bg: Option<String>,
    info_fg: Option<String>,
    good_bg: Option<String>,
    good_fg: Option<String>,
    warning_bg: Option<String>,
    warning_fg: Option<String>,
    critical_bg: Option<String>,
    critical_fg: Option<String>,
    separator: Option<String>,
    separator_bg: Option<String>,
    separator_fg: Option<String>,
    alternating_tint_bg: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {
    name: Option<String>,
    file: Option<String>,
    overrides: Option<ThemeOverrides>,
}

impl ThemeConfig {
    pub fn into_theme(self) -> Option<Theme> {
        let mut theme = if let Some(name) = self.name {
            Theme::from_name(&name)
        } else if let Some(file) = self.file {
            Theme::from_file(&file)
        } else {
            None
        }?;
        if let Some(overrides) = self.overrides {
            theme.idle_bg = overrides.idle_bg.or(theme.idle_bg);
            theme.idle_fg = overrides.idle_fg.or(theme.idle_fg);
            theme.info_bg = overrides.info_bg.or(theme.info_bg);
            theme.info_fg = overrides.info_fg.or(theme.info_fg);
            theme.good_bg = overrides.good_bg.or(theme.good_bg);
            theme.good_fg = overrides.good_fg.or(theme.good_fg);
            theme.warning_bg = overrides.warning_bg.or(theme.warning_bg);
            theme.warning_fg = overrides.warning_fg.or(theme.warning_fg);
            theme.critical_bg = overrides.critical_bg.or(theme.critical_bg);
            theme.critical_fg = overrides.critical_fg.or(theme.critical_fg);
            theme.separator = overrides.separator.unwrap_or(theme.separator);
            theme.separator_bg = overrides.separator_bg.or(theme.separator_bg);
            theme.separator_fg = overrides.separator_fg.or(theme.separator_fg);
            theme.alternating_tint_bg = overrides.alternating_tint_bg.or(theme.alternating_tint_bg);
        }
        Some(theme)
    }
}

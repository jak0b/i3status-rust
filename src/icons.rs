use std::collections::HashMap;
use std::fmt;

use lazy_static::lazy_static;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde_derive::Deserialize;

use crate::util;

lazy_static! {
    pub static ref NONE: HashMap<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => "BRIGHT",
        "backlight_full" => "BRIGHT",
        "backlight_1" =>  "BRIGHT",
        "backlight_2" =>  "BRIGHT",
        "backlight_3" =>  "BRIGHT",
        "backlight_4" =>  "BRIGHT",
        "backlight_5" =>  "BRIGHT",
        "backlight_6" =>  "BRIGHT",
        "backlight_7" =>  "BRIGHT",
        "backlight_8" =>  "BRIGHT",
        "backlight_9" =>  "BRIGHT",
        "backlight_10" => "BRIGHT",
        "backlight_11" => "BRIGHT",
        "backlight_12" => "BRIGHT",
        "backlight_13" => "BRIGHT",
        "bat" => "BAT",
        "bat_charging" => "CHG",
        "bat_discharging" => "DCG",
        "bat_empty" => "EMP",
        "bat_full" => "FULL",
        "bat_half" => "BAT",
        "bat_not_available" => "BAT N/A",
        "bat_quarter" => "BAT",
        "bat_three_quarters" => "BAT",
        "bell" => "ON",
        "bell-slash" => "OFF",
        "bluetooth" => "BT",
        "calendar" => "CAL",
        "cogs" => "LOAD",
        "cpu" => "CPU",
        "disk_drive" => "DISK",
        "docker" => "DOCKER",
        "github" => "GITHUB",
        "gpu" => "GPU",
        "headphones" => "HEAD",
        "joystick" => "JOY",
        "keyboard" => "KBD",
        "mail" => "MAIL",
        "memory_mem" => "MEM",
        "memory_swap" => "SWAP",
        "mouse" => "MOUSE",
        "music" => "MUSIC",
        "music_next" => ">",
        "music_pause" => "||",
        "music_play" => ">",
        "music_prev" => "<",
        "net_down" => "DOWN",
        "net_loopback" => "LO",
        "net_up" => "UP ",
        "net_vpn" => "VPN",
        "net_wired" => "ETH",
        "net_wireless" => "WLAN",
        "notification" => "NOTIF",
        "phone" => "PHONE",
        "phone_disconnected" => "PHONE",
        "ping" => "PING",
        "pomodoro" => "POMODORO",
        "resolution" => "RES",
        "tasks" => "TSK",
        "thermometer" => "TEMP",
        "time" => "TIME",
        "toggle_off" => "OFF",
        "toggle_on" => "ON",
        "update" => "UPD",
        "uptime" => "UP",
        "volume_empty" => "VOL",
        "volume_full" => "VOL",
        "volume_half" => "VOL",
        "volume_muted" => "VOL MUTED",
        "microphone_empty" => "MIC ",
        "microphone_full" => "MIC",
        "microphone_half" => "MIC",
        "microphone_muted" => "MIC MUTED",
        "weather_clouds" => "CLOUDY",
        "weather_default" => "WEATHER",
        "weather_rain" => "RAIN",
        "weather_snow" => "SNOW",
        "weather_sun" => "SUNNY",
        "weather_thunder" => "STORM",
        "xrandr" => "SCREEN"
    };

    // FontAwesome 4: https://fontawesome.com/v4.7.0/cheatsheet/
    pub static ref AWESOME: HashMap<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => "\u{1f315}",
        "backlight_full" => "\u{1f311}",
        "backlight_1" =>  "\u{1f314}",
        "backlight_2" =>  "\u{1f314}",
        "backlight_3" =>  "\u{1f314}",
        "backlight_4" =>  "\u{1f314}",
        "backlight_5" =>  "\u{1f313}",
        "backlight_6" =>  "\u{1f313}",
        "backlight_7" =>  "\u{1f313}",
        "backlight_8" =>  "\u{1f313}",
        "backlight_9" =>  "\u{1f313}",
        "backlight_10" => "\u{1f312}",
        "backlight_11" => "\u{1f312}",
        "backlight_12" => "\u{1f312}",
        "backlight_13" => "\u{1f312}",
        "bat_charging" => "\u{f1e6}", // fa-plug
        "bat_discharging" => "\u{f242}", // fa-battery-half
        "bat_empty" => "\u{f244}", // fa-battery-empty
        "bat_full" => "\u{f240}", // fa-battery-full
        "bat_half" => "\u{f242}", // fa-battery-half
        "bat_not_available" => "\u{f244}", // fa-battery-empty
        "bat_quarter" => "\u{f243}", // fa-battery-quarter
        "bat_three_quarters" => "\u{f241}", // fa-battery-three-quarters
        "bell" => "\u{f0f3}", // fa-bell
        "bell-slash" => "\u{f1f7}", // fa-bell-slash-o
        "bluetooth" => "\u{f294}", // fa-bluetooth-b
        "calendar" => "\u{f073}", // fa-calendar
        "cogs" => "\u{f085}", // fa-cogs
        "cpu" => "\u{f0e4}", // fa-dashboard
        "disk_drive" => "\u{f0a0}", // fa-hdd-o
        "docker" => "\u{f21a}", // fa-ship
        "github" => "\u{f09b}", // fa-github
        "gpu" => "\u{f26c}", // fa-television
        "headphones" => "\u{f025}", // fa-headphones
        "joystick" => "\u{f11b}", // fa-gamepad
        "keyboard" => "\u{f11c}", // fa-keyboard-o
        "mail" => "\u{f0e0}", // fa-envelope
        "memory_mem" => "\u{f2db}", // fa-microchip
        "memory_swap" => "\u{f0a0}", // fa-hdd-o
        "mouse" => "\u{f245}", // fa-mouse-pointer
        "music" => "\u{f001}", // fa-music
        "music_next" => "\u{f061}", // fa-arrow-right
        "music_pause" => "\u{f04c}", // fa-pause
        "music_play" => "\u{f04b}", // fa-play
        "music_prev" => "\u{f060}", // fa-arrow-left
        "net_bridge" => "\u{f0e8}", // fa-sitemap
        "net_down" => "\u{2b07}",
        "net_loopback" => "LO",
        "net_modem" => "\u{f095}", // fa-phone
        "net_up" => "\u{2b06}",
        "net_vpn" => "\u{f023}", // fa-lock
        "net_wired" => "\u{f0ac}", // fa-globe
        "net_wireless" => "\u{f1eb}", // fa-wifi
        "notification" => "\u{f0a2}", // fa-bell-o
        "phone" => "\u{f10b}", // fa-mobile
        "phone_disconnected" => "\u{1f4f5}",
        "ping" => "\u{21ba}",
        "pomodoro" => "\u{1f345}",
        "resolution" => "\u{f096}", // fa-square-o
        "tasks" => "\u{f0ae}", // fa-tasks
        "thermometer" => "\u{f2c8}", // fa-thermometer-3
        "time" => "\u{f017}", // fa-clock-o
        "toggle_off" => "\u{f204}", // fa-toggle-off
        "toggle_on" => "\u{f205}", // fa-toggle-on
        "unknown" => "\u{f128}", // fa-question
        "update" => "\u{f062}", // fa-arrow-up
        "uptime" => "\u{f017}", // fa-clock-o
        "volume_empty" => "\u{f026}", // fa-volume-off
        "volume_full" => "\u{f028}", // fa-volume-up
        "volume_half" => "\u{f027}", // fa-volume-down
        "volume_muted" => "\u{f026} \u{f00d}",
        "microphone_empty" => "\u{f130}", // fa-microphone
        "microphone_full" => "\u{f130}", // fa-microphone
        "microphone_half" => "\u{f130}", // fa-microphone
        "microphone_muted" => "\u{f131}", // fa-microphone-slash
        "weather_clouds" => "\u{f0c2}", // fa-cloud
        "weather_default" => "\u{f0c2}", // fa-cloud
        "weather_rain" => "\u{f043}", // fa-tint
        "weather_snow" => "\u{f2dc}", // fa-snowflake-o
        "weather_sun" => "\u{f185}", // fa-sun-o
        "weather_thunder" => "\u{f0e7}", // fa-bolt
        "xrandr" => "\u{f26c}" // fa-television
    };

    // FontAwesome 5: https://fontawesome.com/icons?d=gallery&p=2&m=free
    pub static ref AWESOME5: HashMap<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => "\u{1f315}",
        "backlight_full" => "\u{1f311}",
        "backlight_1" =>  "\u{1f314}",
        "backlight_2" =>  "\u{1f314}",
        "backlight_3" =>  "\u{1f314}",
        "backlight_4" =>  "\u{1f314}",
        "backlight_5" =>  "\u{1f313}",
        "backlight_6" =>  "\u{1f313}",
        "backlight_7" =>  "\u{1f313}",
        "backlight_8" =>  "\u{1f313}",
        "backlight_9" =>  "\u{1f313}",
        "backlight_10" => "\u{1f312}",
        "backlight_11" => "\u{1f312}",
        "backlight_12" => "\u{1f312}",
        "backlight_13" => "\u{1f312}",
        "bat_charging" => "\u{f1e6}",
        "bat_discharging" => "\u{f242}",
        "bat_empty" => "\u{f244}",
        "bat_full" => "\u{f240}",
        "bat_half" => "\u{f242}",
        "bat_quarter" => "\u{f243}",
        "bat_three_quarters" => "\u{f241}",
        "bell" => "\u{f0f3}",
        "bell-slash" => "\u{f1f6}",
        "bluetooth" => "\u{f294}",
        "calendar" => "\u{f073}",
        "cogs" => "\u{f085}",
        "cpu" => "\u{f3fd}",
        "disk_drive" => "\u{f8b5}",
        "docker" => "\u{f21a}",
        "github" => "\u{f09b}",
        "gpu" => "\u{f26c}",
        "headphones" => "\u{f025}",
        "joystick" => "\u{f11b}",
        "keyboard" => "\u{f11c}",
        "mail" => "\u{f0e0}",
        "memory_mem" => "\u{f2db}",
        "memory_swap" => "\u{f0a0}",
        "mouse" => "\u{f245}",
        "music" => "\u{f001}",
        "music_next" => "\u{f061}",
        "music_pause" => "\u{f04c}",
        "music_play" => "\u{f04b}",
        "music_prev" => "\u{f060}",
        "net_bridge" => "\u{f0e8}",
        "net_down" => "\u{f019}",
        "net_loopback" => "LO ",
        "net_modem" => "\u{f095}",
        "net_up" => "\u{f093}",
        "net_vpn" => "\u{f023}",
        "net_wired" => "\u{f6ff}",
        "net_wireless" => "\u{f1eb}",
        "notification" => "\u{f0f3}",
        "phone" => "\u{f3cd}",
        "phone_disconnected" => "\u{1f4f5}",
        "ping" => "\u{f362}",
        "pomodoro" => "\u{1f345}",
        "resolution" => "\u{f096}", // fa-square-o
        "tasks" => "\u{f0ae}",
        "thermometer" => "\u{f2c8}",
        "time" => "\u{f017}",
        "toggle_off" => "\u{f204}",
        "toggle_on" => "\u{f205}",
        "unknown" => "\u{f128}",
        "update" => "\u{f062}",
        "uptime" => "\u{f2f2}",
        "volume_empty" => "\u{f026}",
        "volume_full" => "\u{f028}",
        "volume_half" => "\u{f027}",
        "volume_muted" => "\u{f6a9}",
        "microphone_full" => "\u{f3c9}",
        "microphone_half" => "\u{f3c9}",
        "microphone_empty" => "\u{f3c9}",
        "microphone_muted" => "\u{f539}",
        "weather_clouds" => "\u{f0c2}",
        "weather_default" => "\u{f0c2}", // Cloud symbol as default
        "weather_rain" => "\u{f043}",
        "weather_snow" => "\u{f2dc}",
        "weather_sun" => "\u{f185}",
        "weather_thunder" => "\u{f0e7}",
        "xrandr" => "\u{f26c}"
    };

    // Material Design icons by Google
    // https://github.com/google/material-design-icons/blob/master/font/MaterialIcons-Regular.codepoints
    pub static ref MATERIAL: HashMap<String, String> = map_to_owned! {
        "" => "",
        "bat_charging" => "\u{e1a3}", // battery_charging_full
        "bat_discharging" => "\u{e19c}", // battery_alert
        "bat_empty" => "\u{e19c}", // battery_alert
        "bat_full" => "\u{e1a4}", // battery_full
        "bat_half" => "\u{e1a5}", // battery_std
        "bat_quarter" => "\u{e1a5}",
        "bat_three_quarters" => "\u{e1a5}",
        "bat_not_available" => "\u{e1a6}", // battery_unknown
        "bell" => "\u{e7f4}", // notifications
        "bell-slash" => "\u{e7f8}", // notifications_paused
        "bluetooth" => "\u{e1a7}", // bluetooth
        "calendar" => "\u{e935}", // calendar_today
        "cogs" => "\u{e8b8}", // settings
        "cpu" => "\u{e640}", // network_check
        "disk_drive" => "\u{e1db}", // storage
        "docker" => "\u{e532}", // directions_boat
        "github" => "\u{e86f}", // code
        "gpu" => "\u{e333}", // tv
        "headphones" => "\u{e60f}", // bluetooth_audio
        "joystick" => "\u{e30f}", // gamepad
        "keyboard" => "\u{e312}", // keyboard
        "mail" => "\u{e0be}", // email
        "memory_mem" => "\u{e322}", // memory
        "memory_swap" => "\u{e8d4}", // swap_horiz
        "mouse" => "\u{e323}", // mouse
        "music" => "\u{e405}", // music_note
        "music_next" => "\u{e044}", // skip_next
        "music_pause" => "\u{e034}", // skip_next
        "music_play" => "\u{e037}", // play_arrow
        "music_prev" => "\u{e045}", // skip_previous
        "net_loopback" => "LO",
        "notification" => "\u{e7f7}", // notifications_active
        "phone" => "\u{e324}", // phone_android
        "phone_disconnected" => "\u{e339}", // device_unknown
        "ping" => "\u{e62a}", // system_update
        "pomodoro" => "\u{1f345}",
        "resolution" => "\u{f152}", // crop-square-rounded
        "tasks" => "\u{e8f9}",
        "thermometer" => "\u{e1ff}", // device_thermostat
        "time" => "\u{e192}", // access_time
        "toggle_off" => "\u{e836}", // radio_button_on
        "toggle_on" => "\u{e837}", // radio_button_on
        "update" => "\u{e8d7}", // system_update_alt
        "uptime" => "\u{e425}", // timer
        "volume_empty" => "\u{e04e}", // volume_mute
        "volume_full" => "\u{e050}", // volume_up
        "volume_half" => "\u{e04d}", // volume_down
        "volume_muted" => "\u{e04f}", // volume_off
        "microphone_full" => "\u{e029}", // mic
        "microphone_half" => "\u{e029}", // mic
        "microphone_empty" => "\u{e02a}", // mic_none
        "microphone_muted" => "\u{e02b}", // mic_off
        "weather_clouds" => "\u{e42d}", // wb_cloudy
        "weather_default" => "\u{e42d}", // wb_cloudy
        "weather_sun" => "\u{e430}", // wb_sunny
        "xrandr" => "\u{e31e}"
    };

    // Material from NerdFont
    // https://www.nerdfonts.com/cheat-sheet
    pub static ref MATERIAL_NF: HashMap<String, String> = map_to_owned! {
        "" => "",
        "backlight_empty" => "\u{e38d}", // nf-weather-moon_new
        "backlight_full" => "\u{e39b}", // nf-weather-moon_full
        "backlight_1" => "\u{e3d4}", // nf-weather-moon_alt_waxing_gibbous_6
        "backlight_2" => "\u{e3d3}", // nf-weather-moon_alt_waxing_gibbous_5
        "backlight_3" => "\u{e3d2}", // nf-weather-moon_alt_waxing_gibbous_4
        "backlight_4" => "\u{e3d1}", // nf-weather-moon_alt_waxing_gibbous_3
        "backlight_5" => "\u{e3d0}", // nf-weather-moon_alt_waxing_gibbous_2
        "backlight_6" => "\u{e3cf}", // nf-weather-moon_alt_waxing_gibbous_1
        "backlight_7" => "\u{e3ce}", // nf-weather-moon_alt_first_quarter
        "backlight_8" => "\u{e3cd}", // nf-weather-moon_alt_waxing_crescent_6
        "backlight_9" => "\u{e3cc}", // nf-weather-moon_alt_waxing_crescent_5
        "backlight_10" => "\u{e3cb}", // nf-weather-moon_alt_waxing_crescent_4
        "backlight_11" => "\u{e3ca}", // nf-weather-moon_alt_waxing_crescent_3
        "backlight_12" => "\u{e3c9}", // nf-weather-moon_alt_waxing_crescent_2
        "backlight_13" => "\u{e3c8}", // nf-weather-moon_alt_waxing_crescent_1
        "bat_charging" => "\u{f583}", // nf-mdi-battery_charging
        "bat_discharging" => "\u{f57d}", // nf-mdi-battery_50
        "bat_empty" => "\u{f58d}", // nf-mdi-battery_outline
        "bat_full" => "\u{f578}", // nf-mdi-battery
        "bat_half" => "\u{f57d}", // nf-mdi-battery_50
        "bat_quarter" => "\u{f57a}",// nf-mdi-battery_20
        "bat_three_quarters" => "\u{f57f}", // nf-mdi-battery_70
        "bell" => "\u{f599}", // nf-mdi-bell
        "bell-slash" => "\u{f59a}", // nf-mdi-bell_off
        "bluetooth" => "\u{f5ae}", // nf-mdi-bluetooth
        "calendar" => "\u{f5ec}", // nf-mdi-calendar
        "cogs" => "\u{f992}", // nf-mdi-settings
        "cpu" => "\u{f9c4}", // nf-mdi-speedometer
        "disk_drive" => "\u{f7c9}", // nf-mdi-harddisk
        "docker" => "\u{f308}", // nf-linux-docker
        "github" => "\u{f7a3}", // nf-mdi-github_circle
        "gpu" => "\u{f878}", // nf-mdi-monitor
        "headphones" => "\u{f7ca}", // nf-mdi-headphones
        "joystick" => "\u{f796}", // nf-mdi-gamepad_variant
        "keyboard" => "\u{f80b}", // nf-mdi-keyboard
        "mail" => "\u{f6ed}", // nf-mdi-email
        "memory_mem" => "\u{f85a}", // nf-mdi-memory
        "memory_swap" => "\u{f7c9}", // nf-mdi-harddisk
        "mouse" => "\u{f87c}", // nf-mdi-mouse
        "music" => "\u{f886}", // nf-mdi-music_note
        "music_next" => "\u{f9ac}", // nf-mdi-skip_next
        "music_pause" => "\u{f8e3}", // nf-mdi-pause
        "music_play" => "\u{f909}", // nf-mdi-play
        "music_prev" => "\u{f9ad}", // nf-mdi-skip_previous
        "net_bridge" => "\u{f9a9}", // nf-mdi-sitemap
        "net_down" => "\u{f6d9}", // nf-mdi-download
        "net_loopback" => "\u{fbe9}", // nf-mdi-loop
        "net_modem" => "\u{f8f1}", // nf-mdi-phone
        "net_up" => "\u{fa51}", // nf-mdi-upload
        "net_vpn" => "\u{fa81}", // nf-mdi-vpn
        "net_wired" => "\u{f6ff}", // nf-mdi-ethernet
        "net_wireless" => "\u{faa8}", // nf-mdi-wifi
        "notification" => "\u{f599}", // nf-mdi-bell
        "phone" => "\u{f8f1}", // nf-mdi-phone
        "phone_disconnected" => "\u{fb57}", // nf-mdi-phone_minus
        "ping" => "\u{fa1e}", // nf-mdi-timer_sand
        "pomodoro" => "\u{e001}", // nf-pom-pomodoro_done
        "resolution" => "\u{f792}", // nf-mdi-fullscreen
        "tasks" => "\u{fac6}", // nf-mdi-playlist_check
        "thermometer" => "\u{fa0e}", // nf-mdi-thermometer
        "time" => "\u{f64f}", // nf-mdi-clock
        "toggle_off" => "\u{fa21}", // nf-mdi-toggle_switch_off
        "toggle_on" => "\u{fa20}", // nf-mdi-toggle_switch
        "unknown" => "\u{f685}", // nf-mdi-comment_question_outline | TODO: Make default?
        "update" => "\u{fbae}", // nf-mdi-update
        "uptime" => "\u{f652}", // nf-mdi-clock_in
        "volume_empty" => "\u{fa7e}", // nf-mdi-volume_low
        "volume_full" => "\u{fa7d}", // nf-mdi-volume_high
        "volume_half" => "\u{fa7f}", // nf-mdi-volume_medium
        "volume_muted" => "\u{f466}", // nf-mdi-volume_mute
        "microphone_full" => "\u{f86b}", // nf-mdi-microphone
        "microphone_half" => "\u{f86b}", // nf-mdi-microphone
        "microphone_empty" => "\u{f86d}", // nf-mdi-microphone_outline
        "microphone_muted" => "\u{f86c}", // nf-mdi-microphone_off
        "weather_clouds" => "\u{fa8f}", // nf-mdi-weather_cloudy
        "weather_default" => "\u{fa8f}", // Cloud symbol as default
        "weather_rain" => "\u{fa95}", // nf-mdi-weather_pouring
        "weather_snow" => "\u{fa97}", // nf-mdi-weather_snowy
        "weather_sun" => "\u{fa98}", // nf-mdi-weather_sunny
        "weather_thunder" => "\u{e31d}", // nf-weather-thunderstorm
        "xrandr" => "\u{f879}" // nf-mdi-monitor_multiple
    };
}

#[derive(Debug, Clone)]
pub struct Icons(pub HashMap<String, String>);

impl Default for Icons {
    fn default() -> Self {
        Self(NONE.clone())
    }
}

impl Icons {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "material" => Some(Icons(MATERIAL.clone())),
            "material-nf" => Some(Icons(MATERIAL_NF.clone())),
            "awesome" => Some(Icons(AWESOME.clone())),
            "awesome5" => Some(Icons(AWESOME5.clone())),
            "none" => Some(Icons(NONE.clone())),
            _ => None,
        }
    }

    pub fn from_file(file: &str) -> Option<Self> {
        let file = util::find_file(file, Some("icons"), Some(".toml"))?;
        let icons: HashMap<String, String> = util::deserialize_file(&file).ok()?;
        Some(Icons(icons))
    }
}

impl<'de> Deserialize<'de> for Icons {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            File,
            Overrides,
        }

        struct IconsVisitor;

        impl<'de> Visitor<'de> for IconsVisitor {
            type Value = Icons;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Icons")
            }

            /// Handle configs like:
            ///
            /// ```toml
            /// icons = "awesome"
            /// ```
            fn visit_str<E>(self, name: &str) -> Result<Icons, E>
            where
                E: de::Error,
            {
                Icons::from_name(name)
                    .ok_or_else(|| de::Error::custom(format!("Icon set \"{}\" not found.", name)))
            }

            /// Handle configs like:
            ///
            /// ```toml
            /// [icons]
            /// name = "awesome"
            /// ```
            fn visit_map<V>(self, mut map: V) -> Result<Icons, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut icons = None;
                let mut overrides: Option<HashMap<String, String>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if icons.is_some() {
                                return Err(de::Error::duplicate_field("name or file"));
                            }
                            let name = map.next_value()?;
                            icons = Some(Icons::from_name(name).ok_or_else(|| {
                                de::Error::custom(format!("Icon set \"{}\" not found.", name))
                            })?);
                        }
                        Field::File => {
                            if icons.is_some() {
                                return Err(de::Error::duplicate_field("name or file"));
                            }
                            let file = map.next_value()?;
                            icons = Some(Icons::from_file(file).ok_or_else(|| {
                                de::Error::custom(format!(
                                    "Failed to load icon set from file {}.",
                                    file
                                ))
                            })?);
                        }
                        Field::Overrides => {
                            if overrides.is_some() {
                                return Err(de::Error::duplicate_field("overrides"));
                            }
                            overrides = Some(map.next_value()?);
                        }
                    }
                }
                let mut icons = icons.unwrap_or_default();
                if let Some(overrides) = overrides {
                    for icon in overrides {
                        icons.0.insert(icon.0, icon.1);
                    }
                }
                Ok(icons)
            }
        }

        deserializer.deserialize_any(IconsVisitor)
    }
}

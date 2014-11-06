use std::os::homedir;

/// Common configuration options for the window manager.
pub struct Config {
    /// Whether focus follows mouse movements or
    /// only click events and keyboard movements.
    pub focus_follows_mouse: bool,
    /// Border color for focused windows.
    pub focus_border_color: u32,
    /// Border color for unfocused windows.
    pub border_color: u32,
    /// Border width. This is the same for both, focused and unfocused.
    pub border_width: u32,
    /// Default spacing between windows
    pub spacing: u32,
    /// Default terminal to start
    pub terminal: (String, String),
    /// Path to the logfile
    pub logfile: String,
    /// Default tags for workspaces
    pub tags: Vec<String>
}

impl Config {
    /// Create the default configuration.
    pub fn default() -> Config {
        Config {
            focus_follows_mouse: true,
            focus_border_color:  0x00B6FFB0,
            border_color:        0x00FFB6B0,
            border_width:        2,
            spacing:             10,
            terminal:            (String::from_str("xterm"), String::from_str("-fg White -bg Black")),
            logfile:             format!("{}/.wtftw.log", homedir().unwrap().to_c_str()),
            tags:                vec!(
                                     String::from_str("1: term"), 
                                     String::from_str("2: web"),
                                     String::from_str("3: code"),
                                     String::from_str("4 media"))

        }
    }
}
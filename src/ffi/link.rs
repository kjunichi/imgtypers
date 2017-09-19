#[cfg(windows)]
#[link(name = "imgtype")]
#[link(name = "user32")]
extern "system" {}

#[cfg(not(windows))]
#[link(name = "imgtype")]
extern "C" {}

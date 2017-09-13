#[cfg(windows)]
#[link(name = "imgtype")]
#[link(name = "user32")]
extern "C" {}

#[cfg(not(windows))]
#[link(name = "imgtype")]
extern "C" {}

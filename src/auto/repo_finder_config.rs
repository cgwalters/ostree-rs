// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use RepoFinder;
use glib::translate::*;
use ostree_sys;
use std::fmt;

glib_wrapper! {
    pub struct RepoFinderConfig(Object<ostree_sys::OstreeRepoFinderConfig, ostree_sys::OstreeRepoFinderConfigClass, RepoFinderConfigClass>) @implements RepoFinder;

    match fn {
        get_type => || ostree_sys::ostree_repo_finder_config_get_type(),
    }
}

impl RepoFinderConfig {
    #[cfg(any(feature = "v2018_6", feature = "dox"))]
    pub fn new() -> RepoFinderConfig {
        unsafe {
            from_glib_full(ostree_sys::ostree_repo_finder_config_new())
        }
    }
}

#[cfg(any(feature = "v2018_6", feature = "dox"))]
impl Default for RepoFinderConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_REPO_FINDER_CONFIG: Option<&RepoFinderConfig> = None;

impl fmt::Display for RepoFinderConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepoFinderConfig")
    }
}

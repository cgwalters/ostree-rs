// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BootconfigParser;
#[cfg(any(feature = "v2016_4", feature = "dox"))]
use DeploymentUnlockedState;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Deployment(Object<ffi::OstreeDeployment>);

    match fn {
        get_type => || ffi::ostree_deployment_get_type(),
    }
}

impl Deployment {
    pub fn new(index: i32, osname: &str, csum: &str, deployserial: i32, bootcsum: &str, bootserial: i32) -> Deployment {
        unsafe {
            from_glib_full(ffi::ostree_deployment_new(index, osname.to_glib_none().0, csum.to_glib_none().0, deployserial, bootcsum.to_glib_none().0, bootserial))
        }
    }

    pub fn hash(&self) -> u32 {
        unsafe {
            ffi::ostree_deployment_hash(ToGlibPtr::<*mut ffi::OstreeDeployment>::to_glib_none(self).0 as glib_ffi::gconstpointer)
        }
    }

    #[cfg(any(feature = "v2018_3", feature = "dox"))]
    pub fn origin_remove_transient_state(origin: &glib::KeyFile) {
        unsafe {
            ffi::ostree_deployment_origin_remove_transient_state(origin.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    pub fn unlocked_state_to_string(state: DeploymentUnlockedState) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ostree_deployment_unlocked_state_to_string(state.to_glib()))
        }
    }
}

pub trait DeploymentExt {
    fn clone(&self) -> Option<Deployment>;

    fn equal(&self, bp: &Deployment) -> bool;

    fn get_bootconfig(&self) -> Option<BootconfigParser>;

    fn get_bootcsum(&self) -> Option<String>;

    fn get_bootserial(&self) -> i32;

    fn get_csum(&self) -> Option<String>;

    fn get_deployserial(&self) -> i32;

    fn get_index(&self) -> i32;

    fn get_origin(&self) -> Option<glib::KeyFile>;

    fn get_origin_relpath(&self) -> Option<String>;

    fn get_osname(&self) -> Option<String>;

    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    fn get_unlocked(&self) -> DeploymentUnlockedState;

    #[cfg(any(feature = "v2018_3", feature = "dox"))]
    fn is_pinned(&self) -> bool;

    #[cfg(any(feature = "v2018_3", feature = "dox"))]
    fn is_staged(&self) -> bool;

    fn set_bootconfig(&self, bootconfig: &BootconfigParser);

    fn set_bootserial(&self, index: i32);

    fn set_index(&self, index: i32);

    fn set_origin(&self, origin: &glib::KeyFile);
}

impl<O: IsA<Deployment>> DeploymentExt for O {
    fn clone(&self) -> Option<Deployment> {
        unsafe {
            from_glib_full(ffi::ostree_deployment_clone(self.to_glib_none().0))
        }
    }

    fn equal(&self, bp: &Deployment) -> bool {
        unsafe {
            from_glib(ffi::ostree_deployment_equal(ToGlibPtr::<*mut ffi::OstreeDeployment>::to_glib_none(self).0 as glib_ffi::gconstpointer, ToGlibPtr::<*mut ffi::OstreeDeployment>::to_glib_none(bp).0 as glib_ffi::gconstpointer))
        }
    }

    fn get_bootconfig(&self) -> Option<BootconfigParser> {
        unsafe {
            from_glib_none(ffi::ostree_deployment_get_bootconfig(self.to_glib_none().0))
        }
    }

    fn get_bootcsum(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ostree_deployment_get_bootcsum(self.to_glib_none().0))
        }
    }

    fn get_bootserial(&self) -> i32 {
        unsafe {
            ffi::ostree_deployment_get_bootserial(self.to_glib_none().0)
        }
    }

    fn get_csum(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ostree_deployment_get_csum(self.to_glib_none().0))
        }
    }

    fn get_deployserial(&self) -> i32 {
        unsafe {
            ffi::ostree_deployment_get_deployserial(self.to_glib_none().0)
        }
    }

    fn get_index(&self) -> i32 {
        unsafe {
            ffi::ostree_deployment_get_index(self.to_glib_none().0)
        }
    }

    fn get_origin(&self) -> Option<glib::KeyFile> {
        unsafe {
            from_glib_none(ffi::ostree_deployment_get_origin(self.to_glib_none().0))
        }
    }

    fn get_origin_relpath(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::ostree_deployment_get_origin_relpath(self.to_glib_none().0))
        }
    }

    fn get_osname(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::ostree_deployment_get_osname(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2016_4", feature = "dox"))]
    fn get_unlocked(&self) -> DeploymentUnlockedState {
        unsafe {
            from_glib(ffi::ostree_deployment_get_unlocked(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2018_3", feature = "dox"))]
    fn is_pinned(&self) -> bool {
        unsafe {
            from_glib(ffi::ostree_deployment_is_pinned(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2018_3", feature = "dox"))]
    fn is_staged(&self) -> bool {
        unsafe {
            from_glib(ffi::ostree_deployment_is_staged(self.to_glib_none().0))
        }
    }

    fn set_bootconfig(&self, bootconfig: &BootconfigParser) {
        unsafe {
            ffi::ostree_deployment_set_bootconfig(self.to_glib_none().0, bootconfig.to_glib_none().0);
        }
    }

    fn set_bootserial(&self, index: i32) {
        unsafe {
            ffi::ostree_deployment_set_bootserial(self.to_glib_none().0, index);
        }
    }

    fn set_index(&self, index: i32) {
        unsafe {
            ffi::ostree_deployment_set_index(self.to_glib_none().0, index);
        }
    }

    fn set_origin(&self, origin: &glib::KeyFile) {
        unsafe {
            ffi::ostree_deployment_set_origin(self.to_glib_none().0, origin.to_glib_none().0);
        }
    }
}

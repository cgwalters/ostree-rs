// This file was generated by gir (https://github.com/gtk-rs/gir @ ffda6f9)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RepoTransactionStats(Boxed<ffi::OstreeRepoTransactionStats>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::ostree_repo_transaction_stats_get_type(), ptr as *mut _) as *mut ffi::OstreeRepoTransactionStats,
        free => |ptr| gobject_ffi::g_boxed_free(ffi::ostree_repo_transaction_stats_get_type(), ptr as *mut _),
        get_type => || ffi::ostree_repo_transaction_stats_get_type(),
    }
}
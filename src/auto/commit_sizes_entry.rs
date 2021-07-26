// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::ObjectType;
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommitSizesEntry(Boxed<ffi::OstreeCommitSizesEntry>);

    match fn {
        copy => |ptr| ffi::ostree_commit_sizes_entry_copy(ptr),
        free => |ptr| ffi::ostree_commit_sizes_entry_free(ptr),
        type_ => || ffi::ostree_commit_sizes_entry_get_type(),
    }
}

impl CommitSizesEntry {
    #[doc(alias = "ostree_commit_sizes_entry_new")]
    pub fn new(checksum: &str, objtype: ObjectType, unpacked: u64, archived: u64) -> Option<CommitSizesEntry> {
        unsafe {
            from_glib_full(ffi::ostree_commit_sizes_entry_new(checksum.to_glib_none().0, objtype.into_glib(), unpacked, archived))
        }
    }
}

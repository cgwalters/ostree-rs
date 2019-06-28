// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use ostree_sys;
use std::fmt;

glib_wrapper! {
    pub struct RepoFinder(Interface<ostree_sys::OstreeRepoFinder>);

    match fn {
        get_type => || ostree_sys::ostree_repo_finder_get_type(),
    }
}

impl RepoFinder {
    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //pub fn resolve_all_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result</*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 2 }, Error>) + Send + 'static>(finders: &[RepoFinder], refs: &[&CollectionRef], parent_repo: &Repo, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ostree_sys:ostree_repo_finder_resolve_all_async() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //pub fn resolve_all_async_future(finders: &[RepoFinder], refs: &[&CollectionRef], parent_repo: &Repo) -> Box_<dyn future::Future<Output = Result</*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 2 }, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let finders = finders.clone();
        //let refs = refs.clone();
        //let parent_repo = parent_repo.clone();
        //GioFuture::new(&(), move |_obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    Self::resolve_all_async(
        //        &finders,
        //        &refs,
        //        &parent_repo,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}
}

pub const NONE_REPO_FINDER: Option<&RepoFinder> = None;

pub trait RepoFinderExt: 'static {
    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //fn resolve_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result</*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 2 }, Error>) + Send + 'static>(&self, refs: &[&CollectionRef], parent_repo: &Repo, cancellable: Option<&P>, callback: Q);

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //fn resolve_async_future(&self, refs: &[&CollectionRef], parent_repo: &Repo) -> Box_<dyn future::Future<Output = Result</*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 2 }, Error>> + std::marker::Unpin>;
}

impl<O: IsA<RepoFinder>> RepoFinderExt for O {
    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //fn resolve_async<P: IsA<gio::Cancellable>, Q: FnOnce(Result</*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 2 }, Error>) + Send + 'static>(&self, refs: &[&CollectionRef], parent_repo: &Repo, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ostree_sys:ostree_repo_finder_resolve_async() }
    //}

    //#[cfg(feature = "futures")]
    //#[cfg(any(feature = "v2018_6", feature = "dox"))]
    //fn resolve_async_future(&self, refs: &[&CollectionRef], parent_repo: &Repo) -> Box_<dyn future::Future<Output = Result</*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 2 }, Error>> + std::marker::Unpin> {
        //use gio::GioFuture;
        //use fragile::Fragile;

        //let refs = refs.clone();
        //let parent_repo = parent_repo.clone();
        //GioFuture::new(self, move |obj, send| {
        //    let cancellable = gio::Cancellable::new();
        //    let send = Fragile::new(send);
        //    obj.resolve_async(
        //        &refs,
        //        &parent_repo,
        //        Some(&cancellable),
        //        move |res| {
        //            let _ = send.into_inner().send(res);
        //        },
        //    );

        //    cancellable
        //})
    //}
}

impl fmt::Display for RepoFinder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RepoFinder")
    }
}

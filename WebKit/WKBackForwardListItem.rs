//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKBackForwardListItem")]
    pub struct WKBackForwardListItem;

    #[cfg(feature = "WebKit_WKBackForwardListItem")]
    unsafe impl ClassType for WKBackForwardListItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "WebKit_WKBackForwardListItem")]
    unsafe impl WKBackForwardListItem {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Id<NSURL, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other initialURL)]
        pub unsafe fn initialURL(&self) -> Id<NSURL, Shared>;
    }
);
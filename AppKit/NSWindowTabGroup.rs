//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSWindowTabGroup;

    unsafe impl ClassType for NSWindowTabGroup {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSWindowTabGroup")]
    unsafe impl NSWindowTabGroup {
        #[cfg(feature = "AppKit_NSWindowTabbingIdentifier")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSWindowTabbingIdentifier, Shared>;

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other windows)]
        pub unsafe fn windows(&self) -> Id<NSArray<NSWindow>, Shared>;

        #[method(isOverviewVisible)]
        pub unsafe fn isOverviewVisible(&self) -> bool;

        #[method(setOverviewVisible:)]
        pub unsafe fn setOverviewVisible(&self, overviewVisible: bool);

        #[method(isTabBarVisible)]
        pub unsafe fn isTabBarVisible(&self) -> bool;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other selectedWindow)]
        pub unsafe fn selectedWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setSelectedWindow:)]
        pub unsafe fn setSelectedWindow(&self, selectedWindow: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(addWindow:)]
        pub unsafe fn addWindow(&self, window: &NSWindow);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(insertWindow:atIndex:)]
        pub unsafe fn insertWindow_atIndex(&self, window: &NSWindow, index: NSInteger);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(removeWindow:)]
        pub unsafe fn removeWindow(&self, window: &NSWindow);
    }
);

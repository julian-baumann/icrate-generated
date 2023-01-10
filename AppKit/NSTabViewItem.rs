//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabState {
        NSSelectedTab = 0,
        NSBackgroundTab = 1,
        NSPressedTab = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTabViewItem;

    unsafe impl ClassType for NSTabViewItem {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSTabViewItem")]
    unsafe impl NSTabViewItem {
        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other tabViewItemWithViewController:)]
        pub unsafe fn tabViewItemWithViewController(
            viewController: &NSViewController,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: Option<&Object>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<Object, Shared>>;

        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&Object>);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController, Shared>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, viewController: Option<&NSViewController>);

        #[method(tabState)]
        pub unsafe fn tabState(&self) -> NSTabState;

        #[cfg(feature = "AppKit_NSTabView")]
        #[method_id(@__retain_semantics Other tabView)]
        pub unsafe fn tabView(&self) -> Option<Id<NSTabView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other initialFirstResponder)]
        pub unsafe fn initialFirstResponder(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setInitialFirstResponder:)]
        pub unsafe fn setInitialFirstResponder(&self, initialFirstResponder: Option<&NSView>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);

        #[method(drawLabel:inRect:)]
        pub unsafe fn drawLabel_inRect(&self, shouldTruncateLabel: bool, labelRect: NSRect);

        #[method(sizeOfLabel:)]
        pub unsafe fn sizeOfLabel(&self, computeMin: bool) -> NSSize;
    }
);

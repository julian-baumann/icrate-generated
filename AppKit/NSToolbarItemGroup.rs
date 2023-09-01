//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSToolbarItemGroupSelectionMode {
        NSToolbarItemGroupSelectionModeSelectOne = 0,
        NSToolbarItemGroupSelectionModeSelectAny = 1,
        NSToolbarItemGroupSelectionModeMomentary = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSToolbarItemGroupControlRepresentation {
        NSToolbarItemGroupControlRepresentationAutomatic = 0,
        NSToolbarItemGroupControlRepresentationExpanded = 1,
        NSToolbarItemGroupControlRepresentationCollapsed = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSToolbarItemGroup")]
    pub struct NSToolbarItemGroup;

    #[cfg(feature = "AppKit_NSToolbarItemGroup")]
    unsafe impl ClassType for NSToolbarItemGroup {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSToolbarItemGroup")]
unsafe impl NSCopying for NSToolbarItemGroup {}

#[cfg(feature = "AppKit_NSToolbarItemGroup")]
unsafe impl NSObjectProtocol for NSToolbarItemGroup {}

extern_methods!(
    #[cfg(feature = "AppKit_NSToolbarItemGroup")]
    unsafe impl NSToolbarItemGroup {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other groupWithItemIdentifier:titles:selectionMode:labels:target:action:)]
        pub unsafe fn groupWithItemIdentifier_titles_selectionMode_labels_target_action(
            item_identifier: &NSToolbarItemIdentifier,
            titles: &NSArray<NSString>,
            selection_mode: NSToolbarItemGroupSelectionMode,
            labels: Option<&NSArray<NSString>>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AppKit_NSImage",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other groupWithItemIdentifier:images:selectionMode:labels:target:action:)]
        pub unsafe fn groupWithItemIdentifier_images_selectionMode_labels_target_action(
            item_identifier: &NSToolbarItemIdentifier,
            images: &NSArray<NSImage>,
            selection_mode: NSToolbarItemGroupSelectionMode,
            labels: Option<&NSArray<NSString>>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
            mtm: MainThreadMarker,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other subitems)]
        pub unsafe fn subitems(&self) -> Id<NSArray<NSToolbarItem>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setSubitems:)]
        pub unsafe fn setSubitems(&self, subitems: &NSArray<NSToolbarItem>);

        #[method(controlRepresentation)]
        pub unsafe fn controlRepresentation(&self) -> NSToolbarItemGroupControlRepresentation;

        #[method(setControlRepresentation:)]
        pub unsafe fn setControlRepresentation(
            &self,
            control_representation: NSToolbarItemGroupControlRepresentation,
        );

        #[method(selectionMode)]
        pub unsafe fn selectionMode(&self) -> NSToolbarItemGroupSelectionMode;

        #[method(setSelectionMode:)]
        pub unsafe fn setSelectionMode(&self, selection_mode: NSToolbarItemGroupSelectionMode);

        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[method(setSelected:atIndex:)]
        pub unsafe fn setSelected_atIndex(&self, selected: bool, index: NSInteger);

        #[method(isSelectedAtIndex:)]
        pub unsafe fn isSelectedAtIndex(&self, index: NSInteger) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSToolbarItemGroup")]
    unsafe impl NSToolbarItemGroup {
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSToolbarItemGroup")]
    unsafe impl NSToolbarItemGroup {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

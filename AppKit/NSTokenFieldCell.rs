//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTokenStyle {
        NSTokenStyleDefault = 0,
        NSTokenStyleNone = 1,
        NSTokenStyleRounded = 2,
        NSTokenStyleSquared = 3,
        NSTokenStylePlainSquared = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTokenFieldCell")]
    pub struct NSTokenFieldCell;

    #[cfg(feature = "AppKit_NSTokenFieldCell")]
    unsafe impl ClassType for NSTokenFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

#[cfg(feature = "AppKit_NSTokenFieldCell")]
unsafe impl NSAccessibility for NSTokenFieldCell {}

#[cfg(feature = "AppKit_NSTokenFieldCell")]
unsafe impl NSAccessibilityElementProtocol for NSTokenFieldCell {}

#[cfg(feature = "AppKit_NSTokenFieldCell")]
unsafe impl NSCoding for NSTokenFieldCell {}

#[cfg(feature = "AppKit_NSTokenFieldCell")]
unsafe impl NSObjectProtocol for NSTokenFieldCell {}

#[cfg(feature = "AppKit_NSTokenFieldCell")]
unsafe impl NSUserInterfaceItemIdentification for NSTokenFieldCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTokenFieldCell")]
    unsafe impl NSTokenFieldCell {
        #[method(tokenStyle)]
        pub unsafe fn tokenStyle(&self) -> NSTokenStyle;

        #[method(setTokenStyle:)]
        pub unsafe fn setTokenStyle(&self, token_style: NSTokenStyle);

        #[method(completionDelay)]
        pub unsafe fn completionDelay(&self) -> NSTimeInterval;

        #[method(setCompletionDelay:)]
        pub unsafe fn setCompletionDelay(&self, completion_delay: NSTimeInterval);

        #[method(defaultCompletionDelay)]
        pub unsafe fn defaultCompletionDelay() -> NSTimeInterval;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other tokenizingCharacterSet)]
        pub unsafe fn tokenizingCharacterSet(&self) -> Id<NSCharacterSet, Shared>;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method(setTokenizingCharacterSet:)]
        pub unsafe fn setTokenizingCharacterSet(
            &self,
            tokenizing_character_set: Option<&NSCharacterSet>,
        );

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other defaultTokenizingCharacterSet)]
        pub unsafe fn defaultTokenizingCharacterSet() -> Id<NSCharacterSet, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTokenFieldCellDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTokenFieldCellDelegate>>,
        );
    }
);

extern_protocol!(
    pub unsafe trait NSTokenFieldCellDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSTokenFieldCell",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:completionsForSubstring:indexOfToken:indexOfSelectedItem:)]
        unsafe fn tokenFieldCell_completionsForSubstring_indexOfToken_indexOfSelectedItem(
            &self,
            token_field_cell: &NSTokenFieldCell,
            substring: &NSString,
            token_index: NSInteger,
            selected_index: NonNull<NSInteger>,
        ) -> Id<NSArray, Shared>;

        #[cfg(all(feature = "AppKit_NSTokenFieldCell", feature = "Foundation_NSArray"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:shouldAddObjects:atIndex:)]
        unsafe fn tokenFieldCell_shouldAddObjects_atIndex(
            &self,
            token_field_cell: &NSTokenFieldCell,
            tokens: &NSArray,
            index: NSUInteger,
        ) -> Id<NSArray, Shared>;

        #[cfg(all(feature = "AppKit_NSTokenFieldCell", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:displayStringForRepresentedObject:)]
        unsafe fn tokenFieldCell_displayStringForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "AppKit_NSTokenFieldCell", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:editingStringForRepresentedObject:)]
        unsafe fn tokenFieldCell_editingStringForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &Object,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(all(feature = "AppKit_NSTokenFieldCell", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:representedObjectForEditingString:)]
        unsafe fn tokenFieldCell_representedObjectForEditingString(
            &self,
            token_field_cell: &NSTokenFieldCell,
            editing_string: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSTokenFieldCell",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(tokenFieldCell:writeRepresentedObjects:toPasteboard:)]
        unsafe fn tokenFieldCell_writeRepresentedObjects_toPasteboard(
            &self,
            token_field_cell: &NSTokenFieldCell,
            objects: &NSArray,
            pboard: &NSPasteboard,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSPasteboard",
            feature = "AppKit_NSTokenFieldCell",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:readFromPasteboard:)]
        unsafe fn tokenFieldCell_readFromPasteboard(
            &self,
            token_field_cell: &NSTokenFieldCell,
            pboard: &NSPasteboard,
        ) -> Option<Id<NSArray, Shared>>;

        #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSTokenFieldCell"))]
        #[optional]
        #[method_id(@__retain_semantics Other tokenFieldCell:menuForRepresentedObject:)]
        unsafe fn tokenFieldCell_menuForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &Object,
        ) -> Option<Id<NSMenu, Shared>>;

        #[cfg(feature = "AppKit_NSTokenFieldCell")]
        #[optional]
        #[method(tokenFieldCell:hasMenuForRepresentedObject:)]
        unsafe fn tokenFieldCell_hasMenuForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &Object,
        ) -> bool;

        #[cfg(feature = "AppKit_NSTokenFieldCell")]
        #[optional]
        #[method(tokenFieldCell:styleForRepresentedObject:)]
        unsafe fn tokenFieldCell_styleForRepresentedObject(
            &self,
            token_field_cell: &NSTokenFieldCell,
            represented_object: &Object,
        ) -> NSTokenStyle;
    }

    unsafe impl ProtocolType for dyn NSTokenFieldCellDelegate {}
);

extern_static!(NSDefaultTokenStyle: NSTokenStyle = NSTokenStyleDefault);

extern_static!(NSPlainTextTokenStyle: NSTokenStyle = NSTokenStyleNone);

extern_static!(NSRoundedTokenStyle: NSTokenStyle = NSTokenStyleRounded);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSTokenFieldCell")]
    unsafe impl NSTokenFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;
    }
);

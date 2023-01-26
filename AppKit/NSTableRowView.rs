//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableRowView")]
    pub struct NSTableRowView;

    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl ClassType for NSTableRowView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibility for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibilityElementProtocol for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibilityGroup for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibilityRow for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAnimatablePropertyContainer for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAppearanceCustomization for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSCoding for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSDraggingDestination for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSObjectProtocol for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSUserInterfaceItemIdentification for NSTableRowView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl NSTableRowView {
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> NSTableViewSelectionHighlightStyle;

        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selection_highlight_style: NSTableViewSelectionHighlightStyle,
        );

        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[method(isGroupRowStyle)]
        pub unsafe fn isGroupRowStyle(&self) -> bool;

        #[method(setGroupRowStyle:)]
        pub unsafe fn setGroupRowStyle(&self, group_row_style: bool);

        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[method(isPreviousRowSelected)]
        pub unsafe fn isPreviousRowSelected(&self) -> bool;

        #[method(setPreviousRowSelected:)]
        pub unsafe fn setPreviousRowSelected(&self, previous_row_selected: bool);

        #[method(isNextRowSelected)]
        pub unsafe fn isNextRowSelected(&self) -> bool;

        #[method(setNextRowSelected:)]
        pub unsafe fn setNextRowSelected(&self, next_row_selected: bool);

        #[method(isFloating)]
        pub unsafe fn isFloating(&self) -> bool;

        #[method(setFloating:)]
        pub unsafe fn setFloating(&self, floating: bool);

        #[method(isTargetForDropOperation)]
        pub unsafe fn isTargetForDropOperation(&self) -> bool;

        #[method(setTargetForDropOperation:)]
        pub unsafe fn setTargetForDropOperation(&self, target_for_drop_operation: bool);

        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> NSTableViewDraggingDestinationFeedbackStyle;

        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            dragging_destination_feedback_style: NSTableViewDraggingDestinationFeedbackStyle,
        );

        #[method(indentationForDropOperation)]
        pub unsafe fn indentationForDropOperation(&self) -> CGFloat;

        #[method(setIndentationForDropOperation:)]
        pub unsafe fn setIndentationForDropOperation(
            &self,
            indentation_for_drop_operation: CGFloat,
        );

        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(drawBackgroundInRect:)]
        pub unsafe fn drawBackgroundInRect(&self, dirty_rect: NSRect);

        #[method(drawSelectionInRect:)]
        pub unsafe fn drawSelectionInRect(&self, dirty_rect: NSRect);

        #[method(drawSeparatorInRect:)]
        pub unsafe fn drawSeparatorInRect(&self, dirty_rect: NSRect);

        #[method(drawDraggingDestinationFeedbackInRect:)]
        pub unsafe fn drawDraggingDestinationFeedbackInRect(&self, dirty_rect: NSRect);

        #[method_id(@__retain_semantics Other viewAtColumn:)]
        pub unsafe fn viewAtColumn(&self, column: NSInteger) -> Option<Id<Object, Shared>>;

        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl NSTableRowView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;
    }
);

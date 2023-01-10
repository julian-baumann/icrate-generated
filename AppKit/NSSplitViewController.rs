//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSSplitViewControllerAutomaticDimension: CGFloat);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSSplitViewController;

    unsafe impl ClassType for NSSplitViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSSplitViewController")]
    unsafe impl NSSplitViewController {
        #[cfg(feature = "AppKit_NSSplitView")]
        #[method_id(@__retain_semantics Other splitView)]
        pub unsafe fn splitView(&self) -> Id<NSSplitView, Shared>;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[method(setSplitView:)]
        pub unsafe fn setSplitView(&self, splitView: &NSSplitView);

        #[cfg(all(feature = "AppKit_NSSplitViewItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other splitViewItems)]
        pub unsafe fn splitViewItems(&self) -> Id<NSArray<NSSplitViewItem>, Shared>;

        #[cfg(all(feature = "AppKit_NSSplitViewItem", feature = "Foundation_NSArray"))]
        #[method(setSplitViewItems:)]
        pub unsafe fn setSplitViewItems(&self, splitViewItems: &NSArray<NSSplitViewItem>);

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method(addSplitViewItem:)]
        pub unsafe fn addSplitViewItem(&self, splitViewItem: &NSSplitViewItem);

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method(insertSplitViewItem:atIndex:)]
        pub unsafe fn insertSplitViewItem_atIndex(
            &self,
            splitViewItem: &NSSplitViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method(removeSplitViewItem:)]
        pub unsafe fn removeSplitViewItem(&self, splitViewItem: &NSSplitViewItem);

        #[cfg(feature = "AppKit_NSSplitViewItem")]
        #[method_id(@__retain_semantics Other splitViewItemForViewController:)]
        pub unsafe fn splitViewItemForViewController(
            &self,
            viewController: &NSViewController,
        ) -> Option<Id<NSSplitViewItem, Shared>>;

        #[method(minimumThicknessForInlineSidebars)]
        pub unsafe fn minimumThicknessForInlineSidebars(&self) -> CGFloat;

        #[method(setMinimumThicknessForInlineSidebars:)]
        pub unsafe fn setMinimumThicknessForInlineSidebars(
            &self,
            minimumThicknessForInlineSidebars: CGFloat,
        );

        #[cfg(feature = "AppKit_NSValidatedUserInterfaceItem")]
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(&self, item: &NSValidatedUserInterfaceItem)
            -> bool;

        #[method(viewDidLoad)]
        pub unsafe fn viewDidLoad(&self);

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[method(splitView:canCollapseSubview:)]
        pub unsafe fn splitView_canCollapseSubview(
            &self,
            splitView: &NSSplitView,
            subview: &NSView,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSSplitView", feature = "AppKit_NSView"))]
        #[method(splitView:shouldCollapseSubview:forDoubleClickOnDividerAtIndex:)]
        pub unsafe fn splitView_shouldCollapseSubview_forDoubleClickOnDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            subview: &NSView,
            dividerIndex: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[method(splitView:shouldHideDividerAtIndex:)]
        pub unsafe fn splitView_shouldHideDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[method(splitView:effectiveRect:forDrawnRect:ofDividerAtIndex:)]
        pub unsafe fn splitView_effectiveRect_forDrawnRect_ofDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            proposedEffectiveRect: NSRect,
            drawnRect: NSRect,
            dividerIndex: NSInteger,
        ) -> NSRect;

        #[cfg(feature = "AppKit_NSSplitView")]
        #[method(splitView:additionalEffectiveRectOfDividerAtIndex:)]
        pub unsafe fn splitView_additionalEffectiveRectOfDividerAtIndex(
            &self,
            splitView: &NSSplitView,
            dividerIndex: NSInteger,
        ) -> NSRect;
    }
);

extern_methods!(
    /// NSSplitViewControllerToggleSidebarAction
    #[cfg(feature = "AppKit_NSSplitViewController")]
    unsafe impl NSSplitViewController {
        #[method(toggleSidebar:)]
        pub unsafe fn toggleSidebar(&self, sender: Option<&Object>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AppKit_NSSplitViewController")]
    unsafe impl NSSplitViewController {
        #[cfg(all(feature = "AppKit_NSNibName", feature = "Foundation_NSBundle"))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nibNameOrNil: Option<&NSNibName>,
            nibBundleOrNil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
    }
);

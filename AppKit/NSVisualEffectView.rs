//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSVisualEffectMaterial {
        NSVisualEffectMaterialTitlebar = 3,
        NSVisualEffectMaterialSelection = 4,
        NSVisualEffectMaterialMenu = 5,
        NSVisualEffectMaterialPopover = 6,
        NSVisualEffectMaterialSidebar = 7,
        NSVisualEffectMaterialHeaderView = 10,
        NSVisualEffectMaterialSheet = 11,
        NSVisualEffectMaterialWindowBackground = 12,
        NSVisualEffectMaterialHUDWindow = 13,
        NSVisualEffectMaterialFullScreenUI = 15,
        NSVisualEffectMaterialToolTip = 17,
        NSVisualEffectMaterialContentBackground = 18,
        NSVisualEffectMaterialUnderWindowBackground = 21,
        NSVisualEffectMaterialUnderPageBackground = 22,
        #[deprecated = "Use a specific semantic material instead."]
        NSVisualEffectMaterialAppearanceBased = 0,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        NSVisualEffectMaterialLight = 1,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        NSVisualEffectMaterialDark = 2,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        NSVisualEffectMaterialMediumLight = 8,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        NSVisualEffectMaterialUltraDark = 9,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSVisualEffectBlendingMode {
        NSVisualEffectBlendingModeBehindWindow = 0,
        NSVisualEffectBlendingModeWithinWindow = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSVisualEffectState {
        NSVisualEffectStateFollowsWindowActiveState = 0,
        NSVisualEffectStateActive = 1,
        NSVisualEffectStateInactive = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    pub struct NSVisualEffectView;

    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl ClassType for NSVisualEffectView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAccessibility for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAccessibilityElementProtocol for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAnimatablePropertyContainer for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAppearanceCustomization for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSCoding for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSDraggingDestination for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSObjectProtocol for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSUserInterfaceItemIdentification for NSVisualEffectView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl NSVisualEffectView {
        #[method(material)]
        pub unsafe fn material(&self) -> NSVisualEffectMaterial;

        #[method(setMaterial:)]
        pub unsafe fn setMaterial(&self, material: NSVisualEffectMaterial);

        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[method(blendingMode)]
        pub unsafe fn blendingMode(&self) -> NSVisualEffectBlendingMode;

        #[method(setBlendingMode:)]
        pub unsafe fn setBlendingMode(&self, blending_mode: NSVisualEffectBlendingMode);

        #[method(state)]
        pub unsafe fn state(&self) -> NSVisualEffectState;

        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSVisualEffectState);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other maskImage)]
        pub unsafe fn maskImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setMaskImage:)]
        pub unsafe fn setMaskImage(&self, mask_image: Option<&NSImage>);

        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[method(viewDidMoveToWindow)]
        pub unsafe fn viewDidMoveToWindow(&self);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(viewWillMoveToWindow:)]
        pub unsafe fn viewWillMoveToWindow(&self, new_window: Option<&NSWindow>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl NSVisualEffectView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl NSVisualEffectView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl NSVisualEffectView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

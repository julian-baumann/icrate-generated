//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "OSAKit_OSAScriptView")]
    pub struct OSAScriptView;

    #[cfg(feature = "OSAKit_OSAScriptView")]
    unsafe impl ClassType for OSAScriptView {
        #[inherits(NSText, NSView, NSResponder, NSObject)]
        type Super = NSTextView;
    }
);

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSAccessibility for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSAccessibilityElementProtocol for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSAccessibilityNavigableStaticText for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSAccessibilityStaticText for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSAnimatablePropertyContainer for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSAppearanceCustomization for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSChangeSpelling for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSCoding for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSColorChanging for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSDraggingDestination for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSDraggingSource for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSIgnoreMisspelledWords for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSMenuItemValidation for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSObjectProtocol for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSStandardKeyBindingResponding for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSTextContent for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSTextInput for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSTextInputClient for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSTextLayoutOrientationProvider for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSUserInterfaceItemIdentification for OSAScriptView {}

#[cfg(feature = "OSAKit_OSAScriptView")]
unsafe impl NSUserInterfaceValidations for OSAScriptView {}

extern_methods!(
    #[cfg(feature = "OSAKit_OSAScriptView")]
    unsafe impl OSAScriptView {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSource:)]
        pub unsafe fn setSource(&self, source: Option<&NSString>);

        #[method(usesScriptAssistant)]
        pub unsafe fn usesScriptAssistant(&self) -> bool;

        #[method(setUsesScriptAssistant:)]
        pub unsafe fn setUsesScriptAssistant(&self, uses_script_assistant: bool);

        #[method(usesTabs)]
        pub unsafe fn usesTabs(&self) -> bool;

        #[method(setUsesTabs:)]
        pub unsafe fn setUsesTabs(&self, uses_tabs: bool);

        #[method(tabWidth)]
        pub unsafe fn tabWidth(&self) -> NSUInteger;

        #[method(setTabWidth:)]
        pub unsafe fn setTabWidth(&self, tab_width: NSUInteger);

        #[method(wrapsLines)]
        pub unsafe fn wrapsLines(&self) -> bool;

        #[method(setWrapsLines:)]
        pub unsafe fn setWrapsLines(&self, wraps_lines: bool);

        #[method(indentsWrappedLines)]
        pub unsafe fn indentsWrappedLines(&self) -> bool;

        #[method(setIndentsWrappedLines:)]
        pub unsafe fn setIndentsWrappedLines(&self, indents_wrapped_lines: bool);

        #[method(indentWidth)]
        pub unsafe fn indentWidth(&self) -> NSUInteger;

        #[method(setIndentWidth:)]
        pub unsafe fn setIndentWidth(&self, indent_width: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextView`
    #[cfg(feature = "OSAKit_OSAScriptView")]
    unsafe impl OSAScriptView {
        #[cfg(feature = "AppKit_NSTextContainer")]
        #[method_id(@__retain_semantics Init initWithFrame:textContainer:)]
        pub unsafe fn initWithFrame_textContainer(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
            container: Option<&NSTextContainer>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(
            this: Option<Allocated<Self>>,
            frame_rect: NSRect,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initUsingTextLayoutManager:)]
        pub unsafe fn initUsingTextLayoutManager(
            this: Option<Allocated<Self>>,
            using_text_layout_manager: bool,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other textViewUsingTextLayoutManager:)]
        pub unsafe fn textViewUsingTextLayoutManager(
            using_text_layout_manager: bool,
        ) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextView`
    ///
    /// NSTextView_Factory
    #[cfg(feature = "OSAKit_OSAScriptView")]
    unsafe impl OSAScriptView {
        #[method_id(@__retain_semantics Other fieldEditor)]
        pub unsafe fn fieldEditor() -> Id<Self, Shared>;
    }
);

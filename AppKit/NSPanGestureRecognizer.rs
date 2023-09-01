//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPanGestureRecognizer")]
    pub struct NSPanGestureRecognizer;

    #[cfg(feature = "AppKit_NSPanGestureRecognizer")]
    unsafe impl ClassType for NSPanGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
        type Mutability = MainThreadOnly;
    }
);

#[cfg(feature = "AppKit_NSPanGestureRecognizer")]
unsafe impl NSCoding for NSPanGestureRecognizer {}

#[cfg(feature = "AppKit_NSPanGestureRecognizer")]
unsafe impl NSObjectProtocol for NSPanGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPanGestureRecognizer")]
    unsafe impl NSPanGestureRecognizer {
        #[method(buttonMask)]
        pub unsafe fn buttonMask(&self) -> NSUInteger;

        #[method(setButtonMask:)]
        pub unsafe fn setButtonMask(&self, button_mask: NSUInteger);

        #[cfg(feature = "AppKit_NSView")]
        #[method(translationInView:)]
        pub unsafe fn translationInView(&self, view: Option<&NSView>) -> NSPoint;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setTranslation:inView:)]
        pub unsafe fn setTranslation_inView(&self, translation: NSPoint, view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSView")]
        #[method(velocityInView:)]
        pub unsafe fn velocityInView(&self, view: Option<&NSView>) -> NSPoint;

        #[method(numberOfTouchesRequired)]
        pub unsafe fn numberOfTouchesRequired(&self) -> NSInteger;

        #[method(setNumberOfTouchesRequired:)]
        pub unsafe fn setNumberOfTouchesRequired(&self, number_of_touches_required: NSInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "AppKit_NSPanGestureRecognizer")]
    unsafe impl NSPanGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Option<Allocated<Self>>,
            target: Option<&AnyObject>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSPanGestureRecognizer")]
    unsafe impl NSPanGestureRecognizer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

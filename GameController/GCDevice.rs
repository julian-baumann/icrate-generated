//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    pub struct GCDevice;

    unsafe impl ProtocolType for GCDevice {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other vendorName)]
        pub unsafe fn vendorName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other productCategory)]
        pub unsafe fn productCategory(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "GameController_GCPhysicalInputProfile")]
        #[deprecated = "Use the physicalInputProfile property on GCController instead.  For GCKeyboard, use the keyboardInput property.  For GCMouse, use the mouseInput property."]
        #[method_id(@__retain_semantics Other physicalInputProfile)]
        pub unsafe fn physicalInputProfile(&self) -> Id<GCPhysicalInputProfile, Shared>;
    }
);
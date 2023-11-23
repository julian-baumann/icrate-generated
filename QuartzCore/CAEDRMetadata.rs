//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAEDRMetadata")]
    pub struct CAEDRMetadata;

    #[cfg(feature = "CoreAnimation_CAEDRMetadata")]
    unsafe impl ClassType for CAEDRMetadata {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreAnimation_CAEDRMetadata")]
unsafe impl NSCoding for CAEDRMetadata {}

#[cfg(feature = "CoreAnimation_CAEDRMetadata")]
unsafe impl NSCopying for CAEDRMetadata {}

#[cfg(feature = "CoreAnimation_CAEDRMetadata")]
unsafe impl NSObjectProtocol for CAEDRMetadata {}

#[cfg(feature = "CoreAnimation_CAEDRMetadata")]
unsafe impl NSSecureCoding for CAEDRMetadata {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CAEDRMetadata")]
    unsafe impl CAEDRMetadata {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other HDR10MetadataWithDisplayInfo:contentInfo:opticalOutputScale:)]
        pub unsafe fn HDR10MetadataWithDisplayInfo_contentInfo_opticalOutputScale(
            display_data: Option<&NSData>,
            content_data: Option<&NSData>,
            scale: c_float,
        ) -> Id<CAEDRMetadata>;

        #[method_id(@__retain_semantics Other HDR10MetadataWithMinLuminance:maxLuminance:opticalOutputScale:)]
        pub unsafe fn HDR10MetadataWithMinLuminance_maxLuminance_opticalOutputScale(
            min_nits: c_float,
            max_nits: c_float,
            scale: c_float,
        ) -> Id<CAEDRMetadata>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other HLGMetadataWithAmbientViewingEnvironment:)]
        pub unsafe fn HLGMetadataWithAmbientViewingEnvironment(data: &NSData) -> Id<CAEDRMetadata>;

        #[method_id(@__retain_semantics Other HLGMetadata)]
        pub unsafe fn HLGMetadata() -> Id<CAEDRMetadata>;

        #[method(isAvailable)]
        pub unsafe fn isAvailable() -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreAnimation_CAEDRMetadata")]
    unsafe impl CAEDRMetadata {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

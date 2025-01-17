//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXMetaData")]
    pub struct MXMetaData;

    #[cfg(feature = "MetricKit_MXMetaData")]
    unsafe impl ClassType for MXMetaData {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXMetaData")]
unsafe impl NSCoding for MXMetaData {}

#[cfg(feature = "MetricKit_MXMetaData")]
unsafe impl NSObjectProtocol for MXMetaData {}

#[cfg(feature = "MetricKit_MXMetaData")]
unsafe impl NSSecureCoding for MXMetaData {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXMetaData")]
    unsafe impl MXMetaData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other regionFormat)]
        pub unsafe fn regionFormat(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other osVersion)]
        pub unsafe fn osVersion(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other deviceType)]
        pub unsafe fn deviceType(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other applicationBuildVersion)]
        pub unsafe fn applicationBuildVersion(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other platformArchitecture)]
        pub unsafe fn platformArchitecture(&self) -> Id<NSString>;

        #[method(lowPowerModeEnabled)]
        pub unsafe fn lowPowerModeEnabled(&self) -> bool;

        #[method(isTestFlightApp)]
        pub unsafe fn isTestFlightApp(&self) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[deprecated]
        #[method_id(@__retain_semantics Other DictionaryRepresentation)]
        pub unsafe fn DictionaryRepresentation(&self) -> Id<NSDictionary>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXMetaData")]
    unsafe impl MXMetaData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

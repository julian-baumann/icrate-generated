//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreWLAN::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreWLAN_CWChannel")]
    pub struct CWChannel;

    #[cfg(feature = "CoreWLAN_CWChannel")]
    unsafe impl ClassType for CWChannel {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreWLAN_CWChannel")]
unsafe impl NSCoding for CWChannel {}

#[cfg(feature = "CoreWLAN_CWChannel")]
unsafe impl NSCopying for CWChannel {}

#[cfg(feature = "CoreWLAN_CWChannel")]
unsafe impl NSObjectProtocol for CWChannel {}

#[cfg(feature = "CoreWLAN_CWChannel")]
unsafe impl NSSecureCoding for CWChannel {}

extern_methods!(
    #[cfg(feature = "CoreWLAN_CWChannel")]
    unsafe impl CWChannel {
        #[method(channelNumber)]
        pub unsafe fn channelNumber(&self) -> NSInteger;

        #[method(channelWidth)]
        pub unsafe fn channelWidth(&self) -> CWChannelWidth;

        #[method(channelBand)]
        pub unsafe fn channelBand(&self) -> CWChannelBand;

        #[method(isEqualToChannel:)]
        pub unsafe fn isEqualToChannel(&self, channel: &CWChannel) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreWLAN_CWChannel")]
    unsafe impl CWChannel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
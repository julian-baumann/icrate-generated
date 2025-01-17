//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLFunctionOptions {
        MTLFunctionOptionNone = 0,
        MTLFunctionOptionCompileToBinary = 1 << 0,
        MTLFunctionOptionStoreFunctionInMetalScript = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLFunctionDescriptor")]
    pub struct MTLFunctionDescriptor;

    #[cfg(feature = "Metal_MTLFunctionDescriptor")]
    unsafe impl ClassType for MTLFunctionDescriptor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLFunctionDescriptor")]
unsafe impl NSCopying for MTLFunctionDescriptor {}

#[cfg(feature = "Metal_MTLFunctionDescriptor")]
unsafe impl NSObjectProtocol for MTLFunctionDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLFunctionDescriptor")]
    unsafe impl MTLFunctionDescriptor {
        #[method_id(@__retain_semantics Other functionDescriptor)]
        pub fn functionDescriptor() -> Id<MTLFunctionDescriptor>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub fn name(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other specializedName)]
        pub fn specializedName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSpecializedName:)]
        pub fn setSpecializedName(&self, specialized_name: Option<&NSString>);

        #[cfg(feature = "Metal_MTLFunctionConstantValues")]
        #[method_id(@__retain_semantics Other constantValues)]
        pub fn constantValues(&self) -> Option<Id<MTLFunctionConstantValues>>;

        #[cfg(feature = "Metal_MTLFunctionConstantValues")]
        #[method(setConstantValues:)]
        pub fn setConstantValues(&self, constant_values: Option<&MTLFunctionConstantValues>);

        #[method(options)]
        pub fn options(&self) -> MTLFunctionOptions;

        #[method(setOptions:)]
        pub fn setOptions(&self, options: MTLFunctionOptions);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other binaryArchives)]
        pub unsafe fn binaryArchives(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setBinaryArchives:)]
        pub unsafe fn setBinaryArchives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLFunctionDescriptor")]
    unsafe impl MTLFunctionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Metal_MTLFunctionDescriptor")]
impl DefaultId for MTLFunctionDescriptor {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
    pub struct MTLIntersectionFunctionDescriptor;

    #[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
    unsafe impl ClassType for MTLIntersectionFunctionDescriptor {
        #[inherits(NSObject)]
        type Super = MTLFunctionDescriptor;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
unsafe impl NSCopying for MTLIntersectionFunctionDescriptor {}

#[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
unsafe impl NSObjectProtocol for MTLIntersectionFunctionDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
    unsafe impl MTLIntersectionFunctionDescriptor {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Metal_MTLIntersectionFunctionDescriptor")]
    unsafe impl MTLIntersectionFunctionDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

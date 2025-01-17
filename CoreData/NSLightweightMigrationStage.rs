//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSLightweightMigrationStage")]
    pub struct NSLightweightMigrationStage;

    #[cfg(feature = "CoreData_NSLightweightMigrationStage")]
    unsafe impl ClassType for NSLightweightMigrationStage {
        #[inherits(NSObject)]
        type Super = NSMigrationStage;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSLightweightMigrationStage")]
unsafe impl NSObjectProtocol for NSLightweightMigrationStage {}

extern_methods!(
    #[cfg(feature = "CoreData_NSLightweightMigrationStage")]
    unsafe impl NSLightweightMigrationStage {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other versionChecksums)]
        pub unsafe fn versionChecksums(&self) -> Id<NSArray<NSString>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithVersionChecksums:)]
        pub unsafe fn initWithVersionChecksums(
            this: Allocated<Self>,
            version_checksums: &NSArray<NSString>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSLightweightMigrationStage")]
    unsafe impl NSLightweightMigrationStage {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

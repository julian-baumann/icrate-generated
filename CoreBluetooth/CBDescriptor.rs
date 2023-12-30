//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreBluetooth::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreBluetooth_CBDescriptor")]
    pub struct CBDescriptor;

    #[cfg(feature = "CoreBluetooth_CBDescriptor")]
    unsafe impl ClassType for CBDescriptor {
        #[inherits(NSObject)]
        type Super = CBAttribute;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreBluetooth_CBDescriptor")]
unsafe impl NSObjectProtocol for CBDescriptor {}

extern_methods!(
    #[cfg(feature = "CoreBluetooth_CBDescriptor")]
    unsafe impl CBDescriptor {
        #[cfg(feature = "CoreBluetooth_CBCharacteristic")]
        #[method_id(@__retain_semantics Other characteristic)]
        pub unsafe fn characteristic(&self) -> Option<Id<CBCharacteristic>>;

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<AnyObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CBAttribute`
    #[cfg(feature = "CoreBluetooth_CBDescriptor")]
    unsafe impl CBDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreBluetooth_CBDescriptor")]
    unsafe impl CBDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreBluetooth_CBMutableDescriptor")]
    pub struct CBMutableDescriptor;

    #[cfg(feature = "CoreBluetooth_CBMutableDescriptor")]
    unsafe impl ClassType for CBMutableDescriptor {
        #[inherits(CBAttribute, NSObject)]
        type Super = CBDescriptor;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreBluetooth_CBMutableDescriptor")]
unsafe impl NSObjectProtocol for CBMutableDescriptor {}

extern_methods!(
    #[cfg(feature = "CoreBluetooth_CBMutableDescriptor")]
    unsafe impl CBMutableDescriptor {
        #[cfg(feature = "CoreBluetooth_CBUUID")]
        #[method_id(@__retain_semantics Init initWithType:value:)]
        pub unsafe fn initWithType_value(
            this: Allocated<Self>,
            uuid: &CBUUID,
            value: Option<&AnyObject>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CBAttribute`
    #[cfg(feature = "CoreBluetooth_CBMutableDescriptor")]
    unsafe impl CBMutableDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreBluetooth_CBMutableDescriptor")]
    unsafe impl CBMutableDescriptor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

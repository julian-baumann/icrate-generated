//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreWLAN::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreWLAN_CWConfiguration")]
    pub struct CWConfiguration;

    #[cfg(feature = "CoreWLAN_CWConfiguration")]
    unsafe impl ClassType for CWConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreWLAN_CWConfiguration")]
unsafe impl NSCoding for CWConfiguration {}

#[cfg(feature = "CoreWLAN_CWConfiguration")]
unsafe impl NSCopying for CWConfiguration {}

#[cfg(feature = "CoreWLAN_CWConfiguration")]
unsafe impl NSMutableCopying for CWConfiguration {}

#[cfg(feature = "CoreWLAN_CWConfiguration")]
unsafe impl NSObjectProtocol for CWConfiguration {}

#[cfg(feature = "CoreWLAN_CWConfiguration")]
unsafe impl NSSecureCoding for CWConfiguration {}

extern_methods!(
    #[cfg(feature = "CoreWLAN_CWConfiguration")]
    unsafe impl CWConfiguration {
        #[cfg(all(
            feature = "CoreWLAN_CWNetworkProfile",
            feature = "Foundation_NSOrderedSet"
        ))]
        #[method_id(@__retain_semantics Other networkProfiles)]
        pub unsafe fn networkProfiles(&self) -> Id<NSOrderedSet<CWNetworkProfile>>;

        #[method(requireAdministratorForAssociation)]
        pub unsafe fn requireAdministratorForAssociation(&self) -> bool;

        #[method(requireAdministratorForPower)]
        pub unsafe fn requireAdministratorForPower(&self) -> bool;

        #[method(requireAdministratorForIBSSMode)]
        pub unsafe fn requireAdministratorForIBSSMode(&self) -> bool;

        #[method(rememberJoinedNetworks)]
        pub unsafe fn rememberJoinedNetworks(&self) -> bool;

        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CWConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationWithConfiguration:)]
        pub unsafe fn configurationWithConfiguration(configuration: &CWConfiguration) -> Id<Self>;

        #[method(isEqualToConfiguration:)]
        pub unsafe fn isEqualToConfiguration(&self, configuration: &CWConfiguration) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreWLAN_CWConfiguration")]
    unsafe impl CWConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
    pub struct CWMutableConfiguration;

    #[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
    unsafe impl ClassType for CWMutableConfiguration {
        #[inherits(NSObject)]
        type Super = CWConfiguration;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
unsafe impl NSCoding for CWMutableConfiguration {}

#[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
unsafe impl NSCopying for CWMutableConfiguration {}

#[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
unsafe impl NSMutableCopying for CWMutableConfiguration {}

#[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
unsafe impl NSObjectProtocol for CWMutableConfiguration {}

#[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
unsafe impl NSSecureCoding for CWMutableConfiguration {}

extern_methods!(
    #[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
    unsafe impl CWMutableConfiguration {
        #[cfg(all(
            feature = "CoreWLAN_CWNetworkProfile",
            feature = "Foundation_NSOrderedSet"
        ))]
        #[method_id(@__retain_semantics Other networkProfiles)]
        pub unsafe fn networkProfiles(&self) -> Id<NSOrderedSet<CWNetworkProfile>>;

        #[cfg(all(
            feature = "CoreWLAN_CWNetworkProfile",
            feature = "Foundation_NSOrderedSet"
        ))]
        #[method(setNetworkProfiles:)]
        pub unsafe fn setNetworkProfiles(&self, network_profiles: &NSOrderedSet<CWNetworkProfile>);

        #[method(requireAdministratorForAssociation)]
        pub unsafe fn requireAdministratorForAssociation(&self) -> bool;

        #[method(setRequireAdministratorForAssociation:)]
        pub unsafe fn setRequireAdministratorForAssociation(
            &self,
            require_administrator_for_association: bool,
        );

        #[method(requireAdministratorForPower)]
        pub unsafe fn requireAdministratorForPower(&self) -> bool;

        #[method(setRequireAdministratorForPower:)]
        pub unsafe fn setRequireAdministratorForPower(&self, require_administrator_for_power: bool);

        #[deprecated]
        #[method(requireAdministratorForIBSSMode)]
        pub unsafe fn requireAdministratorForIBSSMode(&self) -> bool;

        #[deprecated]
        #[method(setRequireAdministratorForIBSSMode:)]
        pub unsafe fn setRequireAdministratorForIBSSMode(
            &self,
            require_administrator_for_ibss_mode: bool,
        );

        #[method(rememberJoinedNetworks)]
        pub unsafe fn rememberJoinedNetworks(&self) -> bool;

        #[method(setRememberJoinedNetworks:)]
        pub unsafe fn setRememberJoinedNetworks(&self, remember_joined_networks: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `CWConfiguration`
    #[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
    unsafe impl CWMutableConfiguration {
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Allocated<Self>,
            configuration: &CWConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationWithConfiguration:)]
        pub unsafe fn configurationWithConfiguration(configuration: &CWConfiguration) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreWLAN_CWMutableConfiguration")]
    unsafe impl CWMutableConfiguration {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

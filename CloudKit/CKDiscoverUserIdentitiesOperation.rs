//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
    #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
    pub struct CKDiscoverUserIdentitiesOperation;

    #[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
    unsafe impl ClassType for CKDiscoverUserIdentitiesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
unsafe impl NSObjectProtocol for CKDiscoverUserIdentitiesOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
    unsafe impl CKDiscoverUserIdentitiesOperation {
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method_id(@__retain_semantics Init initWithUserIdentityLookupInfos:)]
        pub unsafe fn initWithUserIdentityLookupInfos(
            this: Allocated<Self>,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method_id(@__retain_semantics Other userIdentityLookupInfos)]
        pub unsafe fn userIdentityLookupInfos(&self) -> Id<NSArray<CKUserIdentityLookupInfo>>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setUserIdentityLookupInfos:)]
        pub unsafe fn setUserIdentityLookupInfos(
            &self,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "CloudKit_CKUserIdentityLookupInfo"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(userIdentityDiscoveredBlock)]
        pub unsafe fn userIdentityDiscoveredBlock(
            &self,
        ) -> *mut Block<(NonNull<CKUserIdentity>, NonNull<CKUserIdentityLookupInfo>), ()>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "CloudKit_CKUserIdentityLookupInfo"
        ))]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setUserIdentityDiscoveredBlock:)]
        pub unsafe fn setUserIdentityDiscoveredBlock(
            &self,
            user_identity_discovered_block: Option<
                &Block<(NonNull<CKUserIdentity>, NonNull<CKUserIdentityLookupInfo>), ()>,
            >,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(discoverUserIdentitiesCompletionBlock)]
        pub unsafe fn discoverUserIdentitiesCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSError,), ()>;

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "No longer supported. Please see Sharing CloudKit Data with Other iCloud Users."]
        #[method(setDiscoverUserIdentitiesCompletionBlock:)]
        pub unsafe fn setDiscoverUserIdentitiesCompletionBlock(
            &self,
            discover_user_identities_completion_block: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKDiscoverUserIdentitiesOperation")]
    unsafe impl CKDiscoverUserIdentitiesOperation {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

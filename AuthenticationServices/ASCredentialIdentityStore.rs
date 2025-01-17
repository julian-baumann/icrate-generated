//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_static!(ASCredentialIdentityStoreErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum ASCredentialIdentityStoreErrorCode {
        ASCredentialIdentityStoreErrorCodeInternalError = 0,
        ASCredentialIdentityStoreErrorCodeStoreDisabled = 1,
        ASCredentialIdentityStoreErrorCodeStoreBusy = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    pub struct ASCredentialIdentityStore;

    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    unsafe impl ClassType for ASCredentialIdentityStore {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
unsafe impl NSObjectProtocol for ASCredentialIdentityStore {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    unsafe impl ASCredentialIdentityStore {
        #[method_id(@__retain_semantics Other sharedStore)]
        pub unsafe fn sharedStore() -> Id<ASCredentialIdentityStore>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
        #[method(getCredentialIdentityStoreStateWithCompletion:)]
        pub unsafe fn getCredentialIdentityStoreStateWithCompletion(
            &self,
            completion: &Block<(NonNull<ASCredentialIdentityStoreState>,), ()>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[deprecated]
        #[method(saveCredentialIdentities:completion:)]
        pub unsafe fn saveCredentialIdentities_completion(
            &self,
            credential_identities: &NSArray<ASPasswordCredentialIdentity>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(saveCredentialIdentityEntries:completion:)]
        pub unsafe fn saveCredentialIdentityEntries_completion(
            &self,
            credential_identities: &NSArray<ProtocolObject<dyn ASCredentialIdentity>>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[deprecated]
        #[method(removeCredentialIdentities:completion:)]
        pub unsafe fn removeCredentialIdentities_completion(
            &self,
            credential_identities: &NSArray<ASPasswordCredentialIdentity>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(removeCredentialIdentityEntries:completion:)]
        pub unsafe fn removeCredentialIdentityEntries_completion(
            &self,
            credential_identities: &NSArray<ProtocolObject<dyn ASCredentialIdentity>>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(removeAllCredentialIdentitiesWithCompletion:)]
        pub unsafe fn removeAllCredentialIdentitiesWithCompletion(
            &self,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[deprecated]
        #[method(replaceCredentialIdentitiesWithIdentities:completion:)]
        pub unsafe fn replaceCredentialIdentitiesWithIdentities_completion(
            &self,
            new_credential_identities: &NSArray<ASPasswordCredentialIdentity>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(replaceCredentialIdentityEntries:completion:)]
        pub unsafe fn replaceCredentialIdentityEntries_completion(
            &self,
            new_credential_identities: &NSArray<ProtocolObject<dyn ASCredentialIdentity>>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    unsafe impl ASCredentialIdentityStore {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
    pub struct ASPasskeyAssertionCredential;

    #[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
    unsafe impl ClassType for ASPasskeyAssertionCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
unsafe impl ASAuthorizationCredential for ASPasskeyAssertionCredential {}

#[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
unsafe impl NSCoding for ASPasskeyAssertionCredential {}

#[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
unsafe impl NSCopying for ASPasskeyAssertionCredential {}

#[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
unsafe impl NSObjectProtocol for ASPasskeyAssertionCredential {}

#[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
unsafe impl NSSecureCoding for ASPasskeyAssertionCredential {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
    unsafe impl ASPasskeyAssertionCredential {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithUserHandle:relyingParty:signature:clientDataHash:authenticatorData:credentialID:)]
        pub unsafe fn initWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID(
            this: Allocated<Self>,
            user_handle: &NSData,
            relying_party: &NSString,
            signature: &NSData,
            client_data_hash: &NSData,
            authenticator_data: &NSData,
            credential_id: &NSData,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other credentialWithUserHandle:relyingParty:signature:clientDataHash:authenticatorData:credentialID:)]
        pub unsafe fn credentialWithUserHandle_relyingParty_signature_clientDataHash_authenticatorData_credentialID(
            user_handle: &NSData,
            relying_party: &NSString,
            signature: &NSData,
            client_data_hash: &NSData,
            authenticator_data: &NSData,
            credential_id: &NSData,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other userHandle)]
        pub unsafe fn userHandle(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other relyingParty)]
        pub unsafe fn relyingParty(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other signature)]
        pub unsafe fn signature(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other clientDataHash)]
        pub unsafe fn clientDataHash(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other authenticatorData)]
        pub unsafe fn authenticatorData(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other credentialID)]
        pub unsafe fn credentialID(&self) -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASPasskeyAssertionCredential")]
    unsafe impl ASPasskeyAssertionCredential {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

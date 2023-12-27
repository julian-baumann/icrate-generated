//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASPublicKeyCredentialClientDataCrossOriginValue {
        ASPublicKeyCredentialClientDataCrossOriginValueNotSet = 0,
        ASPublicKeyCredentialClientDataCrossOriginValueCrossOrigin = 1,
        ASPublicKeyCredentialClientDataCrossOriginValueSameOriginWithAncestors = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASPublicKeyCredentialClientData")]
    pub struct ASPublicKeyCredentialClientData;

    #[cfg(feature = "AuthenticationServices_ASPublicKeyCredentialClientData")]
    unsafe impl ClassType for ASPublicKeyCredentialClientData {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASPublicKeyCredentialClientData")]
unsafe impl NSObjectProtocol for ASPublicKeyCredentialClientData {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASPublicKeyCredentialClientData")]
    unsafe impl ASPublicKeyCredentialClientData {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithChallenge:origin:)]
        pub unsafe fn initWithChallenge_origin(
            this: Allocated<Self>,
            challenge: &NSData,
            origin: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other challenge)]
        pub unsafe fn challenge(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setChallenge:)]
        pub unsafe fn setChallenge(&self, challenge: &NSData);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other origin)]
        pub unsafe fn origin(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setOrigin:)]
        pub unsafe fn setOrigin(&self, origin: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other topOrigin)]
        pub unsafe fn topOrigin(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTopOrigin:)]
        pub unsafe fn setTopOrigin(&self, top_origin: Option<&NSString>);

        #[method(crossOrigin)]
        pub unsafe fn crossOrigin(&self) -> ASPublicKeyCredentialClientDataCrossOriginValue;

        #[method(setCrossOrigin:)]
        pub unsafe fn setCrossOrigin(
            &self,
            cross_origin: ASPublicKeyCredentialClientDataCrossOriginValue,
        );
    }
);
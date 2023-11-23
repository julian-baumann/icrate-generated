//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum ASCredentialRequestType {
        ASCredentialRequestTypePassword = 0,
        ASCredentialRequestTypePasskeyAssertion = 1,
    }
);

extern_protocol!(
    pub unsafe trait ASCredentialRequest:
        NSCopying + NSObjectProtocol + NSSecureCoding
    {
        #[method(type)]
        unsafe fn r#type(&self) -> ASCredentialRequestType;

        #[method_id(@__retain_semantics Other credentialIdentity)]
        unsafe fn credentialIdentity(&self) -> Id<ProtocolObject<dyn ASCredentialIdentity>>;
    }

    unsafe impl ProtocolType for dyn ASCredentialRequest {}
);

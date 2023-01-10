//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type ASAuthorizationProviderAuthorizationOperation = NSString;
);

extern_static!(
    ASAuthorizationProviderAuthorizationOperationConfigurationRemoved:
        &'static ASAuthorizationProviderAuthorizationOperation
);

extern_protocol!(
    pub struct ASAuthorizationProviderExtensionAuthorizationRequestHandler;

    unsafe impl ProtocolType for ASAuthorizationProviderExtensionAuthorizationRequestHandler {
        #[method(beginAuthorizationWithRequest:)]
        pub unsafe fn beginAuthorizationWithRequest(
            &self,
            request: &ASAuthorizationProviderExtensionAuthorizationRequest,
        );

        #[optional]
        #[method(cancelAuthorizationWithRequest:)]
        pub unsafe fn cancelAuthorizationWithRequest(
            &self,
            request: &ASAuthorizationProviderExtensionAuthorizationRequest,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAuthorizationProviderExtensionAuthorizationRequest;

    unsafe impl ClassType for ASAuthorizationProviderExtensionAuthorizationRequest {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest")]
    unsafe impl ASAuthorizationProviderExtensionAuthorizationRequest {
        #[method(doNotHandle)]
        pub unsafe fn doNotHandle(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(complete)]
        pub unsafe fn complete(&self);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(completeWithHTTPAuthorizationHeaders:)]
        pub unsafe fn completeWithHTTPAuthorizationHeaders(
            &self,
            httpAuthorizationHeaders: &NSDictionary<NSString, NSString>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSHTTPURLResponse"
        ))]
        #[method(completeWithHTTPResponse:httpBody:)]
        pub unsafe fn completeWithHTTPResponse_httpBody(
            &self,
            httpResponse: &NSHTTPURLResponse,
            httpBody: Option<&NSData>,
        );

        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult"
        )]
        #[method(completeWithAuthorizationResult:)]
        pub unsafe fn completeWithAuthorizationResult(
            &self,
            authorizationResult: &ASAuthorizationProviderExtensionAuthorizationResult,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(completeWithError:)]
        pub unsafe fn completeWithError(&self, error: &NSError);

        #[cfg(feature = "Foundation_NSError")]
        #[method(presentAuthorizationViewControllerWithCompletion:)]
        pub unsafe fn presentAuthorizationViewControllerWithCompletion(
            &self,
            completion: &Block<(Bool, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Id<NSURL, Shared>;

        #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderAuthorizationOperation")]
        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(
            &self,
        ) -> Id<ASAuthorizationProviderAuthorizationOperation, Shared>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other httpHeaders)]
        pub unsafe fn httpHeaders(&self) -> Id<NSDictionary<NSString, NSString>, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other httpBody)]
        pub unsafe fn httpBody(&self) -> Id<NSData, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other realm)]
        pub unsafe fn realm(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other extensionData)]
        pub unsafe fn extensionData(&self) -> Id<NSDictionary, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other callerBundleIdentifier)]
        pub unsafe fn callerBundleIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other authorizationOptions)]
        pub unsafe fn authorizationOptions(&self) -> Id<NSDictionary, Shared>;

        #[method(isCallerManaged)]
        pub unsafe fn isCallerManaged(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other callerTeamIdentifier)]
        pub unsafe fn callerTeamIdentifier(&self) -> Id<NSString, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedCallerDisplayName)]
        pub unsafe fn localizedCallerDisplayName(&self) -> Id<NSString, Shared>;

        #[method(isUserInterfaceEnabled)]
        pub unsafe fn isUserInterfaceEnabled(&self) -> bool;
    }
);

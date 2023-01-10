//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct ASAccountAuthenticationModificationExtensionContext;

    unsafe impl ClassType for ASAccountAuthenticationModificationExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationExtensionContext")]
    unsafe impl ASAccountAuthenticationModificationExtensionContext {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationAppleIDCredential",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(getSignInWithAppleUpgradeAuthorizationWithState:nonce:completionHandler:)]
        pub unsafe fn getSignInWithAppleUpgradeAuthorizationWithState_nonce_completionHandler(
            &self,
            state: Option<&NSString>,
            nonce: Option<&NSString>,
            completionHandler: &Block<(*mut ASAuthorizationAppleIDCredential, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(completeUpgradeToSignInWithAppleWithUserInfo:)]
        pub unsafe fn completeUpgradeToSignInWithAppleWithUserInfo(
            &self,
            userInfo: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredential",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(completeChangePasswordRequestWithUpdatedCredential:userInfo:)]
        pub unsafe fn completeChangePasswordRequestWithUpdatedCredential_userInfo(
            &self,
            updatedCredential: &ASPasswordCredential,
            userInfo: Option<&NSDictionary>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
    }
);

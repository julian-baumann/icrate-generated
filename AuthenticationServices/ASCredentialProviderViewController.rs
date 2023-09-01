//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
unsafe impl NSCoding for ASCredentialProviderViewController {}

#[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
unsafe impl NSEditor for ASCredentialProviderViewController {}

#[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
unsafe impl NSObjectProtocol for ASCredentialProviderViewController {}

#[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
unsafe impl NSSeguePerforming for ASCredentialProviderViewController {}

#[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
unsafe impl NSUserInterfaceItemIdentification for ASCredentialProviderViewController {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    unsafe impl ASCredentialProviderViewController {
        #[cfg(feature = "AuthenticationServices_ASCredentialProviderExtensionContext")]
        #[method_id(@__retain_semantics Other extensionContext)]
        pub unsafe fn extensionContext(&self) -> Id<ASCredentialProviderExtensionContext>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSArray"
        ))]
        #[method(prepareCredentialListForServiceIdentifiers:)]
        pub unsafe fn prepareCredentialListForServiceIdentifiers(
            &self,
            service_identifiers: &NSArray<ASCredentialServiceIdentifier>,
        );

        #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
        #[method(provideCredentialWithoutUserInteractionForIdentity:)]
        pub unsafe fn provideCredentialWithoutUserInteractionForIdentity(
            &self,
            credential_identity: &ASPasswordCredentialIdentity,
        );

        #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
        #[method(prepareInterfaceToProvideCredentialForIdentity:)]
        pub unsafe fn prepareInterfaceToProvideCredentialForIdentity(
            &self,
            credential_identity: &ASPasswordCredentialIdentity,
        );

        #[method(prepareInterfaceForExtensionConfiguration)]
        pub unsafe fn prepareInterfaceForExtensionConfiguration(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    unsafe impl ASCredentialProviderViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASCredentialProviderViewController")]
    unsafe impl ASCredentialProviderViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

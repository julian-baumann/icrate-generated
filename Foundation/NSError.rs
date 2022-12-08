//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSErrorDomain = NSString;

extern_static!(NSCocoaErrorDomain: &'static NSErrorDomain);

extern_static!(NSPOSIXErrorDomain: &'static NSErrorDomain);

extern_static!(NSOSStatusErrorDomain: &'static NSErrorDomain);

extern_static!(NSMachErrorDomain: &'static NSErrorDomain);

pub type NSErrorUserInfoKey = NSString;

extern_static!(NSUnderlyingErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSMultipleUnderlyingErrorsKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedDescriptionKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedFailureReasonErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedRecoverySuggestionErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedRecoveryOptionsErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSRecoveryAttempterErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSHelpAnchorErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSDebugDescriptionErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSLocalizedFailureErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSStringEncodingErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSURLErrorKey: &'static NSErrorUserInfoKey);

extern_static!(NSFilePathErrorKey: &'static NSErrorUserInfoKey);

extern_class!(
    #[derive(Debug)]
    pub struct NSError;

    unsafe impl ClassType for NSError {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSError {
        #[method_id(@__retain_semantics Init initWithDomain:code:userInfo:)]
        pub unsafe fn initWithDomain_code_userInfo(
            this: Option<Allocated<Self>>,
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other errorWithDomain:code:userInfo:)]
        pub unsafe fn errorWithDomain_code_userInfo(
            domain: &NSErrorDomain,
            code: NSInteger,
            dict: Option<&NSDictionary<NSErrorUserInfoKey, Object>>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other domain)]
        pub unsafe fn domain(&self) -> Id<NSErrorDomain, Shared>;

        #[method(code)]
        pub unsafe fn code(&self) -> NSInteger;

        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary<NSErrorUserInfoKey, Object>, Shared>;

        #[method_id(@__retain_semantics Other localizedDescription)]
        pub unsafe fn localizedDescription(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other localizedFailureReason)]
        pub unsafe fn localizedFailureReason(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedRecoverySuggestion)]
        pub unsafe fn localizedRecoverySuggestion(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other localizedRecoveryOptions)]
        pub unsafe fn localizedRecoveryOptions(&self) -> Option<Id<NSArray<NSString>, Shared>>;

        #[method_id(@__retain_semantics Other recoveryAttempter)]
        pub unsafe fn recoveryAttempter(&self) -> Option<Id<Object, Shared>>;

        #[method_id(@__retain_semantics Other helpAnchor)]
        pub unsafe fn helpAnchor(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other underlyingErrors)]
        pub unsafe fn underlyingErrors(&self) -> Id<NSArray<NSError>, Shared>;

        #[method(setUserInfoValueProviderForDomain:provider:)]
        pub unsafe fn setUserInfoValueProviderForDomain_provider(
            errorDomain: &NSErrorDomain,
            provider: Option<&Block<(NonNull<NSError>, NonNull<NSErrorUserInfoKey>), *mut Object>>,
        );

        #[method(userInfoValueProviderForDomain:)]
        pub unsafe fn userInfoValueProviderForDomain(
            errorDomain: &NSErrorDomain,
        ) -> *mut Block<(NonNull<NSError>, NonNull<NSErrorUserInfoKey>), *mut Object>;
    }
);

extern_methods!(
    /// NSErrorRecoveryAttempting
    unsafe impl NSObject {
        #[method(attemptRecoveryFromError:optionIndex:delegate:didRecoverSelector:contextInfo:)]
        pub unsafe fn attemptRecoveryFromError_optionIndex_delegate_didRecoverSelector_contextInfo(
            &self,
            error: &NSError,
            recoveryOptionIndex: NSUInteger,
            delegate: Option<&Object>,
            didRecoverSelector: OptionSel,
            contextInfo: *mut c_void,
        );

        #[method(attemptRecoveryFromError:optionIndex:)]
        pub unsafe fn attemptRecoveryFromError_optionIndex(
            &self,
            error: &NSError,
            recoveryOptionIndex: NSUInteger,
        ) -> bool;
    }
);
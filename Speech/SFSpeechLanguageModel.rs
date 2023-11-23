//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
    pub struct SFSpeechLanguageModelConfiguration;

    #[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
    unsafe impl ClassType for SFSpeechLanguageModelConfiguration {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
unsafe impl Send for SFSpeechLanguageModelConfiguration {}

#[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
unsafe impl Sync for SFSpeechLanguageModelConfiguration {}

#[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
unsafe impl NSCopying for SFSpeechLanguageModelConfiguration {}

#[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
unsafe impl NSObjectProtocol for SFSpeechLanguageModelConfiguration {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
    unsafe impl SFSpeechLanguageModelConfiguration {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other languageModel)]
        pub unsafe fn languageModel(&self) -> Id<NSURL>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other vocabulary)]
        pub unsafe fn vocabulary(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithLanguageModel:)]
        pub unsafe fn initWithLanguageModel(
            this: Allocated<Self>,
            language_model: &NSURL,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithLanguageModel:vocabulary:)]
        pub unsafe fn initWithLanguageModel_vocabulary(
            this: Allocated<Self>,
            language_model: &NSURL,
            vocabulary: Option<&NSURL>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech_SFSpeechLanguageModelConfiguration")]
    unsafe impl SFSpeechLanguageModelConfiguration {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechLanguageModel")]
    pub struct SFSpeechLanguageModel;

    #[cfg(feature = "Speech_SFSpeechLanguageModel")]
    unsafe impl ClassType for SFSpeechLanguageModel {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech_SFSpeechLanguageModel")]
unsafe impl NSObjectProtocol for SFSpeechLanguageModel {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechLanguageModel")]
    unsafe impl SFSpeechLanguageModel {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "Speech_SFSpeechLanguageModelConfiguration"
        ))]
        #[method(prepareCustomLanguageModelForUrl:clientIdentifier:configuration:completion:)]
        pub unsafe fn prepareCustomLanguageModelForUrl_clientIdentifier_configuration_completion(
            asset: &NSURL,
            client_identifier: &NSString,
            configuration: &SFSpeechLanguageModelConfiguration,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL",
            feature = "Speech_SFSpeechLanguageModelConfiguration"
        ))]
        #[method(prepareCustomLanguageModelForUrl:clientIdentifier:configuration:ignoresCache:completion:)]
        pub unsafe fn prepareCustomLanguageModelForUrl_clientIdentifier_configuration_ignoresCache_completion(
            asset: &NSURL,
            client_identifier: &NSString,
            configuration: &SFSpeechLanguageModelConfiguration,
            ignores_cache: bool,
            completion: &Block<(*mut NSError,), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech_SFSpeechLanguageModel")]
    unsafe impl SFSpeechLanguageModel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSItemProviderRepresentationVisibility {
        NSItemProviderRepresentationVisibilityAll = 0,
        NSItemProviderRepresentationVisibilityTeam = 1,
        NSItemProviderRepresentationVisibilityGroup = 2,
        NSItemProviderRepresentationVisibilityOwnProcess = 3,
    }
);

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSItemProviderFileOptions {
        NSItemProviderFileOptionOpenInPlace = 1,
    }
);

extern_protocol!(
    pub struct NSItemProviderWriting;

    unsafe impl ProtocolType for NSItemProviderWriting {
        #[method_id(@__retain_semantics Other loadDataWithTypeIdentifier:forItemProviderCompletionHandler:)]
        pub unsafe fn loadDataWithTypeIdentifier_forItemProviderCompletionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: &Block<(*mut NSData, *mut NSError), ()>,
        ) -> Option<Id<NSProgress, Shared>>;
    }
);

extern_protocol!(
    pub struct NSItemProviderReading;

    unsafe impl ProtocolType for NSItemProviderReading {}
);

pub type NSItemProviderCompletionHandler = *mut Block<(*mut NSSecureCoding, *mut NSError), ()>;

pub type NSItemProviderLoadHandler = *mut Block<
    (
        NSItemProviderCompletionHandler,
        *const Class,
        *mut NSDictionary,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSItemProvider;

    unsafe impl ClassType for NSItemProvider {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString"
        ))]
        #[method(registerDataRepresentationForTypeIdentifier:visibility:loadHandler:)]
        pub unsafe fn registerDataRepresentationForTypeIdentifier_visibility_loadHandler(
            &self,
            typeIdentifier: &NSString,
            visibility: NSItemProviderRepresentationVisibility,
            loadHandler: &Block<
                (NonNull<Block<(*mut NSData, *mut NSError), ()>>,),
                *mut NSProgress,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(registerFileRepresentationForTypeIdentifier:fileOptions:visibility:loadHandler:)]
        pub unsafe fn registerFileRepresentationForTypeIdentifier_fileOptions_visibility_loadHandler(
            &self,
            typeIdentifier: &NSString,
            fileOptions: NSItemProviderFileOptions,
            visibility: NSItemProviderRepresentationVisibility,
            loadHandler: &Block<
                (NonNull<Block<(*mut NSURL, Bool, *mut NSError), ()>>,),
                *mut NSProgress,
            >,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other registeredTypeIdentifiers)]
        pub unsafe fn registeredTypeIdentifiers(&self) -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other registeredTypeIdentifiersWithFileOptions:)]
        pub unsafe fn registeredTypeIdentifiersWithFileOptions(
            &self,
            fileOptions: NSItemProviderFileOptions,
        ) -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasItemConformingToTypeIdentifier:)]
        pub unsafe fn hasItemConformingToTypeIdentifier(&self, typeIdentifier: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasRepresentationConformingToTypeIdentifier:fileOptions:)]
        pub unsafe fn hasRepresentationConformingToTypeIdentifier_fileOptions(
            &self,
            typeIdentifier: &NSString,
            fileOptions: NSItemProviderFileOptions,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other loadDataRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadDataRepresentationForTypeIdentifier_completionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: &Block<(*mut NSData, *mut NSError), ()>,
        ) -> Id<NSProgress, Shared>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other loadFileRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadFileRepresentationForTypeIdentifier_completionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: &Block<(*mut NSURL, *mut NSError), ()>,
        ) -> Id<NSProgress, Shared>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other loadInPlaceFileRepresentationForTypeIdentifier:completionHandler:)]
        pub unsafe fn loadInPlaceFileRepresentationForTypeIdentifier_completionHandler(
            &self,
            typeIdentifier: &NSString,
            completionHandler: &Block<(*mut NSURL, Bool, *mut NSError), ()>,
        ) -> Id<NSProgress, Shared>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other suggestedName)]
        pub unsafe fn suggestedName(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSuggestedName:)]
        pub unsafe fn setSuggestedName(&self, suggestedName: Option<&NSString>);

        #[cfg(feature = "Foundation_NSItemProviderWriting")]
        #[method_id(@__retain_semantics Init initWithObject:)]
        pub unsafe fn initWithObject(
            this: Option<Allocated<Self>>,
            object: &NSItemProviderWriting,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSItemProviderWriting")]
        #[method(registerObject:visibility:)]
        pub unsafe fn registerObject_visibility(
            &self,
            object: &NSItemProviderWriting,
            visibility: NSItemProviderRepresentationVisibility,
        );

        #[cfg(all(feature = "Foundation_NSSecureCoding", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithItem:typeIdentifier:)]
        pub unsafe fn initWithItem_typeIdentifier(
            this: Option<Allocated<Self>>,
            item: Option<&NSSecureCoding>,
            typeIdentifier: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            fileURL: Option<&NSURL>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(registerItemForTypeIdentifier:loadHandler:)]
        pub unsafe fn registerItemForTypeIdentifier_loadHandler(
            &self,
            typeIdentifier: &NSString,
            loadHandler: NSItemProviderLoadHandler,
        );

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(loadItemForTypeIdentifier:options:completionHandler:)]
        pub unsafe fn loadItemForTypeIdentifier_options_completionHandler(
            &self,
            typeIdentifier: &NSString,
            options: Option<&NSDictionary>,
            completionHandler: NSItemProviderCompletionHandler,
        );
    }
);

extern_static!(NSItemProviderPreferredImageSizeKey: &'static NSString);

extern_methods!(
    /// NSPreviewSupport
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {
        #[method(previewImageHandler)]
        pub unsafe fn previewImageHandler(&self) -> NSItemProviderLoadHandler;

        #[method(setPreviewImageHandler:)]
        pub unsafe fn setPreviewImageHandler(&self, previewImageHandler: NSItemProviderLoadHandler);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(loadPreviewImageWithOptions:completionHandler:)]
        pub unsafe fn loadPreviewImageWithOptions_completionHandler(
            &self,
            options: Option<&NSDictionary>,
            completionHandler: NSItemProviderCompletionHandler,
        );
    }
);

extern_static!(NSExtensionJavaScriptPreprocessingResultsKey: Option<&'static NSString>);

extern_static!(NSExtensionJavaScriptFinalizeArgumentKey: Option<&'static NSString>);

extern_static!(NSItemProviderErrorDomain: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSItemProviderErrorCode {
        NSItemProviderUnknownError = -1,
        NSItemProviderItemUnavailableError = -1000,
        NSItemProviderUnexpectedValueClassError = -1100,
        NSItemProviderUnavailableCoercionError = -1200,
    }
);

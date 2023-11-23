//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    pub struct GKAchievementDescription;

    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl ClassType for GKAchievementDescription {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKAchievementDescription")]
unsafe impl NSCoding for GKAchievementDescription {}

#[cfg(feature = "GameKit_GKAchievementDescription")]
unsafe impl NSObjectProtocol for GKAchievementDescription {}

#[cfg(feature = "GameKit_GKAchievementDescription")]
unsafe impl NSSecureCoding for GKAchievementDescription {}

extern_methods!(
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl GKAchievementDescription {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadAchievementDescriptionsWithCompletionHandler:)]
        pub unsafe fn loadAchievementDescriptionsWithCompletionHandler(
            completion_handler: Option<
                &Block<(*mut NSArray<GKAchievementDescription>, *mut NSError), ()>,
            >,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other achievedDescription)]
        pub unsafe fn achievedDescription(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other unachievedDescription)]
        pub unsafe fn unachievedDescription(&self) -> Option<Id<NSString>>;

        #[method(maximumPoints)]
        pub unsafe fn maximumPoints(&self) -> NSInteger;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(isReplayable)]
        pub unsafe fn isReplayable(&self) -> bool;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other rarityPercent)]
        pub unsafe fn rarityPercent(&self) -> Option<Id<NSNumber>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl GKAchievementDescription {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKAchievementDescription")]
    unsafe impl GKAchievementDescription {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSImage, *mut NSError), ()>>,
        );

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other incompleteAchievementImage)]
        pub unsafe fn incompleteAchievementImage() -> Id<NSImage>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other placeholderCompletedAchievementImage)]
        pub unsafe fn placeholderCompletedAchievementImage() -> Id<NSImage>;
    }
);

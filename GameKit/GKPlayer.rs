//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_static!(GKPlayerIDNoLongerAvailable: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKPlayer")]
    pub struct GKPlayer;

    #[cfg(feature = "GameKit_GKPlayer")]
    unsafe impl ClassType for GKPlayer {
        #[inherits(NSObject)]
        type Super = GKBasePlayer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKPlayer")]
unsafe impl NSObjectProtocol for GKPlayer {}

extern_methods!(
    #[cfg(feature = "GameKit_GKPlayer")]
    unsafe impl GKPlayer {
        #[method(scopedIDsArePersistent)]
        pub unsafe fn scopedIDsArePersistent(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other gamePlayerID)]
        pub unsafe fn gamePlayerID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other teamPlayerID)]
        pub unsafe fn teamPlayerID(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other alias)]
        pub unsafe fn alias(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other anonymousGuestPlayerWithIdentifier:)]
        pub unsafe fn anonymousGuestPlayerWithIdentifier(guest_identifier: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other guestIdentifier)]
        pub unsafe fn guestIdentifier(&self) -> Option<Id<NSString>>;

        #[method(isInvitable)]
        pub unsafe fn isInvitable(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKPlayer")]
    unsafe impl GKPlayer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKPhotoSize {
        GKPhotoSizeSmall = 0,
        GKPhotoSizeNormal = 1,
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKPlayer")]
    unsafe impl GKPlayer {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadPhotoForSize:withCompletionHandler:)]
        pub unsafe fn loadPhotoForSize_withCompletionHandler(
            &self,
            size: GKPhotoSize,
            completion_handler: Option<&Block<(*mut NSImage, *mut NSError), ()>>,
        );
    }
);

extern_static!(GKPlayerDidChangeNotificationName: &'static NSNotificationName);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKPlayer")]
    unsafe impl GKPlayer {
        #[deprecated]
        #[method(isFriend)]
        pub unsafe fn isFriend(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use either the gamePlayerID or teamPlayerID property to identify a player."]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated]
        #[method(loadPlayersForIdentifiers:withCompletionHandler:)]
        pub unsafe fn loadPlayersForIdentifiers_withCompletionHandler(
            identifiers: &NSArray<NSString>,
            completion_handler: Option<&Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>>,
        );
    }
);

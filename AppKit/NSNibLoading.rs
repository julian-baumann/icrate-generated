//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_methods!(
    /// NSNibLoading
    unsafe impl NSBundle {
        #[method(loadNibNamed:owner:topLevelObjects:)]
        pub unsafe fn loadNibNamed_owner_topLevelObjects(
            &self,
            nibName: &NSNibName,
            owner: Option<&Object>,
            topLevelObjects: *mut *mut NSArray,
        ) -> bool;
    }
);

extern_methods!(
    /// NSNibAwaking
    unsafe impl NSObject {
        #[method(awakeFromNib)]
        pub unsafe fn awakeFromNib(&self);

        #[method(prepareForInterfaceBuilder)]
        pub unsafe fn prepareForInterfaceBuilder(&self);
    }
);

extern_methods!(
    /// NSNibLoadingDeprecated
    unsafe impl NSBundle {
        #[method(loadNibNamed:owner:)]
        pub unsafe fn loadNibNamed_owner(
            nibName: Option<&NSString>,
            owner: Option<&Object>,
        ) -> bool;
    }
);
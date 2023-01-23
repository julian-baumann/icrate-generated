//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::ExtensionKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    pub struct EXHostViewController;

    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    unsafe impl ClassType for EXHostViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

extern_methods!(
    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    unsafe impl EXHostViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<EXHostViewControllerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&EXHostViewControllerDelegate>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other placeholderView)]
        pub unsafe fn placeholderView(&self) -> Id<NSView, Shared>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setPlaceholderView:)]
        pub unsafe fn setPlaceholderView(&self, placeholder_view: &NSView);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSXPCConnection"))]
        #[method_id(@__retain_semantics Other makeXPCConnectionWithError:_)]
        pub unsafe fn makeXPCConnectionWithError(
            &self,
        ) -> Result<Id<NSXPCConnection, Shared>, Id<NSError, Shared>>;
    }
);

extern_protocol!(
    pub struct EXHostViewControllerDelegate;

    unsafe impl ProtocolType for EXHostViewControllerDelegate {
        #[cfg(feature = "ExtensionKit_EXHostViewController")]
        #[optional]
        #[method(hostViewControllerDidActivate:)]
        pub unsafe fn hostViewControllerDidActivate(&self, view_controller: &EXHostViewController);

        #[cfg(all(
            feature = "ExtensionKit_EXHostViewController",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(hostViewControllerWillDeactivate:error:)]
        pub unsafe fn hostViewControllerWillDeactivate_error(
            &self,
            view_controller: &EXHostViewController,
            error: Option<&NSError>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    unsafe impl EXHostViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self, Shared>;
    }
);

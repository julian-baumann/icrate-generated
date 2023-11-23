//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WKInactiveSchedulingPolicy {
        WKInactiveSchedulingPolicySuspend = 0,
        WKInactiveSchedulingPolicyThrottle = 1,
        WKInactiveSchedulingPolicyNone = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKPreferences")]
    pub struct WKPreferences;

    #[cfg(feature = "WebKit_WKPreferences")]
    unsafe impl ClassType for WKPreferences {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WKPreferences")]
unsafe impl NSCoding for WKPreferences {}

#[cfg(feature = "WebKit_WKPreferences")]
unsafe impl NSObjectProtocol for WKPreferences {}

#[cfg(feature = "WebKit_WKPreferences")]
unsafe impl NSSecureCoding for WKPreferences {}

extern_methods!(
    #[cfg(feature = "WebKit_WKPreferences")]
    unsafe impl WKPreferences {
        #[method(minimumFontSize)]
        pub unsafe fn minimumFontSize(&self) -> CGFloat;

        #[method(setMinimumFontSize:)]
        pub unsafe fn setMinimumFontSize(&self, minimum_font_size: CGFloat);

        #[method(javaScriptCanOpenWindowsAutomatically)]
        pub unsafe fn javaScriptCanOpenWindowsAutomatically(&self) -> bool;

        #[method(setJavaScriptCanOpenWindowsAutomatically:)]
        pub unsafe fn setJavaScriptCanOpenWindowsAutomatically(
            &self,
            java_script_can_open_windows_automatically: bool,
        );

        #[method(isFraudulentWebsiteWarningEnabled)]
        pub unsafe fn isFraudulentWebsiteWarningEnabled(&self) -> bool;

        #[method(setFraudulentWebsiteWarningEnabled:)]
        pub unsafe fn setFraudulentWebsiteWarningEnabled(
            &self,
            fraudulent_website_warning_enabled: bool,
        );

        #[method(shouldPrintBackgrounds)]
        pub unsafe fn shouldPrintBackgrounds(&self) -> bool;

        #[method(setShouldPrintBackgrounds:)]
        pub unsafe fn setShouldPrintBackgrounds(&self, should_print_backgrounds: bool);

        #[method(tabFocusesLinks)]
        pub unsafe fn tabFocusesLinks(&self) -> bool;

        #[method(setTabFocusesLinks:)]
        pub unsafe fn setTabFocusesLinks(&self, tab_focuses_links: bool);

        #[method(isTextInteractionEnabled)]
        pub unsafe fn isTextInteractionEnabled(&self) -> bool;

        #[method(setTextInteractionEnabled:)]
        pub unsafe fn setTextInteractionEnabled(&self, text_interaction_enabled: bool);

        #[method(isSiteSpecificQuirksModeEnabled)]
        pub unsafe fn isSiteSpecificQuirksModeEnabled(&self) -> bool;

        #[method(setSiteSpecificQuirksModeEnabled:)]
        pub unsafe fn setSiteSpecificQuirksModeEnabled(
            &self,
            site_specific_quirks_mode_enabled: bool,
        );

        #[method(isElementFullscreenEnabled)]
        pub unsafe fn isElementFullscreenEnabled(&self) -> bool;

        #[method(setElementFullscreenEnabled:)]
        pub unsafe fn setElementFullscreenEnabled(&self, element_fullscreen_enabled: bool);

        #[method(inactiveSchedulingPolicy)]
        pub unsafe fn inactiveSchedulingPolicy(&self) -> WKInactiveSchedulingPolicy;

        #[method(setInactiveSchedulingPolicy:)]
        pub unsafe fn setInactiveSchedulingPolicy(
            &self,
            inactive_scheduling_policy: WKInactiveSchedulingPolicy,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WKPreferences")]
    unsafe impl WKPreferences {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// WKDeprecated
    #[cfg(feature = "WebKit_WKPreferences")]
    unsafe impl WKPreferences {
        #[deprecated = "Java is no longer supported"]
        #[method(javaEnabled)]
        pub unsafe fn javaEnabled(&self) -> bool;

        #[deprecated = "Java is no longer supported"]
        #[method(setJavaEnabled:)]
        pub unsafe fn setJavaEnabled(&self, java_enabled: bool);

        #[deprecated = "Plug-ins are no longer supported"]
        #[method(plugInsEnabled)]
        pub unsafe fn plugInsEnabled(&self) -> bool;

        #[deprecated = "Plug-ins are no longer supported"]
        #[method(setPlugInsEnabled:)]
        pub unsafe fn setPlugInsEnabled(&self, plug_ins_enabled: bool);

        #[deprecated = "Use WKWebpagePreferences.allowsContentJavaScript to disable content JavaScript on a per-navigation basis"]
        #[method(javaScriptEnabled)]
        pub unsafe fn javaScriptEnabled(&self) -> bool;

        #[deprecated = "Use WKWebpagePreferences.allowsContentJavaScript to disable content JavaScript on a per-navigation basis"]
        #[method(setJavaScriptEnabled:)]
        pub unsafe fn setJavaScriptEnabled(&self, java_script_enabled: bool);
    }
);

//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebFrame")]
    #[deprecated]
    pub struct WebFrame;

    #[cfg(feature = "WebKit_WebFrame")]
    unsafe impl ClassType for WebFrame {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WebFrame")]
unsafe impl NSObjectProtocol for WebFrame {}

extern_methods!(
    #[cfg(feature = "WebKit_WebFrame")]
    unsafe impl WebFrame {
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_WebFrameView",
            feature = "WebKit_WebView"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithName:webFrameView:webView:)]
        pub unsafe fn initWithName_webFrameView_webView(
            this: Option<Allocated<Self>>,
            name: Option<&NSString>,
            view: Option<&WebFrameView>,
            web_view: Option<&WebView>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "WebKit_WebView")]
        #[deprecated]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self, mtm: MainThreadMarker) -> Option<Id<WebView>>;

        #[cfg(feature = "WebKit_WebFrameView")]
        #[deprecated]
        #[method_id(@__retain_semantics Other frameView)]
        pub unsafe fn frameView(&self, mtm: MainThreadMarker) -> Option<Id<WebFrameView>>;

        #[cfg(feature = "WebKit_DOMDocument")]
        #[deprecated]
        #[method_id(@__retain_semantics Other DOMDocument)]
        pub unsafe fn DOMDocument(&self) -> Option<Id<DOMDocument>>;

        #[cfg(feature = "WebKit_DOMHTMLElement")]
        #[deprecated]
        #[method_id(@__retain_semantics Other frameElement)]
        pub unsafe fn frameElement(&self) -> Option<Id<DOMHTMLElement>>;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[deprecated]
        #[method(loadRequest:)]
        pub unsafe fn loadRequest(&self, request: Option<&NSURLRequest>);

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated]
        #[method(loadData:MIMEType:textEncodingName:baseURL:)]
        pub unsafe fn loadData_MIMEType_textEncodingName_baseURL(
            &self,
            data: Option<&NSData>,
            mime_type: Option<&NSString>,
            encoding_name: Option<&NSString>,
            url: Option<&NSURL>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method(loadHTMLString:baseURL:)]
        pub unsafe fn loadHTMLString_baseURL(&self, string: Option<&NSString>, url: Option<&NSURL>);

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method(loadAlternateHTMLString:baseURL:forUnreachableURL:)]
        pub unsafe fn loadAlternateHTMLString_baseURL_forUnreachableURL(
            &self,
            string: Option<&NSString>,
            base_url: Option<&NSURL>,
            unreachable_url: Option<&NSURL>,
        );

        #[cfg(feature = "WebKit_WebArchive")]
        #[deprecated]
        #[method(loadArchive:)]
        pub unsafe fn loadArchive(&self, archive: Option<&WebArchive>);

        #[cfg(feature = "WebKit_WebDataSource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<WebDataSource>>;

        #[cfg(feature = "WebKit_WebDataSource")]
        #[deprecated]
        #[method_id(@__retain_semantics Other provisionalDataSource)]
        pub unsafe fn provisionalDataSource(&self) -> Option<Id<WebDataSource>>;

        #[deprecated]
        #[method(stopLoading)]
        pub unsafe fn stopLoading(&self);

        #[deprecated]
        #[method(reload)]
        pub unsafe fn reload(&self);

        #[deprecated]
        #[method(reloadFromOrigin)]
        pub unsafe fn reloadFromOrigin(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other findFrameNamed:)]
        pub unsafe fn findFrameNamed(&self, name: Option<&NSString>) -> Option<Id<WebFrame>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other parentFrame)]
        pub unsafe fn parentFrame(&self) -> Option<Id<WebFrame>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other childFrames)]
        pub unsafe fn childFrames(&self) -> Id<NSArray>;

        #[cfg(feature = "WebKit_WebScriptObject")]
        #[deprecated]
        #[method_id(@__retain_semantics Other windowObject)]
        pub unsafe fn windowObject(&self) -> Option<Id<WebScriptObject>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WebFrame")]
    unsafe impl WebFrame {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

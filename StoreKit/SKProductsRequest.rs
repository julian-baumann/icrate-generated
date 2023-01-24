//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_protocol!(
    pub unsafe trait SKProductsRequestDelegate: SKRequestDelegate {
        #[cfg(all(
            feature = "StoreKit_SKProductsRequest",
            feature = "StoreKit_SKProductsResponse"
        ))]
        #[method(productsRequest:didReceiveResponse:)]
        unsafe fn productsRequest_didReceiveResponse(
            &self,
            request: &SKProductsRequest,
            response: &SKProductsResponse,
        );
    }

    unsafe impl ProtocolType for dyn SKProductsRequestDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKProductsRequest")]
    pub struct SKProductsRequest;

    #[cfg(feature = "StoreKit_SKProductsRequest")]
    unsafe impl ClassType for SKProductsRequest {
        #[inherits(NSObject)]
        type Super = SKRequest;
    }
);

#[cfg(feature = "StoreKit_SKProductsRequest")]
unsafe impl NSObjectProtocol for SKProductsRequest {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKProductsRequest")]
    unsafe impl SKProductsRequest {
        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithProductIdentifiers:)]
        pub unsafe fn initWithProductIdentifiers(
            this: Option<Allocated<Self>>,
            product_identifiers: &NSSet<NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKProductsRequestDelegate>, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKProductsRequestDelegate>>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKProductsResponse")]
    pub struct SKProductsResponse;

    #[cfg(feature = "StoreKit_SKProductsResponse")]
    unsafe impl ClassType for SKProductsResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKProductsResponse")]
unsafe impl NSObjectProtocol for SKProductsResponse {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKProductsResponse")]
    unsafe impl SKProductsResponse {
        #[cfg(all(feature = "Foundation_NSArray", feature = "StoreKit_SKProduct"))]
        #[method_id(@__retain_semantics Other products)]
        pub unsafe fn products(&self) -> Id<NSArray<SKProduct>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other invalidProductIdentifiers)]
        pub unsafe fn invalidProductIdentifiers(&self) -> Id<NSArray<NSString>, Shared>;
    }
);

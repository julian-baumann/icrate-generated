//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
    pub struct CLCircularGeographicCondition;

    #[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
    unsafe impl ClassType for CLCircularGeographicCondition {
        #[inherits(NSObject)]
        type Super = CLCondition;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
unsafe impl NSCoding for CLCircularGeographicCondition {}

#[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
unsafe impl NSCopying for CLCircularGeographicCondition {}

#[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
unsafe impl NSObjectProtocol for CLCircularGeographicCondition {}

#[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
unsafe impl NSSecureCoding for CLCircularGeographicCondition {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
    unsafe impl CLCircularGeographicCondition {
        #[method(center)]
        pub unsafe fn center(&self) -> CLLocationCoordinate2D;

        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[method_id(@__retain_semantics Init initWithCenter:radius:)]
        pub unsafe fn initWithCenter_radius(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLCondition`
    #[cfg(feature = "CoreLocation_CLCircularGeographicCondition")]
    unsafe impl CLCircularGeographicCondition {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

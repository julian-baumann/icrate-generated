//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLCircularRegion")]
    #[deprecated]
    pub struct CLCircularRegion;

    #[cfg(feature = "CoreLocation_CLCircularRegion")]
    unsafe impl ClassType for CLCircularRegion {
        #[inherits(NSObject)]
        type Super = CLRegion;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLCircularRegion")]
unsafe impl NSCoding for CLCircularRegion {}

#[cfg(feature = "CoreLocation_CLCircularRegion")]
unsafe impl NSCopying for CLCircularRegion {}

#[cfg(feature = "CoreLocation_CLCircularRegion")]
unsafe impl NSObjectProtocol for CLCircularRegion {}

#[cfg(feature = "CoreLocation_CLCircularRegion")]
unsafe impl NSSecureCoding for CLCircularRegion {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLCircularRegion")]
    unsafe impl CLCircularRegion {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithCenter:radius:identifier:)]
        pub unsafe fn initWithCenter_radius_identifier(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Id<Self>;

        #[deprecated]
        #[method(center)]
        pub unsafe fn center(&self) -> CLLocationCoordinate2D;

        #[deprecated]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[deprecated]
        #[method(containsCoordinate:)]
        pub unsafe fn containsCoordinate(&self, coordinate: CLLocationCoordinate2D) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLRegion`
    #[cfg(feature = "CoreLocation_CLCircularRegion")]
    unsafe impl CLCircularRegion {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Please see CLCircularRegion"]
        #[method_id(@__retain_semantics Init initCircularRegionWithCenter:radius:identifier:)]
        pub unsafe fn initCircularRegionWithCenter_radius_identifier(
            this: Allocated<Self>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreLocation_CLCircularRegion")]
    unsafe impl CLCircularRegion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

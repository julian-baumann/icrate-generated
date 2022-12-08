//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutAnchor<AnchorType: Message = Object> {
        _inner0: PhantomData<*mut AnchorType>,
    }

    unsafe impl<AnchorType: Message> ClassType for NSLayoutAnchor<AnchorType> {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl<AnchorType: Message> NSLayoutAnchor<AnchorType> {
        #[method_id(@__retain_semantics Other constraintEqualToAnchor:)]
        pub unsafe fn constraintEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintEqualToAnchor:constant:)]
        pub unsafe fn constraintEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:constant:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:constant:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_constant(
            &self,
            anchor: &NSLayoutAnchor<AnchorType>,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other item)]
        pub unsafe fn item(&self) -> Option<Id<Object, Shared>>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[method_id(@__retain_semantics Other constraintsAffectingLayout)]
        pub unsafe fn constraintsAffectingLayout(&self) -> Id<NSArray<NSLayoutConstraint>, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutXAxisAnchor;

    unsafe impl ClassType for NSLayoutXAxisAnchor {
        type Super = NSLayoutAnchor;
    }
);

extern_methods!(
    unsafe impl NSLayoutXAxisAnchor {
        #[method_id(@__retain_semantics Other anchorWithOffsetToAnchor:)]
        pub unsafe fn anchorWithOffsetToAnchor(
            &self,
            otherAnchor: &NSLayoutXAxisAnchor,
        ) -> Id<NSLayoutDimension, Shared>;

        #[method_id(@__retain_semantics Other constraintEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToSystemSpacingAfterAnchor_multiplier(
            &self,
            anchor: &NSLayoutXAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutYAxisAnchor;

    unsafe impl ClassType for NSLayoutYAxisAnchor {
        type Super = NSLayoutAnchor;
    }
);

extern_methods!(
    unsafe impl NSLayoutYAxisAnchor {
        #[method_id(@__retain_semantics Other anchorWithOffsetToAnchor:)]
        pub unsafe fn anchorWithOffsetToAnchor(
            &self,
            otherAnchor: &NSLayoutYAxisAnchor,
        ) -> Id<NSLayoutDimension, Shared>;

        #[method_id(@__retain_semantics Other constraintEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToSystemSpacingBelowAnchor_multiplier(
            &self,
            anchor: &NSLayoutYAxisAnchor,
            multiplier: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSLayoutDimension;

    unsafe impl ClassType for NSLayoutDimension {
        type Super = NSLayoutAnchor;
    }
);

extern_methods!(
    unsafe impl NSLayoutDimension {
        #[method_id(@__retain_semantics Other constraintEqualToConstant:)]
        pub unsafe fn constraintEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToConstant:)]
        pub unsafe fn constraintGreaterThanOrEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToConstant:)]
        pub unsafe fn constraintLessThanOrEqualToConstant(
            &self,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintEqualToAnchor:multiplier:)]
        pub unsafe fn constraintEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:multiplier:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:multiplier:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_multiplier(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintGreaterThanOrEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintGreaterThanOrEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;

        #[method_id(@__retain_semantics Other constraintLessThanOrEqualToAnchor:multiplier:constant:)]
        pub unsafe fn constraintLessThanOrEqualToAnchor_multiplier_constant(
            &self,
            anchor: &NSLayoutDimension,
            m: CGFloat,
            c: CGFloat,
        ) -> Id<NSLayoutConstraint, Shared>;
    }
);
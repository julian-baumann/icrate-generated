//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundTasks::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGTask")]
    pub struct BGTask;

    #[cfg(feature = "BackgroundTasks_BGTask")]
    unsafe impl ClassType for BGTask {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "BackgroundTasks_BGTask")]
unsafe impl NSObjectProtocol for BGTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGTask")]
    unsafe impl BGTask {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(expirationHandler)]
        pub unsafe fn expirationHandler(&self) -> *mut Block<(), ()>;

        #[method(setExpirationHandler:)]
        pub unsafe fn setExpirationHandler(&self, expiration_handler: Option<&Block<(), ()>>);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(&self) -> Id<Self>;

        #[method(setTaskCompletedWithSuccess:)]
        pub unsafe fn setTaskCompletedWithSuccess(&self, success: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "BackgroundTasks_BGTask")]
    unsafe impl BGTask {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new_class() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    pub struct BGProcessingTask;

    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    unsafe impl ClassType for BGProcessingTask {
        #[inherits(NSObject)]
        type Super = BGTask;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "BackgroundTasks_BGProcessingTask")]
unsafe impl NSObjectProtocol for BGProcessingTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    unsafe impl BGProcessingTask {}
);

extern_methods!(
    /// Methods declared on superclass `BGTask`
    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    unsafe impl BGProcessingTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    unsafe impl BGProcessingTask {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGHealthResearchTask")]
    pub struct BGHealthResearchTask;

    #[cfg(feature = "BackgroundTasks_BGHealthResearchTask")]
    unsafe impl ClassType for BGHealthResearchTask {
        #[inherits(BGTask, NSObject)]
        type Super = BGProcessingTask;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "BackgroundTasks_BGHealthResearchTask")]
unsafe impl NSObjectProtocol for BGHealthResearchTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGHealthResearchTask")]
    unsafe impl BGHealthResearchTask {}
);

extern_methods!(
    /// Methods declared on superclass `BGTask`
    #[cfg(feature = "BackgroundTasks_BGHealthResearchTask")]
    unsafe impl BGHealthResearchTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "BackgroundTasks_BGHealthResearchTask")]
    unsafe impl BGHealthResearchTask {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    pub struct BGAppRefreshTask;

    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    unsafe impl ClassType for BGAppRefreshTask {
        #[inherits(NSObject)]
        type Super = BGTask;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
unsafe impl NSObjectProtocol for BGAppRefreshTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    unsafe impl BGAppRefreshTask {}
);

extern_methods!(
    /// Methods declared on superclass `BGTask`
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    unsafe impl BGAppRefreshTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    unsafe impl BGAppRefreshTask {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

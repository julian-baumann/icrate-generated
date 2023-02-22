//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKWorkoutSessionState {
        HKWorkoutSessionStateNotStarted = 1,
        HKWorkoutSessionStateRunning = 2,
        HKWorkoutSessionStateEnded = 3,
        HKWorkoutSessionStatePaused = 4,
        HKWorkoutSessionStatePrepared = 5,
        HKWorkoutSessionStateStopped = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKWorkoutSession")]
    pub struct HKWorkoutSession;

    #[cfg(feature = "HealthKit_HKWorkoutSession")]
    unsafe impl ClassType for HKWorkoutSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKWorkoutSession")]
unsafe impl NSCoding for HKWorkoutSession {}

#[cfg(feature = "HealthKit_HKWorkoutSession")]
unsafe impl NSObjectProtocol for HKWorkoutSession {}

#[cfg(feature = "HealthKit_HKWorkoutSession")]
unsafe impl NSSecureCoding for HKWorkoutSession {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKWorkoutSession")]
    unsafe impl HKWorkoutSession {
        #[deprecated]
        #[method(activityType)]
        pub unsafe fn activityType(&self) -> HKWorkoutActivityType;

        #[deprecated]
        #[method(locationType)]
        pub unsafe fn locationType(&self) -> HKWorkoutSessionLocationType;

        #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
        #[method_id(@__retain_semantics Other workoutConfiguration)]
        pub unsafe fn workoutConfiguration(&self) -> Id<HKWorkoutConfiguration>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn HKWorkoutSessionDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn HKWorkoutSessionDelegate>>,
        );

        #[method(state)]
        pub unsafe fn state(&self) -> HKWorkoutSessionState;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "HealthKit_HKWorkoutActivity")]
        #[method_id(@__retain_semantics Other currentActivity)]
        pub unsafe fn currentActivity(&self) -> Id<HKWorkoutActivity>;

        #[deprecated]
        #[method_id(@__retain_semantics Init initWithActivityType:locationType:)]
        pub unsafe fn initWithActivityType_locationType(
            this: Option<Allocated<Self>>,
            activity_type: HKWorkoutActivityType,
            location_type: HKWorkoutSessionLocationType,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithConfiguration:error:_)]
        pub unsafe fn initWithConfiguration_error(
            this: Option<Allocated<Self>>,
            workout_configuration: &HKWorkoutConfiguration,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "HealthKit_HKHealthStore",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Init initWithHealthStore:configuration:error:_)]
        pub unsafe fn initWithHealthStore_configuration_error(
            this: Option<Allocated<Self>>,
            health_store: &HKHealthStore,
            workout_configuration: &HKWorkoutConfiguration,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(prepare)]
        pub unsafe fn prepare(&self);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(startActivityWithDate:)]
        pub unsafe fn startActivityWithDate(&self, date: Option<&NSDate>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(stopActivityWithDate:)]
        pub unsafe fn stopActivityWithDate(&self, date: Option<&NSDate>);

        #[method(end)]
        pub unsafe fn end(&self);

        #[method(pause)]
        pub unsafe fn pause(&self);

        #[method(resume)]
        pub unsafe fn resume(&self);

        #[cfg(feature = "HealthKit_HKLiveWorkoutBuilder")]
        #[method_id(@__retain_semantics Other associatedWorkoutBuilder)]
        pub unsafe fn associatedWorkoutBuilder(&self) -> Id<HKLiveWorkoutBuilder>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method(beginNewActivityWithConfiguration:date:metadata:)]
        pub unsafe fn beginNewActivityWithConfiguration_date_metadata(
            &self,
            workout_configuration: &HKWorkoutConfiguration,
            date: &NSDate,
            metadata: Option<&NSDictionary<NSString, Object>>,
        );

        #[cfg(feature = "Foundation_NSDate")]
        #[method(endCurrentActivityOnDate:)]
        pub unsafe fn endCurrentActivityOnDate(&self, date: &NSDate);
    }
);

extern_protocol!(
    pub unsafe trait HKWorkoutSessionDelegate: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSDate", feature = "HealthKit_HKWorkoutSession"))]
        #[method(workoutSession:didChangeToState:fromState:date:)]
        unsafe fn workoutSession_didChangeToState_fromState_date(
            &self,
            workout_session: &HKWorkoutSession,
            to_state: HKWorkoutSessionState,
            from_state: HKWorkoutSessionState,
            date: &NSDate,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "HealthKit_HKWorkoutSession"))]
        #[method(workoutSession:didFailWithError:)]
        unsafe fn workoutSession_didFailWithError(
            &self,
            workout_session: &HKWorkoutSession,
            error: &NSError,
        );

        #[cfg(all(
            feature = "HealthKit_HKWorkoutEvent",
            feature = "HealthKit_HKWorkoutSession"
        ))]
        #[optional]
        #[method(workoutSession:didGenerateEvent:)]
        unsafe fn workoutSession_didGenerateEvent(
            &self,
            workout_session: &HKWorkoutSession,
            event: &HKWorkoutEvent,
        );

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKWorkoutConfiguration",
            feature = "HealthKit_HKWorkoutSession"
        ))]
        #[optional]
        #[method(workoutSession:didBeginActivityWithConfiguration:date:)]
        unsafe fn workoutSession_didBeginActivityWithConfiguration_date(
            &self,
            workout_session: &HKWorkoutSession,
            workout_configuration: &HKWorkoutConfiguration,
            date: &NSDate,
        );

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "HealthKit_HKWorkoutConfiguration",
            feature = "HealthKit_HKWorkoutSession"
        ))]
        #[optional]
        #[method(workoutSession:didEndActivityWithConfiguration:date:)]
        unsafe fn workoutSession_didEndActivityWithConfiguration_date(
            &self,
            workout_session: &HKWorkoutSession,
            workout_configuration: &HKWorkoutConfiguration,
            date: &NSDate,
        );
    }

    unsafe impl ProtocolType for dyn HKWorkoutSessionDelegate {}
);
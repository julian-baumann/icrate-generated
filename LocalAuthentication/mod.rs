// This file has been automatically generated by `objc2`'s `header-translator`.
// DO NOT EDIT

//! # Bindings to the `LocalAuthentication` framework

#[path = "../../fixes/LocalAuthentication/mod.rs"]
mod fixes;
pub use self::fixes::*;

#[cfg_attr(
    feature = "apple",
    link(name = "LocalAuthentication", kind = "framework")
)]
extern "C" {}

#[path = "LABase.rs"]
mod __LABase;
#[path = "LAContext.rs"]
mod __LAContext;
#[path = "LAError.rs"]
mod __LAError;
#[path = "LAPersistedRight.rs"]
mod __LAPersistedRight;
#[path = "LAPrivateKey.rs"]
mod __LAPrivateKey;
#[path = "LAPublicDefines.rs"]
mod __LAPublicDefines;
#[path = "LAPublicKey.rs"]
mod __LAPublicKey;
#[path = "LARequirement.rs"]
mod __LARequirement;
#[path = "LARight.rs"]
mod __LARight;
#[path = "LARightStore.rs"]
mod __LARightStore;
#[path = "LASecret.rs"]
mod __LASecret;

pub use self::__LAContext::LAAccessControlOperation;
pub use self::__LAContext::LABiometryType;
#[cfg(feature = "LocalAuthentication_LAContext")]
pub use self::__LAContext::LAContext;
pub use self::__LAContext::LAPolicy;
pub use self::__LAContext::LATouchIDAuthenticationMaximumAllowableReuseDuration;
pub use self::__LAContext::{
    LAAccessControlOperationCreateItem, LAAccessControlOperationCreateKey,
    LAAccessControlOperationUseItem, LAAccessControlOperationUseKeyDecrypt,
    LAAccessControlOperationUseKeyKeyExchange, LAAccessControlOperationUseKeySign,
};
pub use self::__LAContext::{
    LABiometryNone, LABiometryTypeFaceID, LABiometryTypeNone, LABiometryTypeOpticID,
    LABiometryTypeTouchID,
};
pub use self::__LAContext::{
    LAPolicyDeviceOwnerAuthentication, LAPolicyDeviceOwnerAuthenticationWithBiometrics,
    LAPolicyDeviceOwnerAuthenticationWithBiometricsOrWatch,
    LAPolicyDeviceOwnerAuthenticationWithWatch,
    LAPolicyDeviceOwnerAuthenticationWithWristDetection,
};
pub use self::__LAError::LAErrorDomain;
#[cfg(feature = "LocalAuthentication_LAPersistedRight")]
pub use self::__LAPersistedRight::LAPersistedRight;
#[cfg(feature = "LocalAuthentication_LAPrivateKey")]
pub use self::__LAPrivateKey::LAPrivateKey;
#[cfg(feature = "LocalAuthentication_LAPublicKey")]
pub use self::__LAPublicKey::LAPublicKey;
#[cfg(feature = "LocalAuthentication_LAAuthenticationRequirement")]
pub use self::__LARequirement::LAAuthenticationRequirement;
#[cfg(feature = "LocalAuthentication_LABiometryFallbackRequirement")]
pub use self::__LARequirement::LABiometryFallbackRequirement;
#[cfg(feature = "LocalAuthentication_LARight")]
pub use self::__LARight::LARight;
pub use self::__LARight::LARightState;
pub use self::__LARight::{
    LARightStateAuthorized, LARightStateAuthorizing, LARightStateNotAuthorized, LARightStateUnknown,
};
#[cfg(feature = "LocalAuthentication_LARightStore")]
pub use self::__LARightStore::LARightStore;
#[cfg(feature = "LocalAuthentication_LASecret")]
pub use self::__LASecret::LASecret;

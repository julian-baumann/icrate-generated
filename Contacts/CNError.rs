//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_static!(CNErrorDomain: Option<&'static NSString>);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CNErrorCode {
        CNErrorCodeCommunicationError = 1,
        CNErrorCodeDataAccessError = 2,
        CNErrorCodeAuthorizationDenied = 100,
        CNErrorCodeNoAccessableWritableContainers = 101,
        CNErrorCodeUnauthorizedKeys = 102,
        CNErrorCodeFeatureDisabledByUser = 103,
        CNErrorCodeFeatureNotAvailable = 104,
        CNErrorCodeRecordDoesNotExist = 200,
        CNErrorCodeInsertedRecordAlreadyExists = 201,
        CNErrorCodeContainmentCycle = 202,
        CNErrorCodeContainmentScope = 203,
        CNErrorCodeParentRecordDoesNotExist = 204,
        CNErrorCodeRecordIdentifierInvalid = 205,
        CNErrorCodeRecordNotWritable = 206,
        CNErrorCodeParentContainerNotWritable = 207,
        CNErrorCodeValidationMultipleErrors = 300,
        CNErrorCodeValidationTypeMismatch = 301,
        CNErrorCodeValidationConfigurationError = 302,
        CNErrorCodePredicateInvalid = 400,
        CNErrorCodePolicyViolation = 500,
        CNErrorCodeClientIdentifierInvalid = 600,
        CNErrorCodeClientIdentifierDoesNotExist = 601,
        CNErrorCodeClientIdentifierCollision = 602,
        CNErrorCodeChangeHistoryExpired = 603,
        CNErrorCodeChangeHistoryInvalidAnchor = 604,
        CNErrorCodeChangeHistoryInvalidFetchRequest = 605,
        CNErrorCodeVCardMalformed = 700,
        CNErrorCodeVCardSummarizationError = 701,
    }
);

extern_static!(CNErrorUserInfoAffectedRecordsKey: Option<&'static NSString>);

extern_static!(CNErrorUserInfoAffectedRecordIdentifiersKey: Option<&'static NSString>);

extern_static!(CNErrorUserInfoValidationErrorsKey: Option<&'static NSString>);

extern_static!(CNErrorUserInfoKeyPathsKey: Option<&'static NSString>);

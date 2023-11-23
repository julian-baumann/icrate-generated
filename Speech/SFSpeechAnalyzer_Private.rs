//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFSpeechAnalyzer")]
    pub struct _SFSpeechAnalyzer;

    #[cfg(feature = "Speech__SFSpeechAnalyzer")]
    unsafe impl ClassType for _SFSpeechAnalyzer {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFSpeechAnalyzer")]
unsafe impl NSObjectProtocol for _SFSpeechAnalyzer {}

extern_methods!(
    #[cfg(feature = "Speech__SFSpeechAnalyzer")]
    unsafe impl _SFSpeechAnalyzer {
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Speech__SFModelDownloadRequest",
            feature = "Speech__SFSpeechAnalyzerTranscriberOptions"
        ))]
        #[method_id(@__retain_semantics Other modelDownloadRequestForClientIdentifier:transcriberOptions:)]
        pub unsafe fn modelDownloadRequestForClientIdentifier_transcriberOptions(
            client_identifier: &NSString,
            transcriber_options: &_SFSpeechAnalyzerTranscriberOptions,
        ) -> Option<Id<_SFModelDownloadRequest>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[cfg(feature = "Speech__SFInputSequencer")]
        #[method_id(@__retain_semantics Other inputSequence)]
        pub unsafe fn inputSequence(&self) -> Id<_SFInputSequencer>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelPendingResultsAndPauseWithCompletion:)]
        pub unsafe fn cancelPendingResultsAndPauseWithCompletion(
            &self,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(resumeWithCompletion:)]
        pub unsafe fn resumeWithCompletion(&self, completion: &Block<(*mut NSError,), ()>);

        #[cfg(feature = "Foundation_NSError")]
        #[method(finalizeWithCompletion:)]
        pub unsafe fn finalizeWithCompletion(&self, completion: &Block<(*mut NSError,), ()>);

        #[cfg(feature = "Foundation_NSError")]
        #[method(finalizeAndFinishWithCompletion:)]
        pub unsafe fn finalizeAndFinishWithCompletion(
            &self,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(finalizeAndFinishThroughEndOfInputWithCompletion:)]
        pub unsafe fn finalizeAndFinishThroughEndOfInputWithCompletion(
            &self,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method(requestResultAtEndpointTimes:)]
        pub unsafe fn requestResultAtEndpointTimes(&self, times: &NSArray<NSValue>);

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method(getModelInfoTasksWithCompletion:)]
        pub unsafe fn getModelInfoTasksWithCompletion(
            &self,
            completion: &Block<(NonNull<NSSet<NSString>>,), ()>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(getModelInfoLanguageWithCompletion:)]
        pub unsafe fn getModelInfoLanguageWithCompletion(
            &self,
            completion: &Block<(NonNull<NSString>,), ()>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(getRecognitionStatisticsWithCompletion:)]
        pub unsafe fn getRecognitionStatisticsWithCompletion(
            &self,
            completion: &Block<(NonNull<NSDictionary>,), ()>,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(getRecognitionUtterenceStatisticsWithCompletion:)]
        pub unsafe fn getRecognitionUtterenceStatisticsWithCompletion(
            &self,
            completion: &Block<(NonNull<NSDictionary>,), ()>,
        );

        #[cfg(feature = "Speech__SFAnalysisContext")]
        #[method(getContextWithCompletion:)]
        pub unsafe fn getContextWithCompletion(
            &self,
            completion: &Block<(NonNull<_SFAnalysisContext>,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSProgress"))]
        #[method(prepareToAnalyzeReportingInto:completion:)]
        pub unsafe fn prepareToAnalyzeReportingInto_completion(
            &self,
            progress: Option<&NSProgress>,
            completion: &Block<(*mut NSError,), ()>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFSpeechAnalyzer")]
    unsafe impl _SFSpeechAnalyzer {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait _SFSpeechAnalyzerTranscriberResultDelegate {
        #[cfg(all(
            feature = "Speech__SFSpeechAnalyzer",
            feature = "Speech__SFTranscriberResult"
        ))]
        #[method(speechAnalyzer:didProduceTranscriberResult:)]
        unsafe fn speechAnalyzer_didProduceTranscriberResult(
            &self,
            speech_analyzer: &_SFSpeechAnalyzer,
            transcriber_result: &_SFTranscriberResult,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Speech__SFSpeechAnalyzer"))]
        #[method(speechAnalyzer:didStopTranscriptionWithError:)]
        unsafe fn speechAnalyzer_didStopTranscriptionWithError(
            &self,
            speech_analyzer: &_SFSpeechAnalyzer,
            error: &NSError,
        );

        #[cfg(feature = "Speech__SFSpeechAnalyzer")]
        #[optional]
        #[method(speechAnalyzerDidProduceAllTranscriberResults:)]
        unsafe fn speechAnalyzerDidProduceAllTranscriberResults(
            &self,
            speech_analyzer: &_SFSpeechAnalyzer,
        );
    }

    unsafe impl ProtocolType for dyn _SFSpeechAnalyzerTranscriberResultDelegate {}
);

extern_protocol!(
    pub unsafe trait _SFSpeechAnalyzerEndpointingResultDelegate {
        #[cfg(all(
            feature = "Speech__SFEndpointingResult",
            feature = "Speech__SFSpeechAnalyzer"
        ))]
        #[method(speechAnalyzer:didProduceEndpointingResult:)]
        unsafe fn speechAnalyzer_didProduceEndpointingResult(
            &self,
            speech_analyzer: &_SFSpeechAnalyzer,
            endpointing_result: &_SFEndpointingResult,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Speech__SFSpeechAnalyzer"))]
        #[method(speechAnalyzer:didStopEndpointingWithError:)]
        unsafe fn speechAnalyzer_didStopEndpointingWithError(
            &self,
            speech_analyzer: &_SFSpeechAnalyzer,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn _SFSpeechAnalyzerEndpointingResultDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFModelDownloadRequest")]
    pub struct _SFModelDownloadRequest;

    #[cfg(feature = "Speech__SFModelDownloadRequest")]
    unsafe impl ClassType for _SFModelDownloadRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFModelDownloadRequest")]
unsafe impl NSObjectProtocol for _SFModelDownloadRequest {}

extern_methods!(
    #[cfg(feature = "Speech__SFModelDownloadRequest")]
    unsafe impl _SFModelDownloadRequest {
        #[cfg(feature = "Foundation_NSProgress")]
        #[method_id(@__retain_semantics Other progress)]
        pub unsafe fn progress(&self) -> Id<NSProgress>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(downloadWithCompletion:)]
        pub unsafe fn downloadWithCompletion(&self, completion: &Block<(*mut NSError,), ()>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFModelDownloadRequest")]
    unsafe impl _SFModelDownloadRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFSpeechAnalyzerTranscriberOptions")]
    pub struct _SFSpeechAnalyzerTranscriberOptions;

    #[cfg(feature = "Speech__SFSpeechAnalyzerTranscriberOptions")]
    unsafe impl ClassType for _SFSpeechAnalyzerTranscriberOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFSpeechAnalyzerTranscriberOptions")]
unsafe impl NSObjectProtocol for _SFSpeechAnalyzerTranscriberOptions {}

extern_methods!(
    #[cfg(feature = "Speech__SFSpeechAnalyzerTranscriberOptions")]
    unsafe impl _SFSpeechAnalyzerTranscriberOptions {
        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: &NSLocale);

        #[method(taskHint)]
        pub unsafe fn taskHint(&self) -> SFSpeechRecognitionTaskHint;

        #[method(setTaskHint:)]
        pub unsafe fn setTaskHint(&self, task_hint: SFSpeechRecognitionTaskHint);

        #[cfg(feature = "Speech__SFTranscriberModelOptions")]
        #[method_id(@__retain_semantics Other modelOptions)]
        pub unsafe fn modelOptions(&self) -> Option<Id<_SFTranscriberModelOptions>>;

        #[cfg(feature = "Speech__SFTranscriberModelOptions")]
        #[method(setModelOptions:)]
        pub unsafe fn setModelOptions(&self, model_options: Option<&_SFTranscriberModelOptions>);

        #[method(transcriptionOptions)]
        pub unsafe fn transcriptionOptions(&self) -> _SFTranscriptionOptions;

        #[method(setTranscriptionOptions:)]
        pub unsafe fn setTranscriptionOptions(
            &self,
            transcription_options: _SFTranscriptionOptions,
        );

        #[method(attributeOptions)]
        pub unsafe fn attributeOptions(&self) -> _SFTranscriptionResultAttributeOptions;

        #[method(setAttributeOptions:)]
        pub unsafe fn setAttributeOptions(
            &self,
            attribute_options: _SFTranscriptionResultAttributeOptions,
        );

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFSpeechAnalyzerTranscriberOptions")]
    unsafe impl _SFSpeechAnalyzerTranscriberOptions {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFSpeechAnalyzerCommandRecognizerOptions")]
    pub struct _SFSpeechAnalyzerCommandRecognizerOptions;

    #[cfg(feature = "Speech__SFSpeechAnalyzerCommandRecognizerOptions")]
    unsafe impl ClassType for _SFSpeechAnalyzerCommandRecognizerOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFSpeechAnalyzerCommandRecognizerOptions")]
unsafe impl NSObjectProtocol for _SFSpeechAnalyzerCommandRecognizerOptions {}

extern_methods!(
    #[cfg(feature = "Speech__SFSpeechAnalyzerCommandRecognizerOptions")]
    unsafe impl _SFSpeechAnalyzerCommandRecognizerOptions {
        #[cfg(feature = "Speech_EARVoiceCommandActiveSet")]
        #[method_id(@__retain_semantics Other voiceCommandActiveSet)]
        pub unsafe fn voiceCommandActiveSet(&self) -> Id<EARVoiceCommandActiveSet>;

        #[cfg(feature = "Speech_EARVoiceCommandActiveSet")]
        #[method(setVoiceCommandActiveSet:)]
        pub unsafe fn setVoiceCommandActiveSet(
            &self,
            voice_command_active_set: &EARVoiceCommandActiveSet,
        );

        #[cfg(feature = "Speech_EARVoiceCommandActiveSet")]
        #[method_id(@__retain_semantics Init initWithVoiceCommandActiveSet:)]
        pub unsafe fn initWithVoiceCommandActiveSet(
            this: Allocated<Self>,
            voice_command_active_set: &EARVoiceCommandActiveSet,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFSpeechAnalyzerCommandRecognizerOptions")]
    unsafe impl _SFSpeechAnalyzerCommandRecognizerOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptions")]
    pub struct _SFSpeechAnalyzerOptions;

    #[cfg(feature = "Speech__SFSpeechAnalyzerOptions")]
    unsafe impl ClassType for _SFSpeechAnalyzerOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFSpeechAnalyzerOptions")]
unsafe impl NSCopying for _SFSpeechAnalyzerOptions {}

#[cfg(feature = "Speech__SFSpeechAnalyzerOptions")]
unsafe impl NSObjectProtocol for _SFSpeechAnalyzerOptions {}

extern_methods!(
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptions")]
    unsafe impl _SFSpeechAnalyzerOptions {
        #[method(highPriority)]
        pub unsafe fn highPriority(&self) -> bool;

        #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
        #[method_id(@__retain_semantics Other loggingInfo)]
        pub unsafe fn loggingInfo(&self) -> Option<Id<_SFSpeechAnalyzerOptionsLoggingInfo>>;

        #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
        #[method_id(@__retain_semantics Other powerContext)]
        pub unsafe fn powerContext(&self) -> Option<Id<_SFSpeechAnalyzerOptionsPowerContext>>;

        #[cfg(all(
            feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo",
            feature = "Speech__SFSpeechAnalyzerOptionsPowerContext"
        ))]
        #[method_id(@__retain_semantics Init initWithHighPriority:loggingInfo:powerContext:)]
        pub unsafe fn initWithHighPriority_loggingInfo_powerContext(
            this: Allocated<Self>,
            high_priority: bool,
            logging_info: Option<&_SFSpeechAnalyzerOptionsLoggingInfo>,
            power_context: Option<&_SFSpeechAnalyzerOptionsPowerContext>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptions")]
    unsafe impl _SFSpeechAnalyzerOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
    pub struct _SFSpeechAnalyzerOptionsLoggingInfo;

    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
    unsafe impl ClassType for _SFSpeechAnalyzerOptionsLoggingInfo {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
unsafe impl NSCopying for _SFSpeechAnalyzerOptionsLoggingInfo {}

#[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
unsafe impl NSObjectProtocol for _SFSpeechAnalyzerOptionsLoggingInfo {}

extern_methods!(
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
    unsafe impl _SFSpeechAnalyzerOptionsLoggingInfo {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other asrID)]
        pub unsafe fn asrID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other requestID)]
        pub unsafe fn requestID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithAsrID:requestID:)]
        pub unsafe fn initWithAsrID_requestID(
            this: Allocated<Self>,
            asr_id: &NSUUID,
            request_id: &NSUUID,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsLoggingInfo")]
    unsafe impl _SFSpeechAnalyzerOptionsLoggingInfo {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
    pub struct _SFSpeechAnalyzerOptionsPowerContext;

    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
    unsafe impl ClassType for _SFSpeechAnalyzerOptionsPowerContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
unsafe impl NSCopying for _SFSpeechAnalyzerOptionsPowerContext {}

#[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
unsafe impl NSObjectProtocol for _SFSpeechAnalyzerOptionsPowerContext {}

extern_methods!(
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
    unsafe impl _SFSpeechAnalyzerOptionsPowerContext {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other ane)]
        pub unsafe fn ane(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cpu)]
        pub unsafe fn cpu(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other gpu)]
        pub unsafe fn gpu(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAne:cpu:gpu:)]
        pub unsafe fn initWithAne_cpu_gpu(
            this: Allocated<Self>,
            ane: &NSString,
            cpu: &NSString,
            gpu: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFSpeechAnalyzerOptionsPowerContext")]
    unsafe impl _SFSpeechAnalyzerOptionsPowerContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFTranscriberModelOptions")]
    pub struct _SFTranscriberModelOptions;

    #[cfg(feature = "Speech__SFTranscriberModelOptions")]
    unsafe impl ClassType for _SFTranscriberModelOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFTranscriberModelOptions")]
unsafe impl NSCopying for _SFTranscriberModelOptions {}

#[cfg(feature = "Speech__SFTranscriberModelOptions")]
unsafe impl NSObjectProtocol for _SFTranscriberModelOptions {}

extern_methods!(
    #[cfg(feature = "Speech__SFTranscriberModelOptions")]
    unsafe impl _SFTranscriberModelOptions {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other supplementalModelURL)]
        pub unsafe fn supplementalModelURL(&self) -> Option<Id<NSURL>>;

        #[method(farField)]
        pub unsafe fn farField(&self) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other modelOverrideURL)]
        pub unsafe fn modelOverrideURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other taskForMemoryLock)]
        pub unsafe fn taskForMemoryLock(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other speechProfileURLs)]
        pub unsafe fn speechProfileURLs(&self) -> Id<NSArray<NSURL>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithSupplementalModelURL:farField:modelOverrideURL:speechProfileURLs:taskForMemoryLock:)]
        pub unsafe fn initWithSupplementalModelURL_farField_modelOverrideURL_speechProfileURLs_taskForMemoryLock(
            this: Allocated<Self>,
            supplemental_model_url: Option<&NSURL>,
            far_field: bool,
            model_override_url: Option<&NSURL>,
            speech_profile_ur_ls: &NSArray<NSURL>,
            task_for_memory_lock: Option<&NSString>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFTranscriberModelOptions")]
    unsafe impl _SFTranscriberModelOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFTranscriberResult")]
    pub struct _SFTranscriberResult;

    #[cfg(feature = "Speech__SFTranscriberResult")]
    unsafe impl ClassType for _SFTranscriberResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFTranscriberResult")]
unsafe impl NSObjectProtocol for _SFTranscriberResult {}

extern_methods!(
    #[cfg(feature = "Speech__SFTranscriberResult")]
    unsafe impl _SFTranscriberResult {
        #[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
        #[method_id(@__retain_semantics Other normalizedTranscriberMultisegmentResult)]
        pub unsafe fn normalizedTranscriberMultisegmentResult(
            &self,
        ) -> Id<_STTranscriberMultisegmentResult>;

        #[cfg(feature = "Speech__STCommandRecognizerResult")]
        #[method_id(@__retain_semantics Other normalizedCommandRecognizerResult)]
        pub unsafe fn normalizedCommandRecognizerResult(
            &self,
        ) -> Option<Id<_STCommandRecognizerResult>>;

        #[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
        #[method_id(@__retain_semantics Other contextualizedTranscriberMultisegmentResult)]
        pub unsafe fn contextualizedTranscriberMultisegmentResult(
            &self,
        ) -> Id<_STTranscriberMultisegmentResult>;

        #[cfg(feature = "Speech__STCommandRecognizerResult")]
        #[method_id(@__retain_semantics Other contextualizedCommandRecognizerResult)]
        pub unsafe fn contextualizedCommandRecognizerResult(
            &self,
        ) -> Option<Id<_STCommandRecognizerResult>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFTranscriberResult")]
    unsafe impl _SFTranscriberResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
    pub struct _STTranscriberMultisegmentResult;

    #[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
    unsafe impl ClassType for _STTranscriberMultisegmentResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
unsafe impl NSCopying for _STTranscriberMultisegmentResult {}

#[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
unsafe impl NSObjectProtocol for _STTranscriberMultisegmentResult {}

extern_methods!(
    #[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
    unsafe impl _STTranscriberMultisegmentResult {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Speech__SFAnalyzerTranscriptionSegment"
        ))]
        #[method_id(@__retain_semantics Other segments)]
        pub unsafe fn segments(&self) -> Id<NSArray<_SFAnalyzerTranscriptionSegment>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Speech__SFToken"))]
        #[method_id(@__retain_semantics Other transcriptions)]
        pub unsafe fn transcriptions(&self) -> Id<NSArray<NSArray<_SFToken>>>;

        #[method(earResultType)]
        pub unsafe fn earResultType(&self) -> _SFEARResultType;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSIndexPath"))]
        #[method_id(@__retain_semantics Other nBestChoices)]
        pub unsafe fn nBestChoices(&self) -> Id<NSArray<NSIndexPath>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__STTranscriberMultisegmentResult")]
    unsafe impl _STTranscriberMultisegmentResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFAnalyzerTranscriptionSegment")]
    pub struct _SFAnalyzerTranscriptionSegment;

    #[cfg(feature = "Speech__SFAnalyzerTranscriptionSegment")]
    unsafe impl ClassType for _SFAnalyzerTranscriptionSegment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFAnalyzerTranscriptionSegment")]
unsafe impl NSObjectProtocol for _SFAnalyzerTranscriptionSegment {}

extern_methods!(
    #[cfg(feature = "Speech__SFAnalyzerTranscriptionSegment")]
    unsafe impl _SFAnalyzerTranscriptionSegment {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Speech__SFToken"))]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSArray<_SFToken>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Speech__SFToken"))]
        #[method_id(@__retain_semantics Other alternatives)]
        pub unsafe fn alternatives(&self) -> Id<NSArray<NSArray<_SFToken>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Speech__SFToken"))]
        #[method_id(@__retain_semantics Init initWithText:alternatives:)]
        pub unsafe fn initWithText_alternatives(
            this: Allocated<Self>,
            text: &NSArray<_SFToken>,
            alternatives: &NSArray<NSArray<_SFToken>>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFAnalyzerTranscriptionSegment")]
    unsafe impl _SFAnalyzerTranscriptionSegment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__STCommandRecognizerResult")]
    pub struct _STCommandRecognizerResult;

    #[cfg(feature = "Speech__STCommandRecognizerResult")]
    unsafe impl ClassType for _STCommandRecognizerResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__STCommandRecognizerResult")]
unsafe impl NSCopying for _STCommandRecognizerResult {}

#[cfg(feature = "Speech__STCommandRecognizerResult")]
unsafe impl NSObjectProtocol for _STCommandRecognizerResult {}

extern_methods!(
    #[cfg(feature = "Speech__STCommandRecognizerResult")]
    unsafe impl _STCommandRecognizerResult {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Speech__SFCommandRecognizerInterpretation"
        ))]
        #[method_id(@__retain_semantics Init initWithTranscriptionCommands:)]
        pub unsafe fn initWithTranscriptionCommands(
            this: Allocated<Self>,
            transcription_commands: &NSArray<NSArray<_SFCommandRecognizerInterpretation>>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Speech__SFCommandRecognizerInterpretation"
        ))]
        #[method_id(@__retain_semantics Other transcriptionCommands)]
        pub unsafe fn transcriptionCommands(
            &self,
        ) -> Id<NSArray<NSArray<_SFCommandRecognizerInterpretation>>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__STCommandRecognizerResult")]
    unsafe impl _STCommandRecognizerResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFCommandRecognizerInterpretation")]
    pub struct _SFCommandRecognizerInterpretation;

    #[cfg(feature = "Speech__SFCommandRecognizerInterpretation")]
    unsafe impl ClassType for _SFCommandRecognizerInterpretation {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFCommandRecognizerInterpretation")]
unsafe impl NSObjectProtocol for _SFCommandRecognizerInterpretation {}

extern_methods!(
    #[cfg(feature = "Speech__SFCommandRecognizerInterpretation")]
    unsafe impl _SFCommandRecognizerInterpretation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other commandIdentifier)]
        pub unsafe fn commandIdentifier(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other suiteIdentifiers)]
        pub unsafe fn suiteIdentifiers(&self) -> Id<NSSet<NSString>>;

        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other verbIndexes)]
        pub unsafe fn verbIndexes(&self) -> Id<NSIndexSet>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Speech__SFCommandRecognizerArgument"
        ))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Id<NSArray<_SFCommandRecognizerArgument>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSIndexSet",
            feature = "Foundation_NSSet",
            feature = "Foundation_NSString",
            feature = "Speech__SFCommandRecognizerArgument"
        ))]
        #[method_id(@__retain_semantics Init initWithCommandIdentifier:suiteIdentifiers:range:verbIndexes:arguments:)]
        pub unsafe fn initWithCommandIdentifier_suiteIdentifiers_range_verbIndexes_arguments(
            this: Allocated<Self>,
            command_identifier: &NSString,
            suite_identifiers: &NSSet<NSString>,
            range: NSRange,
            verb_indexes: &NSIndexSet,
            arguments: &NSArray<_SFCommandRecognizerArgument>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFCommandRecognizerInterpretation")]
    unsafe impl _SFCommandRecognizerInterpretation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFCommandRecognizerArgument")]
    pub struct _SFCommandRecognizerArgument;

    #[cfg(feature = "Speech__SFCommandRecognizerArgument")]
    unsafe impl ClassType for _SFCommandRecognizerArgument {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFCommandRecognizerArgument")]
unsafe impl NSObjectProtocol for _SFCommandRecognizerArgument {}

extern_methods!(
    #[cfg(feature = "Speech__SFCommandRecognizerArgument")]
    unsafe impl _SFCommandRecognizerArgument {
        #[method(presence)]
        pub unsafe fn presence(&self) -> _SFCommandRecognizerArgumentPresence;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other indexes)]
        pub unsafe fn indexes(&self) -> Id<NSIndexSet>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Other adpositionIndexes)]
        pub unsafe fn adpositionIndexes(&self) -> Id<NSIndexSet>;

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method_id(@__retain_semantics Init initWithPresence:indexes:adpositionIndexes:)]
        pub unsafe fn initWithPresence_indexes_adpositionIndexes(
            this: Allocated<Self>,
            presence: _SFCommandRecognizerArgumentPresence,
            indexes: &NSIndexSet,
            adposition_indexes: &NSIndexSet,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFCommandRecognizerArgument")]
    unsafe impl _SFCommandRecognizerArgument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFEndpointingResult")]
    pub struct _SFEndpointingResult;

    #[cfg(feature = "Speech__SFEndpointingResult")]
    unsafe impl ClassType for _SFEndpointingResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFEndpointingResult")]
unsafe impl NSObjectProtocol for _SFEndpointingResult {}

extern_methods!(
    #[cfg(feature = "Speech__SFEndpointingResult")]
    unsafe impl _SFEndpointingResult {
        #[method(wordCount)]
        pub unsafe fn wordCount(&self) -> NSInteger;

        #[method(eosLikelihood)]
        pub unsafe fn eosLikelihood(&self) -> c_double;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other pauseCounts)]
        pub unsafe fn pauseCounts(&self) -> Id<NSArray<NSNumber>>;

        #[method(silencePosterior)]
        pub unsafe fn silencePosterior(&self) -> c_double;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFEndpointingResult")]
    unsafe impl _SFEndpointingResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech__SFToken")]
    pub struct _SFToken;

    #[cfg(feature = "Speech__SFToken")]
    unsafe impl ClassType for _SFToken {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech__SFToken")]
unsafe impl NSCopying for _SFToken {}

#[cfg(feature = "Speech__SFToken")]
unsafe impl NSObjectProtocol for _SFToken {}

extern_methods!(
    #[cfg(feature = "Speech__SFToken")]
    unsafe impl _SFToken {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString>;

        #[method(confidence)]
        pub unsafe fn confidence(&self) -> c_double;

        #[method(startTime)]
        pub unsafe fn startTime(&self) -> c_double;

        #[method(duration)]
        pub unsafe fn duration(&self) -> c_double;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithText:confidence:startTime:duration:)]
        pub unsafe fn initWithText_confidence_startTime_duration(
            this: Allocated<Self>,
            text: &NSString,
            confidence: c_double,
            start_time: c_double,
            duration: c_double,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech__SFToken")]
    unsafe impl _SFToken {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

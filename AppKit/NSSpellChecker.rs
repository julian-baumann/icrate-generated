//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_enum!(
    pub type NSTextCheckingOptionKey = NSString;
);

extern_static!(NSTextCheckingOrthographyKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingQuotesKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingReplacementsKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingReferenceDateKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingReferenceTimeZoneKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingDocumentURLKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingDocumentTitleKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingDocumentAuthorKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingRegularExpressionsKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingSelectedRangeKey: &'static NSTextCheckingOptionKey);

extern_static!(NSTextCheckingGenerateInlinePredictionsKey: &'static NSTextCheckingOptionKey);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSCorrectionResponse {
        NSCorrectionResponseNone = 0,
        NSCorrectionResponseAccepted = 1,
        NSCorrectionResponseRejected = 2,
        NSCorrectionResponseIgnored = 3,
        NSCorrectionResponseEdited = 4,
        NSCorrectionResponseReverted = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSCorrectionIndicatorType {
        NSCorrectionIndicatorTypeDefault = 0,
        NSCorrectionIndicatorTypeReversion = 1,
        NSCorrectionIndicatorTypeGuesses = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSpellChecker")]
    pub struct NSSpellChecker;

    #[cfg(feature = "AppKit_NSSpellChecker")]
    unsafe impl ClassType for NSSpellChecker {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSSpellChecker")]
unsafe impl NSObjectProtocol for NSSpellChecker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSpellChecker")]
    unsafe impl NSSpellChecker {
        #[method_id(@__retain_semantics Other sharedSpellChecker)]
        pub unsafe fn sharedSpellChecker() -> Id<NSSpellChecker>;

        #[method(sharedSpellCheckerExists)]
        pub unsafe fn sharedSpellCheckerExists() -> bool;

        #[method(uniqueSpellDocumentTag)]
        pub unsafe fn uniqueSpellDocumentTag() -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method(checkSpellingOfString:startingAt:language:wrap:inSpellDocumentWithTag:wordCount:)]
        pub unsafe fn checkSpellingOfString_startingAt_language_wrap_inSpellDocumentWithTag_wordCount(
            &self,
            string_to_check: &NSString,
            starting_offset: NSInteger,
            language: Option<&NSString>,
            wrap_flag: bool,
            tag: NSInteger,
            word_count: *mut NSInteger,
        ) -> NSRange;

        #[cfg(feature = "Foundation_NSString")]
        #[method(checkSpellingOfString:startingAt:)]
        pub unsafe fn checkSpellingOfString_startingAt(
            &self,
            string_to_check: &NSString,
            starting_offset: NSInteger,
        ) -> NSRange;

        #[cfg(feature = "Foundation_NSString")]
        #[method(countWordsInString:language:)]
        pub unsafe fn countWordsInString_language(
            &self,
            string_to_count: &NSString,
            language: Option<&NSString>,
        ) -> NSInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(checkGrammarOfString:startingAt:language:wrap:inSpellDocumentWithTag:details:)]
        pub unsafe fn checkGrammarOfString_startingAt_language_wrap_inSpellDocumentWithTag_details(
            &self,
            string_to_check: &NSString,
            starting_offset: NSInteger,
            language: Option<&NSString>,
            wrap_flag: bool,
            tag: NSInteger,
            details: Option<&mut Option<Id<NSArray<NSDictionary<NSString, AnyObject>>>>>,
        ) -> NSRange;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSOrthography",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other checkString:range:types:options:inSpellDocumentWithTag:orthography:wordCount:)]
        pub unsafe fn checkString_range_types_options_inSpellDocumentWithTag_orthography_wordCount(
            &self,
            string_to_check: &NSString,
            range: NSRange,
            checking_types: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, AnyObject>>,
            tag: NSInteger,
            orthography: Option<&mut Option<Id<NSOrthography>>>,
            word_count: *mut NSInteger,
        ) -> Id<NSArray<NSTextCheckingResult>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSOrthography",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method(requestCheckingOfString:range:types:options:inSpellDocumentWithTag:completionHandler:)]
        pub unsafe fn requestCheckingOfString_range_types_options_inSpellDocumentWithTag_completionHandler(
            &self,
            string_to_check: &NSString,
            range: NSRange,
            checking_types: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, AnyObject>>,
            tag: NSInteger,
            completion_handler: Option<
                &Block<
                    (
                        NSInteger,
                        NonNull<NSArray<NSTextCheckingResult>>,
                        NonNull<NSOrthography>,
                        NSInteger,
                    ),
                    (),
                >,
            >,
        ) -> NSInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method(requestCandidatesForSelectedRange:inString:types:options:inSpellDocumentWithTag:completionHandler:)]
        pub unsafe fn requestCandidatesForSelectedRange_inString_types_options_inSpellDocumentWithTag_completionHandler(
            &self,
            selected_range: NSRange,
            string_to_check: &NSString,
            checking_types: NSTextCheckingTypes,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, AnyObject>>,
            tag: NSInteger,
            completion_handler: Option<
                &Block<(NSInteger, NonNull<NSArray<NSTextCheckingResult>>), ()>,
            >,
        ) -> NSInteger;

        #[cfg(all(
            feature = "AppKit_NSMenu",
            feature = "AppKit_NSView",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method_id(@__retain_semantics Other menuForResult:string:options:atLocation:inView:)]
        pub unsafe fn menuForResult_string_options_atLocation_inView(
            &self,
            result: &NSTextCheckingResult,
            checked_string: &NSString,
            options: Option<&NSDictionary<NSTextCheckingOptionKey, AnyObject>>,
            location: NSPoint,
            view: &NSView,
        ) -> Option<Id<NSMenu>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other userQuotesArrayForLanguage:)]
        pub unsafe fn userQuotesArrayForLanguage(
            &self,
            language: &NSString,
        ) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other userReplacementsDictionary)]
        pub unsafe fn userReplacementsDictionary(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(updateSpellingPanelWithMisspelledWord:)]
        pub unsafe fn updateSpellingPanelWithMisspelledWord(&self, word: &NSString);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(updateSpellingPanelWithGrammarString:detail:)]
        pub unsafe fn updateSpellingPanelWithGrammarString_detail(
            &self,
            string: &NSString,
            detail: &NSDictionary<NSString, AnyObject>,
        );

        #[cfg(feature = "AppKit_NSPanel")]
        #[method_id(@__retain_semantics Other spellingPanel)]
        pub unsafe fn spellingPanel(&self, mtm: MainThreadMarker) -> Id<NSPanel>;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self, mtm: MainThreadMarker) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSPanel")]
        #[method_id(@__retain_semantics Other substitutionsPanel)]
        pub unsafe fn substitutionsPanel(&self, mtm: MainThreadMarker) -> Id<NSPanel>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other substitutionsPanelAccessoryViewController)]
        pub unsafe fn substitutionsPanelAccessoryViewController(
            &self,
            mtm: MainThreadMarker,
        ) -> Option<Id<NSViewController>>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(setSubstitutionsPanelAccessoryViewController:)]
        pub unsafe fn setSubstitutionsPanelAccessoryViewController(
            &self,
            substitutions_panel_accessory_view_controller: Option<&NSViewController>,
        );

        #[method(updatePanels)]
        pub unsafe fn updatePanels(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(ignoreWord:inSpellDocumentWithTag:)]
        pub unsafe fn ignoreWord_inSpellDocumentWithTag(
            &self,
            word_to_ignore: &NSString,
            tag: NSInteger,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other ignoredWordsInSpellDocumentWithTag:)]
        pub unsafe fn ignoredWordsInSpellDocumentWithTag(
            &self,
            tag: NSInteger,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setIgnoredWords:inSpellDocumentWithTag:)]
        pub unsafe fn setIgnoredWords_inSpellDocumentWithTag(
            &self,
            words: &NSArray<NSString>,
            tag: NSInteger,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other guessesForWordRange:inString:language:inSpellDocumentWithTag:)]
        pub unsafe fn guessesForWordRange_inString_language_inSpellDocumentWithTag(
            &self,
            range: NSRange,
            string: &NSString,
            language: Option<&NSString>,
            tag: NSInteger,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other correctionForWordRange:inString:language:inSpellDocumentWithTag:)]
        pub unsafe fn correctionForWordRange_inString_language_inSpellDocumentWithTag(
            &self,
            range: NSRange,
            string: &NSString,
            language: &NSString,
            tag: NSInteger,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other completionsForPartialWordRange:inString:language:inSpellDocumentWithTag:)]
        pub unsafe fn completionsForPartialWordRange_inString_language_inSpellDocumentWithTag(
            &self,
            range: NSRange,
            string: &NSString,
            language: Option<&NSString>,
            tag: NSInteger,
        ) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSOrthography", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other languageForWordRange:inString:orthography:)]
        pub unsafe fn languageForWordRange_inString_orthography(
            &self,
            range: NSRange,
            string: &NSString,
            orthography: Option<&NSOrthography>,
        ) -> Option<Id<NSString>>;

        #[method(closeSpellDocumentWithTag:)]
        pub unsafe fn closeSpellDocumentWithTag(&self, tag: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method(recordResponse:toCorrection:forWord:language:inSpellDocumentWithTag:)]
        pub unsafe fn recordResponse_toCorrection_forWord_language_inSpellDocumentWithTag(
            &self,
            response: NSCorrectionResponse,
            correction: &NSString,
            word: &NSString,
            language: Option<&NSString>,
            tag: NSInteger,
        );

        #[cfg(all(
            feature = "AppKit_NSView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(showCorrectionIndicatorOfType:primaryString:alternativeStrings:forStringInRect:view:completionHandler:)]
        pub unsafe fn showCorrectionIndicatorOfType_primaryString_alternativeStrings_forStringInRect_view_completionHandler(
            &self,
            r#type: NSCorrectionIndicatorType,
            primary_string: &NSString,
            alternative_strings: &NSArray<NSString>,
            rect_of_typed_string: NSRect,
            view: &NSView,
            completion_block: Option<&Block<(*mut NSString,), ()>>,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method(dismissCorrectionIndicatorForView:)]
        pub unsafe fn dismissCorrectionIndicatorForView(&self, view: &NSView);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSTextCheckingResult"
        ))]
        #[method(showInlinePredictionForCandidates:client:)]
        pub unsafe fn showInlinePredictionForCandidates_client(
            &self,
            candidates: &NSArray<NSTextCheckingResult>,
            client: &ProtocolObject<dyn NSTextInputClient>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(preventsAutocorrectionBeforeString:language:)]
        pub unsafe fn preventsAutocorrectionBeforeString_language(
            &self,
            string: &NSString,
            language: Option<&NSString>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(deletesAutospaceBetweenString:andString:language:)]
        pub unsafe fn deletesAutospaceBetweenString_andString_language(
            &self,
            preceding_string: &NSString,
            following_string: &NSString,
            language: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other availableLanguages)]
        pub unsafe fn availableLanguages(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other userPreferredLanguages)]
        pub unsafe fn userPreferredLanguages(&self) -> Id<NSArray<NSString>>;

        #[method(automaticallyIdentifiesLanguages)]
        pub unsafe fn automaticallyIdentifiesLanguages(&self) -> bool;

        #[method(setAutomaticallyIdentifiesLanguages:)]
        pub unsafe fn setAutomaticallyIdentifiesLanguages(
            &self,
            automatically_identifies_languages: bool,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(setWordFieldStringValue:)]
        pub unsafe fn setWordFieldStringValue(&self, string: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(learnWord:)]
        pub unsafe fn learnWord(&self, word: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(hasLearnedWord:)]
        pub unsafe fn hasLearnedWord(&self, word: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(unlearnWord:)]
        pub unsafe fn unlearnWord(&self, word: &NSString);

        #[method(isAutomaticTextReplacementEnabled)]
        pub unsafe fn isAutomaticTextReplacementEnabled() -> bool;

        #[method(isAutomaticSpellingCorrectionEnabled)]
        pub unsafe fn isAutomaticSpellingCorrectionEnabled() -> bool;

        #[method(isAutomaticQuoteSubstitutionEnabled)]
        pub unsafe fn isAutomaticQuoteSubstitutionEnabled() -> bool;

        #[method(isAutomaticDashSubstitutionEnabled)]
        pub unsafe fn isAutomaticDashSubstitutionEnabled() -> bool;

        #[method(isAutomaticCapitalizationEnabled)]
        pub unsafe fn isAutomaticCapitalizationEnabled() -> bool;

        #[method(isAutomaticPeriodSubstitutionEnabled)]
        pub unsafe fn isAutomaticPeriodSubstitutionEnabled() -> bool;

        #[method(isAutomaticTextCompletionEnabled)]
        pub unsafe fn isAutomaticTextCompletionEnabled() -> bool;

        #[method(isAutomaticInlinePredictionEnabled)]
        pub unsafe fn isAutomaticInlinePredictionEnabled() -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other language)]
        pub unsafe fn language(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLanguage:)]
        pub unsafe fn setLanguage(&self, language: &NSString) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSSpellChecker")]
    unsafe impl NSSpellChecker {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(NSSpellCheckerDidChangeAutomaticSpellingCorrectionNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticTextReplacementNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticQuoteSubstitutionNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticDashSubstitutionNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticCapitalizationNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticPeriodSubstitutionNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticTextCompletionNotification: &'static NSNotificationName);

extern_static!(NSSpellCheckerDidChangeAutomaticInlinePredictionNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSpellChecker")]
    unsafe impl NSSpellChecker {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use -guessesForWordRange:inString:language:inSpellDocumentWithTag instead"]
        #[method_id(@__retain_semantics Other guessesForWord:)]
        pub unsafe fn guessesForWord(&self, word: Option<&NSString>) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(forgetWord:)]
        pub unsafe fn forgetWord(&self, word: Option<&NSString>);
    }
);

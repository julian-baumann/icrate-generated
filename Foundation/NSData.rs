//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataReadingOptions {
        NSDataReadingMappedIfSafe = 1 << 0,
        NSDataReadingUncached = 1 << 1,
        NSDataReadingMappedAlways = 1 << 3,
        #[deprecated]
        NSDataReadingMapped = NSDataReadingMappedIfSafe,
        #[deprecated]
        NSMappedRead = NSDataReadingMapped,
        #[deprecated]
        NSUncachedRead = NSDataReadingUncached,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataWritingOptions {
        NSDataWritingAtomic = 1 << 0,
        NSDataWritingWithoutOverwriting = 1 << 1,
        NSDataWritingFileProtectionNone = 0x10000000,
        NSDataWritingFileProtectionComplete = 0x20000000,
        NSDataWritingFileProtectionCompleteUnlessOpen = 0x30000000,
        NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication = 0x40000000,
        NSDataWritingFileProtectionCompleteWhenUserInactive = 0x50000000,
        NSDataWritingFileProtectionMask = 0xf0000000,
        #[deprecated]
        NSAtomicWrite = NSDataWritingAtomic,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataSearchOptions {
        NSDataSearchBackwards = 1 << 0,
        NSDataSearchAnchored = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataBase64EncodingOptions {
        NSDataBase64Encoding64CharacterLineLength = 1 << 0,
        NSDataBase64Encoding76CharacterLineLength = 1 << 1,
        NSDataBase64EncodingEndLineWithCarriageReturn = 1 << 4,
        NSDataBase64EncodingEndLineWithLineFeed = 1 << 5,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSDataBase64DecodingOptions {
        NSDataBase64DecodingIgnoreUnknownCharacters = 1 << 0,
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSData")]
    pub struct NSData;

    #[cfg(feature = "Foundation_NSData")]
    unsafe impl ClassType for NSData {
        type Super = NSObject;
        type Mutability = ImmutableWithMutableSubclass<NSMutableData>;
    }
);

#[cfg(feature = "Foundation_NSData")]
unsafe impl NSCoding for NSData {}

#[cfg(feature = "Foundation_NSData")]
unsafe impl NSCopying for NSData {}

#[cfg(feature = "Foundation_NSData")]
unsafe impl NSMutableCopying for NSData {}

#[cfg(feature = "Foundation_NSData")]
unsafe impl NSObjectProtocol for NSData {}

#[cfg(feature = "Foundation_NSData")]
unsafe impl NSSecureCoding for NSData {}

extern_methods!(
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[method(length)]
        pub fn length(&self) -> NSUInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSData")]
impl DefaultId for NSData {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedData
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other description)]
        pub unsafe fn description(&self) -> Id<NSString>;

        #[method(getBytes:length:)]
        pub unsafe fn getBytes_length(&self, buffer: NonNull<c_void>, length: NSUInteger);

        #[method(getBytes:range:)]
        pub unsafe fn getBytes_range(&self, buffer: NonNull<c_void>, range: NSRange);

        #[method(isEqualToData:)]
        pub unsafe fn isEqualToData(&self, other: &NSData) -> bool;

        #[method_id(@__retain_semantics Other subdataWithRange:)]
        pub unsafe fn subdataWithRange(&self, range: NSRange) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            use_auxiliary_file: bool,
        ) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(writeToFile:options:error:_)]
        pub unsafe fn writeToFile_options_error(
            &self,
            path: &NSString,
            write_options_mask: NSDataWritingOptions,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(writeToURL:options:error:_)]
        pub unsafe fn writeToURL_options_error(
            &self,
            url: &NSURL,
            write_options_mask: NSDataWritingOptions,
        ) -> Result<(), Id<NSError>>;

        #[method(rangeOfData:options:range:)]
        pub unsafe fn rangeOfData_options_range(
            &self,
            data_to_find: &NSData,
            mask: NSDataSearchOptions,
            search_range: NSRange,
        ) -> NSRange;

        #[method(enumerateByteRangesUsingBlock:)]
        pub unsafe fn enumerateByteRangesUsingBlock(
            &self,
            block: &Block<(NonNull<c_void>, NSRange, NonNull<Bool>), ()>,
        );
    }
);

extern_methods!(
    /// NSDataCreation
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(bytes: *mut c_void, length: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Allocated<Self>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Allocated<Self>,
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL)
            -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub fn initWithData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataCreation
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data() -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(bytes: *mut c_void, length: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:options:error:_)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:options:error:_)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            this: Allocated<Self>,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            this: Allocated<Self>,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:options:error:_)]
        pub unsafe fn initWithContentsOfFile_options_error(
            this: Allocated<Self>,
            path: &NSString,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:options:error:_)]
        pub unsafe fn initWithContentsOfURL_options_error(
            this: Allocated<Self>,
            url: &NSURL,
            read_options_mask: NSDataReadingOptions,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(this: Allocated<Self>, url: &NSURL)
            -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(this: Allocated<Self>, data: &NSData) -> Id<Self>;

        #[method_id(@__retain_semantics Other dataWithData:)]
        pub fn dataWithData(data: &NSData) -> Id<Self>;
    }
);

extern_methods!(
    /// NSDataBase64Encoding
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Allocated<Self>,
            base64_string: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other base64EncodedStringWithOptions:)]
        pub unsafe fn base64EncodedStringWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Id<NSString>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Allocated<Self>,
            base64_data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other base64EncodedDataWithOptions:)]
        pub unsafe fn base64EncodedDataWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Id<NSData>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDataBase64Encoding
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            this: Allocated<Self>,
            base64_string: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            this: Allocated<Self>,
            base64_data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self>>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDataCompressionAlgorithm {
        NSDataCompressionAlgorithmLZFSE = 0,
        NSDataCompressionAlgorithmLZ4 = 1,
        NSDataCompressionAlgorithmLZMA = 2,
        NSDataCompressionAlgorithmZlib = 3,
    }
);

extern_methods!(
    /// NSDataCompression
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other decompressedDataUsingAlgorithm:error:_)]
        pub unsafe fn decompressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other compressedDataUsingAlgorithm:error:_)]
        pub unsafe fn compressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSData")]
    unsafe impl NSData {
        #[deprecated = "This method is unsafe because it could potentially cause buffer overruns. Use -getBytes:length: instead."]
        #[method(getBytes:)]
        pub unsafe fn getBytes(&self, buffer: NonNull<c_void>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use +dataWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Other dataWithContentsOfMappedFile:)]
        pub unsafe fn dataWithContentsOfMappedFile(path: &NSString) -> Option<Id<AnyObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -initWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use initWithBase64EncodedString:options: instead"]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Allocated<Self>,
            base64_string: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use base64EncodedStringWithOptions: instead"]
        #[method_id(@__retain_semantics Other base64Encoding)]
        pub unsafe fn base64Encoding(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSData`
    ///
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -initWithContentsOfURL:options:error: and NSDataReadingMappedIfSafe or NSDataReadingMappedAlways instead."]
        #[method_id(@__retain_semantics Init initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            this: Allocated<Self>,
            path: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use initWithBase64EncodedString:options: instead"]
        #[method_id(@__retain_semantics Init initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            this: Allocated<Self>,
            base64_string: &NSString,
        ) -> Option<Id<Self>>;
    }
);

extern_class!(
    #[derive(PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSData")]
    pub struct NSMutableData;

    #[cfg(feature = "Foundation_NSData")]
    unsafe impl ClassType for NSMutableData {
        #[inherits(NSObject)]
        type Super = NSData;
        type Mutability = MutableWithImmutableSuperclass<NSData>;
    }
);

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSCoding for NSMutableData {}

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSCopying for NSMutableData {}

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSMutableCopying for NSMutableData {}

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSObjectProtocol for NSMutableData {}

#[cfg(feature = "Foundation_NSMutableData")]
unsafe impl NSSecureCoding for NSMutableData {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method(setLength:)]
        pub fn setLength(&mut self, length: NSUInteger);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Init init)]
        pub fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub fn new() -> Id<Self>;
    }
);

#[cfg(feature = "Foundation_NSMutableData")]
impl DefaultId for NSMutableData {
    #[inline]
    fn default_id() -> Id<Self> {
        Self::new()
    }
}

extern_methods!(
    /// NSExtendedMutableData
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method(appendBytes:length:)]
        pub unsafe fn appendBytes_length(&mut self, bytes: NonNull<c_void>, length: NSUInteger);

        #[cfg(feature = "Foundation_NSData")]
        #[method(appendData:)]
        pub unsafe fn appendData(&mut self, other: &NSData);

        #[method(increaseLengthBy:)]
        pub unsafe fn increaseLengthBy(&mut self, extra_length: NSUInteger);

        #[method(replaceBytesInRange:withBytes:)]
        pub unsafe fn replaceBytesInRange_withBytes(
            &mut self,
            range: NSRange,
            bytes: NonNull<c_void>,
        );

        #[method(resetBytesInRange:)]
        pub unsafe fn resetBytesInRange(&mut self, range: NSRange);

        #[cfg(feature = "Foundation_NSData")]
        #[method(setData:)]
        pub unsafe fn setData(&mut self, data: &NSData);

        #[method(replaceBytesInRange:withBytes:length:)]
        pub unsafe fn replaceBytesInRange_withBytes_length(
            &mut self,
            range: NSRange,
            replacement_bytes: *mut c_void,
            replacement_length: NSUInteger,
        );
    }
);

extern_methods!(
    /// NSMutableDataCreation
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[method_id(@__retain_semantics Other dataWithCapacity:)]
        pub fn dataWithCapacity(a_num_items: NSUInteger) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other dataWithLength:)]
        pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithCapacity:)]
        pub fn initWithCapacity(this: Allocated<Self>, capacity: NSUInteger) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithLength:)]
        pub unsafe fn initWithLength(this: Allocated<Self>, length: NSUInteger)
            -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// NSMutableDataCompression
    #[cfg(feature = "Foundation_NSMutableData")]
    unsafe impl NSMutableData {
        #[cfg(feature = "Foundation_NSError")]
        #[method(decompressUsingAlgorithm:error:_)]
        pub unsafe fn decompressUsingAlgorithm_error(
            &mut self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(compressUsingAlgorithm:error:_)]
        pub unsafe fn compressUsingAlgorithm_error(
            &mut self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Id<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPurgeableData")]
    pub struct NSPurgeableData;

    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl ClassType for NSPurgeableData {
        #[inherits(NSData, NSObject)]
        type Super = NSMutableData;
        type Mutability = Mutable;
    }
);

#[cfg(feature = "Foundation_NSPurgeableData")]
unsafe impl NSCoding for NSPurgeableData {}

#[cfg(feature = "Foundation_NSPurgeableData")]
unsafe impl NSDiscardableContent for NSPurgeableData {}

#[cfg(feature = "Foundation_NSPurgeableData")]
unsafe impl NSObjectProtocol for NSPurgeableData {}

#[cfg(feature = "Foundation_NSPurgeableData")]
unsafe impl NSSecureCoding for NSPurgeableData {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {}
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSPurgeableData")]
    unsafe impl NSPurgeableData {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

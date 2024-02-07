declare namespace Win32Props {

    /**
     * Gets data for all available properties of a file
     * @param file file's fullpath
     * @returns Object whose key is property name and value is formatted property value.
    */
    function read(file:string): Promise<Property>;

    /**
     * Gets data for a property of a file
     * @param file file's fullpath
     * @param propertyName property name
     * @returns Object whose key is file's fullpath and value is file's property value.
    */
    function getValue(file:string, propertyName:PropertyName): Promise<{[file:string]:string}>;

    /**
     * Gets data for a property of files
     * Files must be in the same folder
     * @param files Array of file's fullpath
     * @param propertyName property name
     * @returns Object whose key is file's fullpath and value is file's property value.
    */
    function getValues(files:string[], propertyName:PropertyName): Promise<{[file:string]:string}>;

    /**
     * Sets a property value of a file
     * If the file is in use, process fails
     * @param file file's fullpath
     * @param propertyName property name
     * @param propertyValue value to set
     * @returns True when comment is written sucuseccfully, false when error occurs.
    */
    function setValue(file:string, propertyName:PropertyName, propertyValue:string): Promise<boolean>;

    type PropertyName = "AppUserModelID"
        | "AppUserModelParentID"
        | "AppZoneIdentifier"
        | "AudioChannelCount"
        | "AudioEncodingBitrate"
        | "AudioFormat"
        | "AudioSampleRate"
        | "AudioSampleSize"
        | "AudioStreamNumber"
        | "Author"
        | "Comment"
        | "ComputerName"
        | "ContentType"
        | "DRMIsProtected"
        | "DateAccessed"
        | "DateCreated"
        | "DateImported"
        | "DateModified"
        | "DocumentDateCreated"
        | "DocumentDateSaved"
        | "ExpandoProperties"
        | "FileAttributes"
        | "FileAttributesDisplay"
        | "FileExtension"
        | "FileName"
        | "FileOwner"
        | "FilePlaceholderStatus"
        | "IsFolder"
        | "IsShared"
        | "ItemAuthors"
        | "ItemDate"
        | "ItemFolderNameDisplay"
        | "ItemFolderPathDisplay"
        | "ItemFolderPathDisplayNarrow"
        | "ItemName"
        | "ItemNameDisplay"
        | "ItemNameDisplayWithoutExtension"
        | "ItemParticipants"
        | "ItemPathDisplay"
        | "ItemPathDisplayNarrow"
        | "ItemType"
        | "ItemTypeText"
        | "Kind"
        | "KindText"
        | "LastWriterPackageFamilyName"
        | "LinkTargetExtension"
        | "LinkTargetParsingPath"
        | "LinkTargetSFGAOFlags"
        | "LinkTargetSFGAOFlagsStrings"
        | "MIMEType"
        | "MediaDuration"
        | "MusicAlbumID"
        | "MusicAlbumTitle"
        | "MusicDisplayArtist"
        | "NetworkLocation"
        | "NotUserContent"
        | "OfflineAvailability"
        | "OfflineStatus"
        | "ParsingName"
        | "ParsingPath"
        | "PerceivedType"
        | "SFGAOFlags"
        | "SecurityAllowedEnterpriseDataProtectionIdentities"
        | "SecurityEncryptionOwners"
        | "SecurityEncryptionOwnersDisplay"
        | "ShareScope"
        | "SharedWith"
        | "SharingStatus"
        | "ShellSFGAOFlagsStrings"
        | "Size"
        | "StorageProviderAggregatedCustomStates"
        | "SyncTransferStatusFlags"
        | "ThumbnailCacheId"
        | "Title"
        | "VideoCompression"
        | "VideoEncodingBitrate"
        | "VideoFourCC"
        | "VideoFrameHeight"
        | "VideoFrameRate"
        | "VideoFrameWidth"
        | "VideoHorizontalAspectRatio"
        | "VideoIsSpherical"
        | "VideoIsStereo"
        | "VideoOrientation"
        | "VideoStreamNumber"
        | "VideoTotalBitrate"
        | "VideoVerticalAspectRatio"
        | "VolumeId"
        | "ZoneIdentifier";

    interface Property {
        [key: string]: any;
        AppUserModelID?: string;
        AppUserModelParentID?: string;
        AppZoneIdentifier?: string;
        AudioChannelCount?: string;
        AudioEncodingBitrate?: string;
        AudioFormat?: string;
        AudioSampleRate?: string;
        AudioSampleSize?: string;
        AudioStreamNumber?: string;
        Author?: string;
        Comment?: string;
        ComputerName?: string;
        ContentType?: string;
        DRMIsProtected?: string;
        DateAccessed?: string;
        DateCreated?: string;
        DateImported?: string;
        DateModified?: string;
        DocumentDateCreated?: string;
        DocumentDateSaved?: string;
        ExpandoProperties?: string;
        FileAttributes?: string;
        FileAttributesDisplay?: string;
        FileExtension?: string;
        FileName?: string;
        FileOwner?: string;
        FilePlaceholderStatus?: string;
        IsFolder?: string;
        IsShared?: string;
        ItemAuthors?: string;
        ItemDate?: string;
        ItemFolderNameDisplay?: string;
        ItemFolderPathDisplay?: string;
        ItemFolderPathDisplayNarrow?: string;
        ItemName?: string;
        ItemNameDisplay?: string;
        ItemNameDisplayWithoutExtension?: string;
        ItemParticipants?: string;
        ItemPathDisplay?: string;
        ItemPathDisplayNarrow?: string;
        ItemType?: string;
        ItemTypeText?: string;
        Kind?: string;
        KindText?: string;
        LastWriterPackageFamilyName?: string;
        LinkTargetExtension?: string;
        LinkTargetParsingPath?: string;
        LinkTargetSFGAOFlags?: string;
        LinkTargetSFGAOFlagsStrings?: string;
        MIMEType?: string;
        MediaDuration?: string;
        MusicAlbumID?: string;
        MusicAlbumTitle?: string;
        MusicDisplayArtist?: string;
        NetworkLocation?: string;
        NotUserContent?: string;
        OfflineAvailability?: string;
        OfflineStatus?: string;
        ParsingName?: string;
        ParsingPath?: string;
        PerceivedType?: string;
        SFGAOFlags?: string;
        SecurityAllowedEnterpriseDataProtectionIdentities?: string;
        SecurityEncryptionOwners?: string;
        SecurityEncryptionOwnersDisplay?: string;
        ShareScope?: string;
        SharedWith?: string;
        SharingStatus?: string;
        ShellSFGAOFlagsStrings?: string;
        Size?: string;
        StorageProviderAggregatedCustomStates?: string;
        SyncTransferStatusFlags?: string;
        ThumbnailCacheId?: string;
        Title?: string;
        VideoCompression?: string;
        VideoEncodingBitrate?: string;
        VideoFourCC?: string;
        VideoFrameHeight?: string;
        VideoFrameRate?: string;
        VideoFrameWidth?: string;
        VideoHorizontalAspectRatio?: string;
        VideoIsSpherical?: string;
        VideoIsStereo?: string;
        VideoOrientation?: string;
        VideoStreamNumber?: string;
        VideoTotalBitrate?: string;
        VideoVerticalAspectRatio?: string;
        VolumeId?: string;
        ZoneIdentifier?: string;
    }
}

export = Win32Props;
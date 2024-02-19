declare namespace Win32Props {

    /**
     * Gets data for all available properties of a file
     * @param file File's fullpath.
     * @param format Whether to format property value or not. Default is false.
     * @returns Object whose key is property name and value is formatted property value.
    */
    function read(file:string, format?:boolean): Promise<Property>;

    /**
     * Gets data for a property of a file
     * @param file File's fullpath
     * @param propertyName Property name
     * @returns File's property value
    */
    function getValue(file:string, propertyName:PropertyName): Promise<string>;

    /**
     * Gets data for a property of files. Files must be in the same folder
     * @param files Array of file's fullpath
     * @param propertyName Property name
     * @returns Object whose key is file's fullpath and value is file's property value.
    */
    function getValues(files:string[], propertyName:PropertyName): Promise<{[file:string]:string}>;

    /**
     * Sets a property value of a file. If the file is in use, process fails
     * @param file File's fullpath
     * @param propertyName Property name
     * @param propertyValue Value to set
     * @returns True when value is set, false when error occurs.
    */
    function setValue(file:string, propertyName:PropertyName, propertyValue:string): Promise<boolean>;

    type Property = {
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
        Rating?:string;
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

    type PropertyName = keyof Property;
}

export = Win32Props;
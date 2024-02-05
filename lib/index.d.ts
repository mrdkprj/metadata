declare namespace Win32Props {

    /**
     * Get all available metadata of a file
     * @param file file's fullpath
     * @returns Object whose key is property name and value is formatted property value.
    */
    function read(file:string): Promise<{[key:string]:string}>;

    /**
     * Get comment metadata of files
     * @param files Array of file's fullpath
     * @returns Object whose key is file's fullpath and value is file's comment metadata string.
    */
    function getComments(files:string[]): Promise<{[path:string]:string}>;

    /**
     * Set comment metadata of a file
     * @param file file's fullpath
     * @param comment file's comment
     * @returns True when comment is written sucuseccfully, false when error occurs.
    */
    function setComment(file:string, comment:string): Promise<boolean>;

}

export = Win32Props;
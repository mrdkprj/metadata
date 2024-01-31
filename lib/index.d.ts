declare namespace Metadata {

    /**
     * Get comment metadata of files
     * @param files Array of file's fullpath
     * @returns Object whose key is file's fullpath and value is file's comment metadata string.
     */
    function getComments(files:string[]): {[path:string]:string};

    /**
     * Set comment metadata of a file
     * @param file file's fullpath
     * @param comment file's comment
     * @returns True when comment is written sucuseccfully, false when error occurs.
     */
    function setComment(file:string, comment:string): boolean;

}

export = Metadata;
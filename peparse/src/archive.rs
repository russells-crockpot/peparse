use crate::error::Error;
use segsource::TryFromSegment;

#[derive(TryFromSegment)]
#[from_seg(error(crate::Error))]
pub struct ArchiveMember {
    /// The name of the archive member, with a slash (/) appended to terminate the name. If the
    /// first character is a slash, the name has a special interpretation, as described in the
    /// following table.
    pub name: [u8; 16],

    /// The date and time that the archive member was created: This is the ASCII decimal
    /// representation of the number of seconds since 1/1/1970 UCT.
    pub date: [u8; 12],

    /// An ASCII decimal representation of the user ID. This field does not contain a meaningful
    /// value on Windows platforms because Microsoft tools emit all blanks.
    pub user_id: [u8; 6],

    /// An ASCII decimal representation of the group ID. This field does not contain a meaningful
    /// value on Windows platforms because Microsoft tools emit all blanks.
    pub group_id: [u8; 6],

    /// An ASCII octal representation of the member's file mode. This is the ST_MODE value from the
    /// C run-time function _wstat.
    pub mode: u64,

    /// An ASCII decimal representation of the total size of the archive member, not including the
    /// size of the header.
    pub size: [u8; 10],

    /// The two bytes in the C string "˜\n" (0x60 0x0A).
    //TODO add error_if
    pub end_of_header: u16,
}

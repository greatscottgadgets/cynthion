//! Error and Result types

/// libgreat [`Result`] type.
pub type GreatResult<T> = core::result::Result<T, GreatError>;

/// Provides a uniform set of error codes common to all libgreat
/// implementations, and that are consistent over platform and RPC
/// boundaries.
///
/// Derived from: [libgreat/firmware/include/errno.h](https://github.com/greatscottgadgets/libgreat/blob/master/firmware/include/errno.h)
#[cfg(feature = "errno_minimal")]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
#[rustfmt::skip]
pub enum GreatError {
    IoError = 5,                           // EIO             - I/O error
    ArgumentListTooLong = 7,               // E2BIG           - Arg list too long
    OperationWouldBlock = 11,              // EWOULDBLOCK     - Operation would block
    NotEnoughSpace = 12,                   // ENOMEM          - Not enough space
    DeviceOrResourceBusy = 16,             // EBUSY           - Device or resource busy
    InvalidArgument = 22,                  // EINVAL          - Invalid argument
    ArgumentOutOfRange = 33,               // EDOM            - Mathematics argument out of domain of function
    ResultTooLarge = 34,                   // ERANGE          - Result too large
    NoMessageOfType = 35,                  // ENOMSG          - No message of desired type
    InvalidRequestDescriptor = 51,         // EBADR           - Invalid request descriptor
    InvalidRequestCode = 54,               // EBADRQC         - Invalid request code
    NoData = 61,                           // ENODATA         - No data
    ProtocolError = 71,                    // EPROTO          - Protocol error
    BadMessage = 77,                       // EBADMSG         - Bad message
    FunctionNotImplemented = 88,           // ENOSYS          - Function not implemented
    ConnectionResetByPeer = 104,           // ECONNRESET      - Connection reset by peer
    NoBufferSpaceAvailable = 105,          // ENOBUFS         - No buffer space available
    ConnectionRefused = 111,               // ECONNREFUSED    - Connection refused
    AddressAlreadyInUse = 112,             // EADDRINUSE      - Address already in use
    ConnectionTimedOut = 116,              // ETIMEDOUT       - Connection timed out
    OperationAlreadyInProgress = 120,      // EALREADY        - Operation already in progress
    AddressNotAvailable = 125,             // EADDRNOTAVAIL   - Address not available
    NotSupported = 134,                    // ENOTSUP         - Not supported
    IllegalByteSequence = 138,             // EILSEQ          - Illegal byte sequence
    ValueTooLargeForDefinedDataType = 139, // EOVERFLOW       - Value too large for defined data type
    OperationCanceled = 140,               // ECANCELED       - Operation canceled
    StateNotRecoverable = 141,             // ENOTRECOVERABLE - State not recoverable
}

#[cfg(not(feature = "errno_minimal"))]
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
#[rustfmt::skip]
pub enum GreatError {
    NotOwner = 1,                          // EPERM           - Not owner
    NoSuchFileOrDirectory = 2,             // ENOENT          - No such file or directory
    NoSuchProcess = 3,                     // ENOSRCH         - No such process
    InterruptedSystemCall = 4,             // EINTR           - Interrupted system call
    IoError = 5,                           // EIO             - I/O error
    NoSuchDeviceOrAddress = 6,             // ENXIO           - No such device or address
    ArgumentListTooLong = 7,               // E2BIG           - Arg list too long
    ExecFormatError = 8,                   // ENOEXEC         - Exec format error
    BadFileNumber = 9,                     // EBADF           - Bad file number
    NoChildren = 10,                       // ECHILD          - No children
    OperationWouldBlock = 11,              // EWOULDBLOCK     - Operation would block
    NotEnoughSpace = 12,                   // ENOMEM          - Not enough space
    PermissionDenied = 13,                 // EACCES          - Permission denied
    BadAddress = 14,                       // EFAULT          - Bad address
    BlockDeviceRequired = 15,              // ENOTBLK         - Block device required
    DeviceOrResourceBusy = 16,             // EBUSY           - Device or resource busy
    FileExists = 17,                       // EEXIST          - File exists
    CrossDeviceLink = 18,                  // EXDEV           - Cross-device link
    NoSuchDevice = 19,                     // ENODEV          - No such device
    NotDirectory = 20,                     // ENOTDIR         - Not a directory
    IsDirectory = 21,                      // EISDIR          - Is a directory
    InvalidArgument = 22,                  // EINVAL          - Invalid argument
    TooManyOpenFiles = 23,                 // ENFILE          - Too many open files in system
    FileDescriptorTooLarge = 24,           // EMFILE          - File descriptor value too large
    NotCharacterDevice = 25,               // ENOTTY          - Not a character device
    TextFileBusy = 26,                     // ETXTBSY         - Text file busy
    FileTooLarge = 27,                     // EFBIG           - File too large
    NoSpaceLeftOnDevice = 28,              // ENOSPC          - No space left on device
    IllegalSeek = 29,                      // ESPIPE          - Illegal seek
    ReadOnlyFileSystem = 30,               // EROFS           - Read-only file system
    TooManyLink = 31,                      // EMLINK          - Too many links
    BrokenPipe = 32,                       // EPIPE           - Broken pipe
    ArgumentOutOfRange = 33,               // EDOM            - Mathematics argument out of domain of function
    ResultTooLarge = 34,                   // ERANGE          - Result too large
    NoMessageOfType = 35,                  // ENOMSG          - No message of desired type
    IdentifierRemoved = 36,                // EIDRM           - Identifier removed
    ChannelOutOfRange = 37,                // ECHRNG          - Channel number out of range
    LevelTwoNotSynchronized = 38,          // EL2NSYNC        - Level 2 not synchronized
    LevelThreeHalted = 39,                 // EL3HLT          - Level 3 halted
    LevelThreeReset = 40,                  // EL3RST          - Level 3 reset
    LinkNumberOutOfRange = 41,             // ELNRNG          - Link number out of range
    ProtocolDriverNotAttached = 42,        // EUNATCH         - Protocol driver not attached
    NoCsiStructureAvailable = 43,          // ENOCSI          - No CSI structure available
    LevelTwoHalted = 44,                   // EL2HLT          - Level 2 halted
    Deadlock = 45,                         // EDEADLK         - Deadlock
    NoLock = 46,                           // ENOLCK          - No lock
    InvalidExchange = 50,                  // EBADE           - Invalid exchange
    InvalidRequestDescriptor = 51,         // EBADR           - Invalid request descriptor
    ExchangeFull = 52,                     // EXFULL          - Exchange full
    NoAnode = 53,                          // ENOANO          - No anode
    InvalidRequestCode = 54,               // EBADRQC         - Invalid request code
    InvalidSlot = 55,                      // EBADSLT         - Invalid slot
    FileLockingDeadLockError = 56,         // EDEADLOCK       - File locking deadlock error
    BadFontFileFormat = 57,                // EBFONT          - Bad font file fmt
    NotStream = 60,                        // ENOSTR          - Not a stream
    NoData = 61,                           // ENODATA         - No data
    StreamIoctlTimeout = 62,               // ETIME           - Stream ioctl timeout
    NoStreamResources = 63,                // ENOSR           - No stream resources
    NoNetwork = 64,                        // ENONET          - Machine is not on the network
    PackageNotInstalled = 65,              // ENOPKG          - Package not installed
    ObjectIsRemote = 66,                   // EREMOTE         - The object is remote
    VirtualCircuitGone = 67,               // ENOLINK         - Virtual circuit is gone
    AdvertiseError = 68,                   // EADV            - Advertise error
    SrmountError = 69,                     // ESRMNT          - Srmount error
    CommunicationErrorOnSend = 70,         // ECOMM           - Communication error on send
    ProtocolError = 71,                    // EPROTO          - Protocol error
    MultihopAttempted = 72,                // EMULTIHOP       - Multihop attempted
    InodeRemote = 75,                      // ELBIN           - Inode is remote (not really error)
    CrossMountPoint = 76,                  // EDOTDOT         - Cross mount point (not really error)
    BadMessage = 77,                       // EBADMSG         - Bad message
    WrongFileTypeOrFromat = 79,            // EFTYPE          - Inappropriate file type or format
    GivenNameNotUnique = 80,               // ENOTUNIQ        - Given log. name not unique
    InvalidFileDescriptor = 81,            // EBADFD          - f.d. invalid for this operation
    RemoteAddressChanged = 82,             // EREMCHG         - Remote address changed
    CantAccessLibrary = 83,                // ELIBACC         - Can't access a needed shared lib
    CorruptedLibrary = 84,                 // ELIBBAD         - Accessing a corrupted shared lib
    LibrarySectionCorrupted = 85,          // ELIBSCN         - .lib section in a.out corrupted
    LinkLimitExceeded = 86,                // ELIBMAX         - Attempting to link in too many libs
    InvalidExecutable = 87,                // ELIBEXEC        - Attempting to exec a shared library
    FunctionNotImplemented = 88,           // ENOSYS          - Function not implemented
    NoMoreFiles = 89,                      // ENMFILE         - No more files
    DirectoryNotEmpty = 90,                // ENOTEMPTY       - Directory not empty
    NameTooLong = 91,                      // ENAMETOOLONG    - File or path name too long
    TooManySymbolicLinks = 92,             // ELOOP           - Too many symbolic links
    SocketOperationNotSupported = 95,      // EOPNOTSUPP      - Operation not supported on socket
    ProtocolFamilyNotSupported = 96,       // EPFNOSUPPORT    - Protocol family not supported
    ConnectionResetByPeer = 104,           // ECONNRESET      - Connection reset by peer
    NoBufferSpaceAvailable = 105,          // ENOBUFS         - No buffer space available
    AddressFamilyNotSupported = 106,       // EAFNOSUPPORT    - Address family not supported by protocol family
    WrongProtocolType = 107,               // EPROTOTYPE      - Protocol wrong type for socket
    SocketOperationOnNonSocket = 108,      // ENOTSOCK        - Socket operation on non-socket
    ProtocolNotAvailable = 109,            // ENOPROTOOPT     - Protocol not available
    CantSendAfterSocketShutdown = 110,     // ESHUTDOWN       - Can't send after socket shutdown
    ConnectionRefused = 111,               // ECONNREFUSED    - Connection refused
    AddressAlreadyInUse = 112,             // EADDRINUSE      - Address already in use
    SoftwareCausedConnectionAbort = 113,   // ECONNABORTED    - Software caused connection abort
    NetworkIsUnreachable = 114,            // ENETUNREACH     - Network is unreachable
    NetworkInterfaceIsNotConfigured = 115, // ENETDOWN        - Network interface is not configured
    ConnectionTimedOut = 116,              // ETIMEDOUT       - Connection timed out
    HostIsDown = 117,                      // EHOSTDOWN       - Host is down
    HostIsUnreachable = 118,               // EHOSTUNREACH    - Host is unreachable
    ConnectionAlreadyInProgress = 119,     // EINPROGRESS     - Connection already in progress
    OperationAlreadyInProgress = 120,      // EALREADY        - Operation already in progress
    DestinationAddressRequired = 121,      // EDESTADDRREQ    - Destination address required
    MessageTooLong = 122,                  // EMSGSIZE        - Message too long
    UnknownProtocol = 123,                 // EPROTONOSUPPORT - Unknown protocol
    SocketTypeNotSupported = 124,          // ESOCKTNOSUPPORT - Socket type not supported
    AddressNotAvailable = 125,             // EADDRNOTAVAIL   - Address not available
    ConnectionAbortedByNetwork = 126,      // ENETRESET       - Connection aborted by network
    SocketAlreadyConnected = 127,          // EISCONN         - Socket is already connected
    SocketNotConnected = 128,              // ENOTCONN        - Socket is not connected
    TooManyReferences = 129,               // ETOOMANYREFS    - Too many references: cannot splice
    ProcessLimitExceeded = 130,            // EPROCLIM        - Process limit exceeded
    TooManyUSers = 131,                    // EUSERS          - Too many users
    QuotaExeeded = 132,                    // EDQUOT          - Quota Exceeded
    StaleNfsFileHandle = 133,              // ESTALE          - Stale NFS file handle
    NotSupported = 134,                    // ENOTSUP         - Not supported
    NoMedium = 135,                        // ENOMEDIUM       - No medium (in tape drive)
    NoSuchHostOrNetworkPath = 136,         // ENOSHARE        - No such host or network path
    FilenameExistsWithDifferentCase = 137, // ECASECLASH      - Filename exists with different case
    IllegalByteSequence = 138,             // EILSEQ          - Illegal byte sequence
    ValueTooLargeForDefinedDataType = 139, // EOVERFLOW       - Value too large for defined data type
    OperationCanceled = 140,               // ECANCELED       - Operation canceled
    StateNotRecoverable = 141,             // ENOTRECOVERABLE - State not recoverable
    PreviousOwnerDied = 142,               // EOWNERDEAD      - Previous owner died
    StreamsPipeError = 143,                // ESTRPIPE        - Streams pipe error
}

impl core::fmt::Display for GreatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self, f)
    }
}

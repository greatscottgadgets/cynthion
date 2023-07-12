#
# This file is part of Cynthion
#

class DeviceNotFoundError(IOError):
    """ Error indicating no Cynthion device was found. """
    pass


class DeviceBusyError(IOError):
    """ Error indicating the Cynthion is too busy to service the given request. """
    pass


class DeviceMemoryError(MemoryError):
    """ Error indicating that the Cynthion has run out of memory. """
    pass


class NotFoundError(IOError):
    """ Error indicating that a resource was not found. """
    pass

class CynthionError(RuntimeError):
    """ Runtime error used when no better description is available. """
    pass


class ExternalDeviceError(IOError):
    """
    Error used when a external device (e.g. not on the Cynthion)
    experiences an issue. This typically means that the error is not with
    the Cynthion hardware or software, but may be with e.g. connections.
    """


CYNTHION_ERRORS = {
    -2: ValueError,
    -5: NotFoundError,
    -6: DeviceBusyError,
    -7: MemoryError,
}


def from_Cynthion_error(error_number):
    """
    Returns the error class appropriate for the given Cynthion error.
    """
    error_class = CYNTHION_ERRORS.get(error_number, CynthionError)
    message = "Error {}".format(error_number)
    return error_class(message)

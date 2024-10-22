import usb1
import time
import random

VENDOR_ID  = 0x1209 # https://pid.codes/1209/
PRODUCT_ID = 0x0001

VENDOR_SET_FPGA_LEDS   = 0x01
VENDOR_GET_USER_BUTTON = 0x02

MAX_PACKET_SIZE = 512

# - list available usb devices ------------------------------------------------

def list_available_usb_devices(context):
    for device in context.getDeviceList():
        try:
            manufacturer = device.getManufacturer()
            product = device.getProduct()
            print(f"{device}:  {manufacturer} - {product}")
        except Exception as e:
            print(f"{device}: {e}")


# - wrappers for control requests ---------------------------------------------

def set_fpga_leds(device_handle, led_state):
    response = device_handle.controlWrite(
        request_type = usb1.TYPE_VENDOR | usb1.RECIPIENT_DEVICE,
        request      = VENDOR_SET_FPGA_LEDS,
        index        = 0,
        value        = 0,
        data         = [led_state],
        timeout      = 1000,
    )

def get_user_button(device_handle):
    response = device_handle.controlRead(
        request_type = usb1.TYPE_VENDOR | usb1.RECIPIENT_DEVICE | usb1.ENDPOINT_OUT,
        request      = VENDOR_GET_USER_BUTTON,
        index        = 0,
        value        = 0,
        length       = 1,
        timeout      = 1000,
    )
    return response[0]


# - test control endpoints ----------------------------------------------------

def test_control_endpoints(device_handle):
    led_counter = 0
    last_button_state = False

    while True:
        # led counter
        set_fpga_leds(device_handle, led_counter)
        led_counter = (led_counter + 1) % 256

        # reset led counter when the USER button is pressed
        button_state = get_user_button(device_handle)
        if button_state:
            led_counter = 0

        # print button state when it changes
        if button_state != last_button_state:
            print(f"USER button is: {'ON' if button_state else 'OFF' }")
            last_button_state = button_state

        # slow the loop down so we can see the counter change
        time.sleep(0.1)


# - wrappers for bulk requests ------------------------------------------------

def bulk_out_transfer(device_handle, data):
    response = device_handle.bulkWrite(
        endpoint = 0x01,
        data     = data,
        timeout  = 1000,
    )
    return response

def bulk_in_transfer(device_handle, length):
    response = device_handle.bulkRead(
        endpoint = 0x02,
        length   = length,
        timeout  = 1000,
    )
    return response


# - test bulk endpoints -------------------------------------------------------

def test_bulk_endpoints(device_handle):
    # bulk_out - write a list of random numbers to memory
    data = list([random.randint(0, 255) for _ in range(MAX_PACKET_SIZE)])
    response = bulk_out_transfer(device_handle, data)
    print(f"OUT endpoint transmitted {response} bytes: {data[0:4]} ... {data[-4:]}")

    # bulk_in - retrieve the contents of our memory
    response = list(bulk_in_transfer(device_handle, MAX_PACKET_SIZE))
    print(f"IN  endpoint received {len(response)} bytes:    {response[0:4]} ... {response[-4:]}")

    # check that the stored data matches the sent data
    assert(data == list(response))


# - main ----------------------------------------------------------------------

if __name__ == "__main__":
    with usb1.USBContext() as context:
        # list available devices
        list_available_usb_devices(context)

        # get a device handle to our simple usb device
        device_handle = context.openByVendorIDAndProductID(VENDOR_ID, PRODUCT_ID)
        if device_handle is None:
            raise Exception("Device not found.")

        # claim the device's interface
        device_handle.claimInterface(0)

        # pass the device handle to our bulk endpoint test
        test_bulk_endpoints(device_handle)

        # pass the device handle to our control endpoint test
        test_control_endpoints(device_handle)

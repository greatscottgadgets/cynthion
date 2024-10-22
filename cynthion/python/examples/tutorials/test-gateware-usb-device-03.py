import usb1
import time

VENDOR_ID  = 0x1209 # https://pid.codes/1209/
PRODUCT_ID = 0x0001

VENDOR_SET_FPGA_LEDS   = 0x01
VENDOR_GET_USER_BUTTON = 0x02

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

        # pass the device handle to our control endpoint test
        test_control_endpoints(device_handle)

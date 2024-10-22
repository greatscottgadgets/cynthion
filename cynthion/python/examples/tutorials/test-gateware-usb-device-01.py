import usb1

# - list available usb devices ------------------------------------------------

def list_available_usb_devices(context):
    for device in context.getDeviceList():
        try:
            manufacturer = device.getManufacturer()
            product = device.getProduct()
            print(f"{device}:  {manufacturer} - {product}")
        except Exception as e:
            print(f"{device}: {e}")


# - main ----------------------------------------------------------------------

if __name__ == "__main__":
    with usb1.USBContext() as context:
        list_available_usb_devices(context)

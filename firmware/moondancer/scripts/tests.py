#!/usr/bin/env python3

# Test GCP protocol

import greatfet
import sys, traceback, unittest

class TestGcp(unittest.TestCase):
    def setUp(self):
        self.gf = greatfet.GreatFET()

    def test_connectivity(self):
        result = self.gf.board_name()
        print(f"test_connectivity: {result}")
        self.assertEqual(result, "GreatFET One") # TODO change board name

    def test_class_firmware(self):
        result = self.gf.supports_api("firmware")
        self.assertTrue(result)

        api = self.gf.apis.firmware
        result = api.initialize()
        print(f"test_class_firmware: {result}")

        self.assertEqual(result, (256, 2097152))

    def test_error_no_function(self):
        api = self.gf.apis.firmware
        result = api.supports_verb("test_error_no_function")
        self.assertFalse(result)

        with self.assertRaises(Exception) as context:
            result = api.test_error_no_function()
        self.assertTrue("object has no attribute 'test_error_no_function'" in str(context.exception))

    def test_error_return_code(self):
        from pygreat.errors import LIBGREAT_ERROR_NAMES
        def get_error_code(name):
            return [n for n in LIBGREAT_ERROR_NAMES if LIBGREAT_ERROR_NAMES[n] == name][0]

        api = self.gf.apis.selftest

        result = api.supports_verb("test_error_return_code")
        self.assertTrue(result)

        result = api.test_error_return_code(0)
        self.assertEqual(result, "ok")
        print(f"test_error_return_code: {result}")

        code = get_error_code("EBUSY")
        with self.assertRaises(Exception) as context:
            result = api.test_error_return_code(code)
        self.assertTrue("EBUSY" in str(context.exception))

        code = get_error_code("ECONNRESET")
        with self.assertRaises(Exception) as context:
            result = api.test_error_return_code(code)
        self.assertTrue("ECONNRESET" in str(context.exception))


if __name__ == "__main__":
    unittest.main()

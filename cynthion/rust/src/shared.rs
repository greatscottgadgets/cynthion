#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

/// The values in this module are statically generated at compile
/// time from the corresponding TOML files located in the
/// `cynthion.git/shared/` directory.

pub mod libgreat {
    static_toml::static_toml! {
        pub static TOML = include_toml!("src/shared/libgreat.toml");
    }
    pub mod endpoints {
        use super::TOML;
        pub static bulk_in_address: u8 = TOML.endpoints.bulk_in_address as u8;
        pub static bulk_out_address: u8 = TOML.endpoints.bulk_out_address as u8;
    }
    pub mod vendor {
        use super::TOML;
        pub static command_request: u8 = TOML.vendor.command_request as u8;
    }
}

pub mod registers {
    static_toml::static_toml! {
        pub static TOML = include_toml!("src/shared/registers.toml");
    }

    pub mod todo {
        use super::TOML;
        pub static some_value: u8 = TOML.todo.some_value as u8;
    }
}

pub mod usb {
    static_toml::static_toml! {
        pub static TOML = include_toml!("src/shared/usb.toml");
    }

    pub mod bVendorId {
        use super::TOML;
        pub static apollo: u16 = TOML.b_vendor_id.apollo as u16;
        pub static cynthion: u16 = TOML.b_vendor_id.cynthion as u16;
        pub static example: u16 = TOML.b_vendor_id.example as u16;
    }

    pub mod bProductId {
        use super::TOML;
        pub static apollo: u16 = TOML.b_product_id.apollo as u16;
        pub static cynthion: u16 = TOML.b_product_id.cynthion as u16;
        pub static example: u16 = TOML.b_product_id.example as u16;
        pub static example_2: u16 = TOML.b_product_id.example_2 as u16;
        pub static example_3: u16 = TOML.b_product_id.example_3 as u16;
        pub static example_4: u16 = TOML.b_product_id.example_4 as u16;
        pub static example_5: u16 = TOML.b_product_id.example_5 as u16;
    }

    pub mod bManufacturerString {
        use super::TOML;
        pub static apollo: &str = TOML.b_manufacturer_string.apollo;
        pub static bulk_speed_test: &str = TOML.b_manufacturer_string.bulk_speed_test;
        pub static analyzer: &str = TOML.b_manufacturer_string.analyzer;
        pub static moondancer: &str = TOML.b_manufacturer_string.moondancer;
        pub static example: &str = TOML.b_manufacturer_string.example;
    }

    pub mod bProductString {
        use super::TOML;
        pub static apollo: &str = TOML.b_product_string.apollo;
        pub static bulk_speed_test: &str = TOML.b_product_string.bulk_speed_test;
        pub static analyzer: &str = TOML.b_product_string.analyzer;
        pub static moondancer: &str = TOML.b_product_string.moondancer;
        pub static example: &str = TOML.b_product_string.example;
        pub static example_2: &str = TOML.b_product_string.example_2;
        pub static example_3: &str = TOML.b_product_string.example_3;
        pub static example_4: &str = TOML.b_product_string.example_4;
        pub static example_5: &str = TOML.b_product_string.example_5;
    }

    pub mod bInterfaceSubClass {
        use super::TOML;
        pub static apollo: u8 = TOML.b_interface_sub_class.apollo as u8;
        pub static analyzer: u8 = TOML.b_interface_sub_class.analyzer as u8;
        pub static moondancer: u8 = TOML.b_interface_sub_class.moondancer as u8;
    }

    pub mod bInterfaceProtocol {
        use super::TOML;
        pub static analyzer: u8 = TOML.b_interface_protocol.analyzer as u8;
        pub static moondancer: u8 = TOML.b_interface_protocol.moondancer as u8;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_libgreat() {
        assert_eq!(
            crate::shared::libgreat::TOML.vendor.command_request,
            0x65_i64
        );
        assert_eq!(crate::shared::libgreat::vendor::command_request, 0x65_u8);
    }

    #[test]
    fn test_registers() {
        assert_eq!(crate::shared::registers::TOML.todo.some_value, 42_i64);
        assert_eq!(crate::shared::registers::todo::some_value, 42_u8);
    }

    #[test]
    fn test_usb() {
        assert_eq!(crate::shared::usb::TOML.b_vendor_id.cynthion, 0x1d50_i64);
        assert_eq!(crate::shared::usb::bVendorId::cynthion, 0x1d50_u16);
    }
}

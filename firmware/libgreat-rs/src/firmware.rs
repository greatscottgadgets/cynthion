/// Board information.
pub struct BoardInformation {
    pub board_id: [u8; 4],
    pub version_string: &'static str,
    pub part_id: [u8; 8],
    pub serial_number: [u8; 16],
}

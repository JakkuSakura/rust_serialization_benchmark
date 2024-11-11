use rkyv::{Archive, Deserialize, Serialize};

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[derive(Archive, Serialize, Deserialize, PartialEq)]
pub struct PageHeader {
    pub page_id: u32,
    pub previous_id: u32,
    pub next_id: u32,
    pub page_type: u32,
    pub space_id: u32,
    pub padding: [u32; 3],
}
pub const HEADER_SIZE: usize = 32;
pub const PAGE_SIZE: usize = size_of::<Page>();
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[derive(Archive, Serialize, Deserialize, PartialEq)]
pub struct Page {
    pub header: PageHeader,
    pub data: [u8; 4064],
}

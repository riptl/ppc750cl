use std::io::{Read, Seek, SeekFrom};

use bincode::Options;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A loaded DOL executable.
pub struct Dol {
    pub header: DolHeader,
    pub memory: Vec<u8>,
    pub memory_offset: u32,
}

/// An error that can be raised during DOL parsing.
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BincodeError(bincode::Error),
    #[error("{0}")]
    IOError(std::io::Error),
    #[error("No sections in DOL")]
    NoSections,
    #[error("Overlapping sections")]
    OverlappingSections,
    #[error("Section sizes overflow")]
    SizeOverflow,
}

impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Self::BincodeError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

/// The result of a DOL parsing.
pub type Result<V> = std::result::Result<V, Error>;

impl Dol {
    /// Reads a DOL executable from a `Reader`.
    pub fn read_from<R>(mut r: R) -> Result<Self>
    where
        R: Read + Seek,
    {
        // Read header.
        let header = DolHeader::read_from(&mut r)?;
        Dol::read_with_header(r, header)
    }

    /// Reads a DOL body from a `Reader` given a header.
    pub fn read_with_header<R>(mut r: R, header: DolHeader) -> Result<Self>
    where
        R: Read + Seek,
    {
        let dol_start = r.stream_position()? - DolHeader::SERIALIZED_SIZE;
        // Re-arrange section specifiers into a more workable format.
        // Also remove the text vs data distinction because it's irrelevant.
        struct DolSection {
            offset: u32,
            target: u32,
            size: u32,
        }
        let mut dol_sections =
            Vec::with_capacity(DolHeader::TEXT_SECTION_COUNT + DolHeader::DATA_SECTION_COUNT);
        for i in 0..DolHeader::TEXT_SECTION_COUNT {
            if header.text_sizes[i] > 0 {
                dol_sections.push(DolSection {
                    offset: header.text_offsets[i],
                    target: header.text_targets[i],
                    size: header.text_sizes[i],
                });
            }
        }
        for i in 0..DolHeader::DATA_SECTION_COUNT {
            if header.data_sizes[i] > 0 {
                dol_sections.push(DolSection {
                    offset: header.data_offsets[i],
                    target: header.data_targets[i],
                    size: header.data_sizes[i],
                });
            }
        }
        if dol_sections.is_empty() {
            return Err(Error::NoSections);
        }
        // Sort sections by target address to prepare them for mapping.
        dol_sections.sort_by_key(|s| s.target);
        // Verify that sections are not overlapping.
        let mut end_target_addr = 0u32;
        for section in &dol_sections {
            if section.target < end_target_addr {
                return Err(Error::OverlappingSections);
            }
            end_target_addr = section.target + section.size
        }
        // Remember the memory offset of the first section.
        // Then shift all sections to the beginning of memory.
        let memory_offset = dol_sections[0].target;
        for mut section in &mut dol_sections {
            section.target -= memory_offset;
        }
        // Get the size of all sections combined.
        let mut total_size = 0usize;
        for section in &dol_sections {
            if let Some(sum) = total_size.checked_add(section.size as usize) {
                total_size = sum;
            } else {
                return Err(Error::SizeOverflow);
            }
        }
        // Create memory.
        let mut memory = vec![0u8; total_size];
        // Read sections into memory.
        for section in &dol_sections {
            r.seek(SeekFrom::Start(dol_start + section.offset as u64))?;
            let mem_start = section.target as usize;
            let mem_end = mem_start + section.size as usize;
            r.read_exact(&mut memory[mem_start..mem_end])?;
        }
        Ok(Self {
            header,
            memory,
            memory_offset,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DolHeader {
    text_offsets: [u32; Self::TEXT_SECTION_COUNT],
    data_offsets: [u32; Self::DATA_SECTION_COUNT],
    text_targets: [u32; Self::TEXT_SECTION_COUNT],
    data_targets: [u32; Self::DATA_SECTION_COUNT],
    text_sizes: [u32; Self::TEXT_SECTION_COUNT],
    data_sizes: [u32; Self::DATA_SECTION_COUNT],
    bss_target: u32,
    bss_size: u32,
    entry_point: u32,
}

impl DolHeader {
    const TEXT_SECTION_COUNT: usize = 7;
    const DATA_SECTION_COUNT: usize = 11;
    const SERIALIZED_SIZE: u64 = 0x100;

    fn bincode_options() -> impl bincode::Options {
        bincode::DefaultOptions::new()
            .with_big_endian()
            .allow_trailing_bytes()
    }

    /// Reads the DOL header from a `Reader`.
    pub fn read_from<R: Read>(mut r: R) -> bincode::Result<Self> {
        // Deserialize DOL header.
        let this: Self = Self::bincode_options().deserialize_from(&mut r)?;
        // Read padding.
        let _: [u8; 0x1c] = Self::bincode_options().deserialize_from(&mut r)?;
        Ok(this)
    }
}

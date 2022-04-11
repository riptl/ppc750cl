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
    #[error("Overlapping sections: {0:8>X} {1:8>X}")]
    OverlappingSections(u32, u32),
    #[error("Section sizes too large")]
    SectionsTooLarge,
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
        let header_data = DolHeaderData::read_from(&mut r)?;
        let header: DolHeader = (&header_data).into();
        Dol::read_with_header(r, header)
    }

    /// Reads a DOL body from a `Reader` given a header.
    pub fn read_with_header<R>(mut r: R, header: DolHeader) -> Result<Self>
    where
        R: Read + Seek,
    {
        let dol_start = r.stream_position()? - DolHeaderData::SERIALIZED_SIZE;
        if header.sections.is_empty() {
            return Err(Error::NoSections);
        }
        /*
        // Verify that sections are not overlapping.
        let mut end_target_addr = 0u32;
        for section in &header.sections {
            if section.target < end_target_addr {
                return Err(Error::OverlappingSections(end_target_addr, section.target));
            }
            end_target_addr = section.target + section.size
        }
        */
        // Remember the memory offset of the first section.
        // Then shift all sections to the beginning of memory.
        let memory_offset = header.sections[0].target;
        // Get the size of all sections combined.
        let mut total_size = 0usize;
        for section in &header.sections {
            if let Some(sum) = total_size.checked_add(section.size as usize) {
                total_size = sum;
            } else {
                return Err(Error::SectionsTooLarge);
            }
        }
        // Cannot be larger than 24 MiB.
        if total_size > 0x180_0000 {
            return Err(Error::SectionsTooLarge);
        }
        // Create memory.
        let mut memory = vec![0u8; total_size];
        // Read sections into memory.
        for section in &header.sections {
            if section.kind == DolSectionType::Bss {
                continue;
            }
            r.seek(SeekFrom::Start(dol_start + section.offset as u64))?;
            let mem_start = (section.target - memory_offset) as usize;
            let mem_end = mem_start + section.size as usize;
            r.read_exact(&mut memory[mem_start..mem_end])?;
        }
        Ok(Self {
            header,
            memory,
            memory_offset,
        })
    }

    pub fn section_data(&self, section: &DolSection) -> &[u8] {
        self.virtual_data_at(section.target, section.size as usize)
    }

    /// Returns a slice of DOL data. Does not support bss.
    // TODO Gracefully handle errors.
    pub fn virtual_data_at(&self, virtual_addr: u32, read_len: usize) -> &[u8] {
        let offset = (virtual_addr - self.memory_offset) as usize;
        &self.memory[offset..offset + read_len]
    }

    /// Reads bytes into a destination buffer given a virtual address.
    // TODO Gracefully handle errors.
    pub fn virtual_read(&self, data: &mut [u8], virtual_addr: u32) {
        let offset = (virtual_addr - self.memory_offset) as usize;
        data.copy_from_slice(&self.memory[offset..offset + data.len()])
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DolSectionType {
    Data,
    Bss,
}

#[derive(Debug, Clone)]
pub struct DolSection {
    pub kind: DolSectionType,
    pub index: usize,
    pub offset: u32,
    pub target: u32,
    pub size: u32,
}

pub struct DolHeader {
    pub sections: Vec<DolSection>,
    pub entry_point: u32,
}

impl From<&DolHeaderData> for DolHeader {
    fn from(header: &DolHeaderData) -> Self {
        let mut sections = Vec::with_capacity(DolHeaderData::SECTION_COUNT);
        for i in 0..DolHeaderData::SECTION_COUNT {
            if header.section_sizes[i] > 0 {
                sections.push(DolSection {
                    kind: DolSectionType::Data,
                    index: i,
                    offset: header.section_offsets[i],
                    target: header.section_targets[i],
                    size: header.section_sizes[i],
                });
            }
        }
        if header.bss_target > 0 {
            sections.push(DolSection {
                kind: DolSectionType::Bss,
                index: 0,
                offset: 0,
                target: header.bss_target,
                size: header.bss_size,
            })
        }
        // Sort sections by target address to prepare them for mapping.
        sections.sort_by_key(|s| s.target);
        Self {
            sections,
            entry_point: header.entry_point,
        }
    }
}

impl DolHeader {
    pub fn section_at(&self, addr: u32) -> Option<&DolSection> {
        for section in &self.sections {
            if (section.target..(section.target + section.size)).contains(&addr) {
                return Some(section);
            }
        }
        None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DolHeaderData {
    pub section_offsets: [u32; Self::SECTION_COUNT],
    pub section_targets: [u32; Self::SECTION_COUNT],
    pub section_sizes: [u32; Self::SECTION_COUNT],
    pub bss_target: u32,
    pub bss_size: u32,
    pub entry_point: u32,
    pub padding: [u8; 0x1c],
}

impl DolHeaderData {
    const SECTION_COUNT: usize = 18;
    const SERIALIZED_SIZE: u64 = 0x100;

    /// Reads the DOL header from a `Reader`.
    pub fn read_from<R: Read + Seek>(mut r: R) -> bincode::Result<Self> {
        bincode::DefaultOptions::new()
            .with_big_endian()
            .allow_trailing_bytes()
            .with_fixint_encoding()
            .deserialize_from(&mut r)
    }
}

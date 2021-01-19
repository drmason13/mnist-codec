use std::io::Read;

use anyhow::{self, bail};
use byteorder::{ByteOrder, BigEndian};

const LABEL_FILE_MAGICNUMBER: u32 = 2049;
const IMAGE_FILE_MAGICNUMBER: u32 = 2051;

/// Used to efficiently read a part from the header of a NIST file
/// every part of a header is a BigEndian u32, so this saves even some repetition
macro_rules! read_header_part {
    ($reader:ident) => {
        {
            let mut buf = [0_u8; 4];
            $reader.read_exact(&mut buf)?;
            BigEndian::read_u32(&buf)
        }
    }
}

/// Used to load the header and parse the main content of a NIST Label file
#[derive(Debug, Default)]
pub struct LabelReader {
    label_count: usize,
}

impl LabelReader {
    /// Load the Header from a Readable source and make a LabelReader
    fn load<R: Read>(reader: &mut R) -> anyhow::Result<LabelReader> {
        let label_count = read_header_part!(reader) as usize;
        Ok(LabelReader { label_count })
    }
    
    /// Returns a Vec<u8> which is a vec of labels 0-9 digits.
    pub fn parse<R: Read>(reader: &mut R) -> anyhow::Result<Vec<u8>> {
        let magic_number = read_header_part!(reader);

        // read the header
        if magic_number != LABEL_FILE_MAGICNUMBER {
            bail!("invalid magic number for NIST label file!")
        };
        let header = LabelReader::load(reader)?;

        // read the rest of the data and parse out a Vec<u8>
        let mut buf = Vec::with_capacity(header.label_count as usize);
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

/// Used to load the header and parse the main content of a NIST Image file
#[derive(Debug, Default)]
pub struct ImageReader {
    image_count: usize,
    row_count: usize,
    column_count: usize,
}

impl ImageReader {
    /// Load the header from a Readable source and make an ImageReader
    fn load<R: Read>(reader: &mut R) -> anyhow::Result<ImageReader> {
        let image_count = read_header_part!(reader) as usize;
        let row_count = read_header_part!(reader) as usize;
        let column_count = read_header_part!(reader) as usize;
        Ok(ImageReader { image_count, row_count, column_count })
    }
    
    /// Returns a Vec<Vec<Vec<u8>>> which is a vec of images, which are each a 2d vec of pixels (0-255 intensity).
    pub fn parse<R: Read>(reader: &mut R) -> anyhow::Result<Vec<Vec<Vec<u8>>>> {
        let magic_number = read_header_part!(reader);

        // read the header
        if magic_number != IMAGE_FILE_MAGICNUMBER {
            bail!("invalid magic number for NIST image file!")
        };
        let header = ImageReader::load(reader)?;

        // read the rest of the data and parse out a Vec<u8>
        let mut buf = Vec::with_capacity(header.image_count * header.row_count * header.column_count);
        reader.read_to_end(&mut buf)?;
        // Ok(buf)
        
        // transform the data into desired output format

        // [1, 2, 3, ..., 28, 1, 2, 3, ... 28, ... 28 x, ... 60000 x ]
        // [
            // [
            //     [1, 2, 3, ... 28],
            //     [1, 2, 3, ... 28],
            //     [1, 2, 3, ... 28],
            //     [1, 2, 3, ... 28],
            //     ... 28 x
            // ], ... 60000 x
        // ]

        // let rows = image
        //     .chunks_exact(header.column_count)
        //     .map(|row| row.len())
        //     .collect()

        // let pixels = row
        //     .to_vec()

        let images = buf
            .chunks_exact(header.column_count * header.row_count)
            .map(|image| image
                .chunks_exact(header.column_count)
                .map(|row| row.to_vec())
                .collect())
            .collect();

        Ok(images)
    }
}
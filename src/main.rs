//! Example NIST data reader executable written in rust.
//! This will simply Read local files, parse the Headers and the content and print the data in debug format

use mnist_codec::{LabelReader, ImageReader};
use anyhow::{self, bail};

use std::io::{Write, Seek, SeekFrom::Start};

/// print each image as ascii for funsies!
use ascii::{AsciiArt, luma_to_ascii};
use image::Luma;


fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    let filename_arg = args.skip(1).next();
    if let Some(filename) = filename_arg {
        let mut file = std::fs::File::open(filename)?;
        if let Ok(image_data) = ImageReader::parse(&mut file)  {
            // hacky little loop to page through and view images as ascii because it's fun
            loop {
                println!("Input an index to view the corresponding image");
                let mut input_buf = String::new();
                std::io::stdin().read_line(&mut input_buf)?;
                let current_index = input_buf.trim().parse::<usize>()?;
                if let Err(msg) = view_image_by_index(current_index, &image_data) {
                    eprintln!("{}", msg);
                }
            }
        } else {
            file.seek(Start(0))?;
        }
        
        if let Ok(label_data) = LabelReader::parse(&mut file) {
            println!("LABEL_DATA: \n{:?}", label_data);
        }
    } else {
        bail!("Please provide a filename to read a NIST data file")
    }
    Ok(())
}

fn view_image_by_index(index: usize, image_data: &Vec<Vec<Vec<u8>>>) -> anyhow::Result<()> {
    let image = image_data.get(index);
    if let Some(image) = image {
        let asciis = image.iter().flatten().map(|&pixel| luma_to_ascii(&Luma([pixel]), false)).collect();
        let ascii_art = AsciiArt::new(asciis, (28, 28));
        let mut stdout = std::io::stdout();
        Ok(writeln!(&mut stdout, "{}", &ascii_art)?)
    } else {
        bail!("Sorry, an image at index: {} doesn't exist", index);
    }
}
use std::convert::TryFrom;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::str::FromStr;

use crate::Result;
use crate::args::{DecodeArgs, DownloadArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::download::Download;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let path_buf = args.path.unwrap();
    let path_str = path_buf.to_str().unwrap();
    let file = fs::File::open(path_str).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type.unwrap()).unwrap();
    let message = args.message.unwrap();

    let file_bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let mut png = Png::try_from(file_bytes.as_slice())?;
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());

    match png.chunk_by_type(&chunk.chunk_type().to_string()) {
        Some(_) => println!("Chunk already exists, use the remove command to remove it first"),
        None => {
            png.append_chunk(chunk);
            let file_bytes = png.as_bytes();
            let mut file = OpenOptions::new().write(true).open(path_str)?;
            file.write_all(&file_bytes).unwrap();
        }
    }

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let path_buf = args.path.unwrap();
    let path_str = path_buf.to_str().unwrap();
    let file = fs::File::open(path_str).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type.unwrap()).unwrap();

    let file_bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let png = Png::try_from(file_bytes.as_slice())?;

    match png.chunk_by_type(&chunk_type.to_string()) {
        Some(c) => {
            let message = c.data_as_string().unwrap();
            println!("{}: {}", chunk_type.to_string(), message);
        }
        None => println!("Chunk does not exist"),
    }

    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    let path_buf = args.path.unwrap();
    let path_str = path_buf.to_str().unwrap();
    let file = fs::File::open(path_str).unwrap();
    let chunk_type = ChunkType::from_str(&args.chunk_type.unwrap()).unwrap();

    let file_bytes = &file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let mut png = Png::try_from(file_bytes.as_slice())?;

    png.remove_chunk(&chunk_type.to_string())?;

    let mut file = fs::File::create(path_str)?;
    file.write_all(&png.as_bytes())?;

    Ok(())
}

pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let file = fs::File::open(args.path.unwrap()).unwrap();
    let file_bytes = file.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
    let png = Png::try_from(file_bytes.as_slice())?;
    let chunks = png.chunks();
    for chunk in chunks {
        println!(
            "{}: {}",
            chunk.chunk_type().to_string(),
            chunk.data_as_string().unwrap()
        );
    }
    Ok(())
}

async fn download_file(args: DownloadArgs) -> Result<()> {
    let url = args.url.unwrap();
    let file_name = args.file_name.unwrap();
    let download = Download::new(&url)?;
    download.download(&file_name).await?;
    Ok(())
}

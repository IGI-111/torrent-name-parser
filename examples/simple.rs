extern crate torrent_name_parser;

use torrent_name_parser::Metadata;

pub fn main() {
    let m = Metadata::from("Euphoria.US.S01E03.Made.You.Look.1080p.AMZN.WEB-DL.DDP5.1.H.264-KiNGS");
    println!(
        "{}, Season {}, Episode {}",
        m.title(),
        m.season().unwrap(),
        m.episode().unwrap()
    );
}

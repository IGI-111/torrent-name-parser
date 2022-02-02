use torrent_name_parser::Metadata;

pub fn main() {
    let m = Metadata::from("Euphoria.US.S01E03.Made.You.Look.1080p.AMZN.WEB-DL.DDP5.1.H.264-KiNGS")
        .unwrap();
    print!("Title: {}, ", m.title());
    if m.is_show() {
        print!("Season: {}, ", m.season().unwrap());
        print!("Episode: {}, ", m.episode().unwrap());
        print!("Special: {}, ", m.is_special());
    }
    if let Some(ext) = m.extension() {
        println!("Extension: {}", ext);
    } else {
        println!("");
    }

    let m = Metadata::from(
        "Euphoria.US.S01E03E04.Made.You.Look.1080p.AMZN.WEB-DL.DDP5.1.H.264-KiNGS.avi",
    )
    .unwrap();
    print!("Title: {}, ", m.title());
    if m.is_show() {
        print!("Season: {}, ", m.season().unwrap());
        if m.is_multi_episode() {
            print!("Episodes: ");
            for episode in m.episodes().iter() {
                print!("{}, ", episode);
            }
        }
    }
    if let Some(ext) = m.extension() {
        println!("Extension: {}", ext);
    } else {
        println!("");
    }
}

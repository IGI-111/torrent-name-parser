extern crate torrent_name_parser;

use torrent_name_parser::Metadata;

pub fn main() {
    if let Ok(m) =
        Metadata::from("Euphoria.US.S01E03.Made.You.Look.1080p.AMZN.WEB-DL.DDP5.1.H.264-KiNGS")
    {
        println!(
            "{}, Season {}, Episode {}",
            m.title(),
            m.season().unwrap(),
            m.episode().unwrap()
        );
    }
    if let Ok(m) =
        "Marvels Agents of S.H.I.E.L.D. S01E06 HDTV x264-KILLERS[ettv]".parse::<Metadata>()
    {
        println!(
            "{}, Season {}, Episode {}",
            m.title(),
            m.season().unwrap(),
            m.episode().unwrap()
        );
    }
    let m3: Metadata = "Marvels Agents of S.H.I.E.L.D. S02E06 HDTV x264-KILLERS[ettv]"
        .parse()
        .unwrap();
    println!(
        "{}, Season {}, Episode {}",
        m3.title(),
        m3.season().unwrap(),
        m3.episode().unwrap()
    );

    let m4: Metadata = "Marvels Agents of S.H.I.E.L.D. S02E06e07 HDTV x264-KILLERS[ettv]"
        .parse()
        .unwrap();
    print!("{} contains ", m4.title());
    if let Some(episodes) = m4.multi_episode() {
        for i in 0..episodes.len() {
            print!("Episode {}, ", episodes[i]);
        }
    }
    println!("");

    let m5: Metadata = "Marvels Agents of S.H.I.E.L.D. S02E06e07E08 HDTV x264-KILLERS[ettv]"
        .parse()
        .unwrap();
    print!("{} contains ", m5.title());
    if let Some(_) = m5.multi_episode() {
        for i in m5.episode().unwrap()..=m5.last_episode().unwrap() {
            print!("Episode {}, ", i);
        }
    }
    println!("");
}

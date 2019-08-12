use super::*;

#[test]
fn names() {
    let m = Metadata::from(
        "[TorrentCounter.to].Pacific.Rim.2.Uprising.2018.1080p.HC.HDRip.x264.[2GB].mp4",
    );
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2018));
    assert_eq!(m.title(), "Pacific Rim 2 Uprising");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.quality(), Some("HDRip"));

    let m = Metadata::from("Blade.Runner.2049.2017.HDRip");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2017));
    assert_eq!(m.title(), "Blade Runner 2049");
    assert_eq!(m.quality(), Some("HDRip"));

    let m = Metadata::from("Euphoria.US.S01E03.Made.You.Look.1080p.AMZN.WEB-DL.DDP5.1.H.264-KiNGS");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "Euphoria");

    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(10));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "narcos");

    let m = Metadata::from("Rome.S01E11.The.Spoils.BluRay.10Bit.1080p.Dts.H265-d3g");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(11));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "Rome");

    let m = Metadata::from("the.expanse.s01e09e10.1080p.bluray.x264-rovers");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(9));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "the expanse");

    let m = Metadata::from("Attack on Titan (Shingeki no Kyojin) Season 2 [1080p x265 10bit BD Dual Audio AAC]/Episode 30 - Historia");
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(30));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "Attack on Titan");

    let m = Metadata::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]");
    assert_eq!(m.season(), Some(5));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Walking Dead");
    assert_eq!(m.resolution(), Some("720p"));
    assert_eq!(m.codec(), Some("x264"));
    assert_eq!(m.group(), Some("ASAP[ettv]"));

    let m = Metadata::from("Hercules (2014) 1080p BrRip H264 - YIFY");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.quality(), Some("BrRip"));
    assert_eq!(m.codec(), Some("H264"));
    assert_eq!(m.group(), Some("YIFY"));

    let m = Metadata::from("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Dawn of the Planet of the Apes");
    assert_eq!(m.quality(), Some("HDRip"));
    assert_eq!(m.codec(), Some("XViD"));
    assert_eq!(m.group(), Some("EVO"));

    let m = Metadata::from("The Big Bang Theory S08E06 HDTV XviD-LOL [eztv]");
    assert_eq!(m.season(), Some(8));
    assert_eq!(m.episode(), Some(6));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Big Bang Theory");

    let m = Metadata::from("22 Jump Street (2014) 720p BrRip x264 - YIFY");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "22 Jump Street");

    let m = Metadata::from("Hercules.2014.EXTENDED.1080p.WEB-DL.DD5.1.H264-RARBG");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert!(m.extended());
    assert_eq!(m.audio(), Some("DD5.1"));
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.codec(), Some("H264"));
    assert_eq!(m.group(), Some("RARBG"));

    let m = Metadata::from("Hercules.2014.EXTENDED.HDRip.XViD-juggs[ETRG]");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert!(m.extended());

    let m = Metadata::from("Hercules (2014) WEBDL DVDRip XviD-MAX");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");

    let m = Metadata::from("WWE Hell in a Cell 2014 PPV WEB-DL x264-WD -={SPARROW}=-");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "WWE Hell in a Cell");

    let m = Metadata::from("UFC.179.PPV.HDTV.x264-Ebi[rartv]");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "UFC 179");

    let m = Metadata::from("Marvels Agents of S H I E L D S02E05 HDTV x264-KILLERS [eztv]");
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(5));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Marvels Agents of S H I E L D");

    let m = Metadata::from("X-Men.Days.of.Future.Past.2014.1080p.WEB-DL.DD5.1.H264-RARBG");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "X-Men Days of Future Past");

    let m = Metadata::from("Guardians Of The Galaxy 2014 R6 720p HDCAM x264-JYK");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Guardians Of The Galaxy");

    let m = Metadata::from("Marvel\'s.Agents.of.S.H.I.E.L.D.S02E01.Shadows.1080p.WEB-DL.DD5.1");
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Marvel\'s Agents of S H I E L D");

    let m = Metadata::from("Marvels Agents of S.H.I.E.L.D. S02E06 HDTV x264-KILLERS[ettv]");
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(6));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Marvels Agents of S.H.I.E.L.D.");

    let m = Metadata::from("Guardians of the Galaxy (CamRip / 2014)");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Guardians of the Galaxy");

    let m = Metadata::from("The.Walking.Dead.S05E03.1080p.WEB-DL.DD5.1.H.264-Cyphanix[rartv]");
    assert_eq!(m.season(), Some(5));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Walking Dead");

    let m = Metadata::from("Brave.2012.R5.DVDRip.XViD.LiNE-UNiQUE");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2012));
    assert_eq!(m.title(), "Brave");

    let m = Metadata::from("Lets.Be.Cops.2014.BRRip.XViD-juggs[ETRG]");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Lets Be Cops");

    let m = Metadata::from("These.Final.Hours.2013.WBBRip XViD");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2013));
    assert_eq!(m.title(), "These Final Hours");

    let m = Metadata::from("Downton Abbey 5x06 HDTV x264-FoV [eztv]");
    assert_eq!(m.season(), Some(5));
    assert_eq!(m.episode(), Some(6));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Downton Abbey");

    let m = Metadata::from("Annabelle.2014.HC.HDRip.XViD.AC3-juggs[ETRG]");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Annabelle");
    assert!(m.hardcoded());

    let m = Metadata::from("Lucy.2014.HC.HDRip.XViD-juggs[ETRG]");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Lucy");
    assert!(m.hardcoded());

    let m = Metadata::from("The Flash 2014 S01E04 HDTV x264-FUM[ettv]");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(4));
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "The Flash");

    let m = Metadata::from("South Park S18E05 HDTV x264-KILLERS [eztv]");
    assert_eq!(m.season(), Some(18));
    assert_eq!(m.episode(), Some(5));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "South Park");

    let m = Metadata::from("The Flash 2014 S01E03 HDTV x264-LOL[ettv]");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "The Flash");

    let m = Metadata::from("The Flash 2014 S01E01 HDTV x264-LOL[ettv]");
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "The Flash");

    let m = Metadata::from("Lucy 2014 Dual-Audio WEBRip 1400Mb");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Lucy");

    let m = Metadata::from("Teenage Mutant Ninja Turtles (HdRip / 2014)");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Teenage Mutant Ninja Turtles");

    let m = Metadata::from("Teenage Mutant Ninja Turtles (unknown_release_type / 2014)");
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Teenage Mutant Ninja Turtles");

    let m = Metadata::from("The Simpsons S26E05 HDTV x264 PROPER-LOL [eztv]");
    assert_eq!(m.season(), Some(26));
    assert_eq!(m.episode(), Some(5));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Simpsons");
    assert!(m.proper());
}

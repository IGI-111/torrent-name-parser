use super::*;

#[test]
fn names() {
    let m = Metadata::from("[ www.UsaBit.com ] - My Cousin Vinny (1992) BluRay 720p 750MB Ganool")
        .unwrap();
    assert_eq!(m.year(), Some(1992));
    assert_eq!(m.title(), "My Cousin Vinny");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("2012.2009.1080p.BluRay.x264.DTS-METiS").unwrap();
    assert_eq!(m.year(), Some(2009));
    assert_eq!(m.title(), "2012");
    assert_eq!(m.extension(), None);
    let m =
        Metadata::from("[TorrentCounter.to].Pacific.Rim.2.Uprising.2018.1080p.HC.HDRip.x264.[2GB]")
            .unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2018));
    assert_eq!(m.title(), "Pacific Rim 2 Uprising");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.quality(), Some("HDRip"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Blade.Runner.2049.2017.HDRip").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2017));
    assert_eq!(m.title(), "Blade Runner 2049");
    assert_eq!(m.quality(), Some("HDRip"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Euphoria.US.S01E03.Made.You.Look.1080p.AMZN.WEB-DL.DDP5.1.H.264-KiNGS")
        .unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "Euphoria");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("narcos.s01e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(10));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "narcos");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Rome.S01E11.The.Spoils.BluRay.10Bit.1080p.Dts.H265-d3g").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(11));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "Rome");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("the.expanse.s01e09e10.1080p.bluray.x264-rovers").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(9));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.is_multi_episode(), true);
    assert_eq!(m.title(), "the expanse");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Attack on Titan (Shingeki no Kyojin) Season 2 [1080p x265 10bit BD Dual Audio AAC]/Episode 30 - Historia").unwrap();
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(30));
    assert_eq!(m.year(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.title(), "Attack on Titan");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("The Walking Dead S05E03 720p HDTV x264-ASAP[ettv]").unwrap();
    assert_eq!(m.season(), Some(5));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Walking Dead");
    assert_eq!(m.resolution(), Some("720p"));
    assert_eq!(m.codec(), Some("x264"));
    assert_eq!(m.group(), Some("ASAP[ettv]"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Hercules (2014) 1080p BrRip H264 - YIFY").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.quality(), Some("BrRip"));
    assert_eq!(m.codec(), Some("H264"));
    assert_eq!(m.group(), Some("YIFY"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Dawn.of.the.Planet.of.the.Apes.2014.HDRip.XViD-EVO").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Dawn of the Planet of the Apes");
    assert_eq!(m.quality(), Some("HDRip"));
    assert_eq!(m.codec(), Some("XViD"));
    assert_eq!(m.group(), Some("EVO"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("The Big Bang Theory S08E06 HDTV XviD-LOL [eztv]").unwrap();
    assert_eq!(m.season(), Some(8));
    assert_eq!(m.episode(), Some(6));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Big Bang Theory");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("22 Jump Street (2014) 720p BrRip x264 - YIFY").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "22 Jump Street");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Hercules.2014.EXTENDED.1080p.WEB-DL.DD5.1.H264-RARBG").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert!(m.extended());
    assert_eq!(m.audio(), Some("DD5.1"));
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.codec(), Some("H264"));
    assert_eq!(m.group(), Some("RARBG"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Hercules.2014.EXTENDED.HDRip.XViD-juggs[ETRG]").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert!(m.extended());
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Hercules (2014) WEBDL DVDRip XviD-MAX").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Hercules");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("WWE Hell in a Cell 2014 PPV WEB-DL x264-WD -={SPARROW}=-").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "WWE Hell in a Cell");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("UFC.179.PPV.HDTV.x264-Ebi[rartv]").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "UFC 179");
    assert_eq!(m.extension(), None);

    let m =
        Metadata::from("Marvels Agents of S H I E L D S02E05 HDTV x264-KILLERS [eztv]").unwrap();
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(5));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Marvels Agents of S H I E L D");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("X-Men.Days.of.Future.Past.2014.1080p.WEB-DL.DD5.1.H264-RARBG").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "X-Men Days of Future Past");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Guardians Of The Galaxy 2014 R6 720p HDCAM x264-JYK").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Guardians Of The Galaxy");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Marvel\'s.Agents.of.S.H.I.E.L.D.S02E01.Shadows.1080p.WEB-DL.DD5.1")
        .unwrap();
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Marvel\'s Agents of S H I E L D");
    assert_eq!(m.extension(), None);

    let m =
        Metadata::from("Marvels Agents of S.H.I.E.L.D. S02E06 HDTV x264-KILLERS[ettv]").unwrap();
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(6));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Marvels Agents of S.H.I.E.L.D.");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Guardians of the Galaxy (CamRip / 2014)").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Guardians of the Galaxy");
    assert_eq!(m.extension(), None);

    let m =
        Metadata::from("The.Walking.Dead.S05E03.1080p.WEB-DL.DD5.1.H.264-Cyphanix[rartv]").unwrap();
    assert_eq!(m.season(), Some(5));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Walking Dead");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Brave.2012.R5.DVDRip.XViD.LiNE-UNiQUE").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2012));
    assert_eq!(m.title(), "Brave");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Lets.Be.Cops.2014.BRRip.XViD-juggs[ETRG]").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Lets Be Cops");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("These.Final.Hours.2013.WBBRip XViD").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2013));
    assert_eq!(m.title(), "These Final Hours");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Downton Abbey 5x06 HDTV x264-FoV [eztv]").unwrap();
    assert_eq!(m.season(), Some(5));
    assert_eq!(m.episode(), Some(6));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "Downton Abbey");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Annabelle.2014.HC.HDRip.XViD.AC3-juggs[ETRG]").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Annabelle");
    assert!(m.hardcoded());
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Lucy.2014.HC.HDRip.XViD-juggs[ETRG]").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Lucy");
    assert!(m.hardcoded());
    assert_eq!(m.extension(), None);

    let m = Metadata::from("The Flash 2014 S01E04 HDTV x264-FUM[ettv]").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(4));
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "The Flash");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("South Park S18E05 HDTV x264-KILLERS [eztv]").unwrap();
    assert_eq!(m.season(), Some(18));
    assert_eq!(m.episode(), Some(5));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "South Park");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("The Flash 2014 S01E03 HDTV x264-LOL[ettv]").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "The Flash");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("The Flash 2014 S01E01 HDTV x264-LOL[ettv]").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "The Flash");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Lucy 2014 Dual-Audio WEBRip 1400Mb").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Lucy");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Teenage Mutant Ninja Turtles (HdRip / 2014)").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Teenage Mutant Ninja Turtles");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Teenage Mutant Ninja Turtles (unknown_release_type / 2014)").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.year(), Some(2014));
    assert_eq!(m.title(), "Teenage Mutant Ninja Turtles");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("The Simpsons S26E05 HDTV x264 PROPER-LOL [eztv]").unwrap();
    assert_eq!(m.season(), Some(26));
    assert_eq!(m.episode(), Some(5));
    assert_eq!(m.year(), None);
    assert_eq!(m.title(), "The Simpsons");
    assert!(m.proper());
    assert_eq!(m.extension(), None);

    let m = Metadata::from(
        "To.All.The.Boys.Always.And.Forever.2021.1080p.NF.WEB-DL.x265.10bit.HDR.DDP5.1.Atmos-NWD",
    )
    .unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.quality(), Some("WEB-DL"));
    assert_eq!(m.title(), "To All The Boys Always And Forever");
    assert_eq!(m.year(), Some(2021));
    assert_eq!(m.extension(), None);

    let m =
        Metadata::from("The EXPANSE - S03 E01 - Fight or Flight (1080p - AMZN Web-DL)").unwrap();
    assert_eq!(m.season(), Some(3));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.title(), "The EXPANSE");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("[Judas] Re Zero 2020 - S01E01").unwrap();
    assert_eq!(m.season(), Some(1));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.title(), "Re Zero");
    assert_eq!(m.year(), Some(2020));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Fargo.S04E03.WEB.x264-PHOENiX[TGx]").unwrap();
    assert_eq!(m.season(), Some(4));
    assert_eq!(m.episode(), Some(3));
    assert_eq!(m.title(), "Fargo");
    assert_eq!(m.quality(), Some("WEB"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("[SubsPlease] Dr. Stone S2 - 07 (1080p) [33538C7C]").unwrap();
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(7));
    assert_eq!(m.title(), "Dr. Stone");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from("[SubsPlease] Fumetsu no Anata e S2 - 01 (1080p) [1D65E30D]").unwrap();
    assert_eq!(m.season(), Some(2));
    assert_eq!(m.episode(), Some(1));
    assert_eq!(m.title(), "Fumetsu no Anata e");
    assert_eq!(m.resolution(), Some("1080p"));
    assert_eq!(m.extension(), None);

    let m = Metadata::from(
        "A Shaun the Sheep Movie - Farmageddon (2019) [h265 Remux-1080p] [tt6193408]",
    )
    .unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.imdb_tag(), Some("tt6193408"));
    assert_eq!(m.year(), Some(2019));
    assert_eq!(m.title(), "A Shaun the Sheep Movie Farmageddon");
    assert_eq!(m.extension(), None);

    let m = Metadata::from("Yes Day (2021) [h265 WEBDL-1080p] [tt8521876]").unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.imdb_tag(), Some("tt8521876"));
    assert_eq!(m.year(), Some(2021));
    assert_eq!(m.title(), "Yes Day");
    assert_eq!(m.extension(), None);

    // FIXME
    Metadata::from("[Rip Time] Mushoku Tensei_ Jobless Reincarnation - 12 [1080p]").unwrap_err();

    let m = Metadata::from(
        "Pokémon the Movie - Black - Victini and Reshiram (2011) [x265 Bluray-1080p] [tt1961324]",
    )
    .unwrap();
    assert_eq!(m.season(), None);
    assert_eq!(m.episode(), None);
    assert_eq!(m.episodes().len(), 0);
    assert_eq!(m.imdb_tag(), Some("tt1961324"));
    assert_eq!(m.title(), "Pokémon the Movie Black - Victini and Reshiram");
    assert_eq!(m.year(), Some(2011));
    assert_eq!(m.extension(), None);
}

#[cfg(test)]
mod extensions {
    use crate::metadata::Metadata;

    #[test]
    fn no_extension() {
        let m = Metadata::from("Yes Day (2021) [h265 WEBDL-1080p] [tt8521876]").unwrap();
        assert_eq!(m.season(), None);
        assert_eq!(m.episode(), None);
        assert_eq!(m.imdb_tag(), Some("tt8521876"));
        assert_eq!(m.year(), Some(2021));
        assert_eq!(m.title(), "Yes Day");
        assert_eq!(m.extension(), None);
    }
    #[test]
    fn avi() {
        let m = Metadata::from("Life.on.Mars.(US).S01E01.avi").unwrap();
        assert_eq!(m.title(), "Life on Mars");
        assert_eq!(m.extension(), Some("avi"));
    }
    #[test]
    fn m4v() {
        let m = Metadata::from("Life.on.Mars.(US).S01E01.m4v").unwrap();
        assert_eq!(m.title(), "Life on Mars");
        assert_eq!(m.extension(), Some("m4v"));
    }
}

#[cfg(test)]
mod special {
    use crate::metadata::Metadata;

    #[test]
    fn special() {
        // Support Files with file extension: avi, mkv, mp4
        let m = Metadata::from("Life.on.Mars.(US).S01E01.avi").unwrap();
        assert_eq!(m.title(), "Life on Mars");
        assert_eq!(m.is_show(), true);
        assert_eq!(m.is_special(), false);
    }
    #[test]
    fn not_special() {
        // detect special
        let m = Metadata::from("Life.on.Mars.(US).S00E01.avi").unwrap();
        assert_eq!(m.title(), "Life on Mars");
        assert_eq!(m.is_show(), true);
        assert_eq!(m.is_special(), true);
    }
}

#[cfg(test)]
mod multi_episodes {
    use super::*;

    #[test]
    fn single_episode() {
        let m = Metadata::from("Doctor.who.(2003).S01E01.avi").unwrap();
        assert_eq!(m.title(), "Doctor who");
        assert_eq!(m.season(), Some(1));
        assert_eq!(m.is_multi_episode(), false);
        assert_eq!(m.episode(), Some(1));
        assert_eq!(m.episodes().len(), 0);
    }

    #[test]
    fn double_episode_no_gap() {
        let m = Metadata::from("the.expanse.s01e09e10.1080p.bluray.x264-rovers").unwrap();
        assert_eq!(m.season(), Some(1));
        assert_eq!(m.is_multi_episode(), true);
        assert_eq!(m.episode(), Some(9));
        if m.episodes().len() != 0 {
            let episodes = m.episodes();
            assert_eq!(episodes.len(), 2);
            assert_eq!(episodes[0], 9);
            assert_eq!(episodes[1], 10);
        }
        let episodes = m.episodes();
        let mut first_episode = m.episode().unwrap();
        for i in episodes {
            assert_eq!(i, first_episode);
            first_episode = first_episode + 1;
        }
    }

    #[test]
    fn double_episode_with_gap() {
        let m = Metadata::from("the.expanse.s01e09e11.1080p.bluray.x264-rovers").unwrap();
        assert_eq!(m.season(), Some(1));
        assert_eq!(m.is_multi_episode(), true);
        assert_eq!(m.episodes().len(), 3);
        assert_eq!(m.episode(), Some(9));
        let mut current_episode = m.episode().unwrap();
        for episode in m.episodes().iter() {
            assert_eq!(*episode, current_episode);
            current_episode = current_episode + 1;
        }
    }

    #[test]
    fn triple_episode() {
        let m = Metadata::from("the.expanse.s01e09e10e11.1080p.bluray.x264-rovers").unwrap();
        assert_eq!(m.season(), Some(1));
        assert_eq!(m.is_multi_episode(), true);
        assert_eq!(m.episodes().len(), 3);
        assert_eq!(m.episode(), Some(9));
        let mut current_episode = m.episode().unwrap();
        for episode in m.episodes().iter() {
            assert_eq!(*episode, current_episode);
            current_episode = current_episode + 1;
        }
    }
    #[test]
    fn last_episode_zero() {
        let m = Metadata::from("the.expanse.s01e09e0.1080p.bluray.x264-rovers").unwrap();
        assert_eq!(m.season(), Some(1));
        assert_eq!(m.is_multi_episode(), false);
        assert_eq!(m.episode(), Some(9));
        assert_eq!(m.episodes().len(), 0);
    }
}
#[test]
fn unicode() {
    Metadata::from("éé").unwrap();
    Metadata::from("é2000").unwrap();
    Metadata::from("é2000é").unwrap();
}

use regex::{Captures, Regex};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pattern {
    regex: Regex,
    before_title: bool,
    capture_last: bool,
}

impl Pattern {
    pub fn new(regex: Regex, before_title: bool, capture_last: bool) -> Self {
        Self {
            regex,
            before_title,
            capture_last,
        }
    }
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        if self.capture_last {
            self.regex.captures_iter(text).last()
        } else {
            self.regex.captures(text)
        }
    }

    pub fn before_title(&self) -> bool {
        self.before_title
    }
}

pub fn all_patterns() -> impl Iterator<Item = &'static Pattern> {
    PATTERNS.values()
}

pub fn pattern(name: &str) -> Option<&Pattern> {
    PATTERNS.get(name)
}

lazy_static! {
    static ref PATTERNS: HashMap<&'static str, Pattern> = {
        let mut m = HashMap::new();

        m.insert(
            "season",
            Pattern::new(
                Regex::new(
                    r"[Ss]?(?P<short>[0-9]+)[Eex]|(Season|SEASON)(?:[^0-9]|$)(?P<long>[0-9]+)",
                )
                .unwrap(),
                false,
                false,
            ),
        );
        m.insert("episode",
                 Pattern::new(Regex::new(r"[Ee](?P<short>[0-9]+)(?:[^0-9]|$)|(Episode|EPISODE)(?:[^0-9]|$)(?P<long>[0-9]+)|[0-9]+x(?P<cross>[0-9]+)").unwrap(),
                 false,
                 false,
                 ));
        m.insert(
            "year",
            Pattern::new(
                Regex::new(r"([\[\(]?(?P<year>(?:19[0-9]|20[01])[0-9])[\]\)]?)").unwrap(),
                false,
                true,
            ),
        );
        m.insert(
            "resolution",
            Pattern::new(Regex::new(r"(([0-9]{3,4}p))[^M]").unwrap(), false, false),
        );
        m.insert("quality", Pattern::new(Regex::new(r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?DL(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv").unwrap(), false, false));

        m.insert(
            "codec",
            Pattern::new(
                Regex::new(r"[Xx][Vv][Ii][Dd]|x264|[hH]\.?264/?").unwrap(),
                false,
                false,
            ),
        );
        m.insert(
            "audio",
            Pattern::new(
                Regex::new(r"MP3|DD5\.?1|Dual[\- ]Audio|LiNE|DTS|AAC(?:\.?2\.0)?|AC3(?:\.5\.1)?")
                    .unwrap(),
                false,
                false,
            ),
        );
        m.insert(
            "group",
            Pattern::new(
                Regex::new(r"(- ?([^-]+(?:-=\{[^-]+-?$)?))$").unwrap(),
                false,
                false,
            ),
        );
        m.insert(
            "region",
            Pattern::new(Regex::new(r"R[0-9]").unwrap(), false, false),
        );
        m.insert(
            "extended",
            Pattern::new(Regex::new(r"EXTENDED").unwrap(), false, false),
        );
        m.insert(
            "hardcoded",
            Pattern::new(Regex::new(r"HC").unwrap(), false, false),
        );
        m.insert(
            "proper",
            Pattern::new(Regex::new(r"PROPER").unwrap(), false, false),
        );
        m.insert(
            "repack",
            Pattern::new(Regex::new(r"REPACK").unwrap(), false, false),
        );
        m.insert(
            "container",
            Pattern::new(Regex::new(r"MKV|AVI").unwrap(), false, false),
        );
        m.insert(
            "widescreen",
            Pattern::new(Regex::new(r"WS").unwrap(), false, false),
        );
        m.insert(
            "website",
            Pattern::new(Regex::new(r"^(\[ ?([^\]]+?) ?\]) ?").unwrap(), true, false),
        );
        m.insert(
            "language",
            Pattern::new(Regex::new(r"rus\.eng|US").unwrap(), false, false),
        );
        m.insert(
            "three_d",
            Pattern::new(Regex::new(r"3D").unwrap(), false, false),
        );
        m.insert(
            "unrated",
            Pattern::new(Regex::new(r"UNRATED").unwrap(), false, false),
        );
        m.insert(
            "garbage",
            Pattern::new(
                Regex::new(r"1400Mb|3rd Nov|((Rip)) ").unwrap(),
                false,
                false,
            ),
        );
        m
    };
}

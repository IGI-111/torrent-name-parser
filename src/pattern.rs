use regex::{Captures, Regex};
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Pattern {
    regex: Regex,
    before_title: bool,
    capture_last: bool,
}

macro_rules! regex {
    ($mapping:expr, $name:expr, $pattern:expr, $before_title:expr, $capture_last:expr) => {
        $mapping.insert(
            $name,
            Pattern::new(Regex::new($pattern).unwrap(), $before_title, $capture_last),
        )
    };

    ($mapping:expr, $name:expr, $pattern:expr) => {
        regex!($mapping, $name, $pattern, false, false);
    };
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

pub fn all_patterns() -> impl Iterator<Item = (&'static &'static str, &'static Pattern)> {
    PATTERNS.iter()
}

pub fn pattern(name: &str) -> Option<&Pattern> {
    PATTERNS.get(name)
}

const ALL_RAW_PATTERNS: [(&str, &str); 18] = [
    (
        "season",
        r"[Ss]?(?P<short>\d+) ?[Eex]|(Season|SEASON)(?:[^\d]|$)(?P<long>\d+)",
    ),
    (
        "episode",
        r"[Ee](?P<short>\d+)(?:[^\d]|$)|(Episode|EPISODE)(?:[^\d]|$)(?P<long>\d+)|\d+x(?P<cross>\d+)",
    ),
    ("resolution", r"((\d{3,4}p))[^M]"),
    (
        "quality",
        r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?DL(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv",
    ),
    (
        "codec",
        r"[Xx][Vv][Ii][Dd]|[Xx]264|[hH]\.?264/?|[Xx]265|[Hh]\.?265|[Hh][Ee][Vv][Cc]?",
    ),
    (
        "audio",
        r"MP3|DD5\.?1|Dual[\- ]Audio|LiNE|DTS|AAC(?:\.?2\.0)?|AC3(?:\.5\.1)?",
    ),
    ("group", r"(- ?([^ -]+(?:-=\{[^ -]+-?$)?))$"),
    ("region", r"R\d"),
    ("extended", r"EXTENDED"),
    ("hardcoded", r"HC"),
    ("proper", r"PROPER"),
    ("repack", r"REPACK"),
    ("container", r"MKV|AVI"),
    ("widescreen", r"WS"),
    ("three_d", r"3D"),
    ("unrated", r"UNRATED"),
    ("language", r"rus\.eng|US"),
    ("garbage", r"1400Mb|3rd Nov|((Rip)) "),
];

lazy_static! {
    static ref PATTERNS: HashMap<&'static str, Pattern> = {
        let mut bucket = HashMap::new();

        for (name, pattern) in &ALL_RAW_PATTERNS {
            regex!(bucket, *name, pattern);
        }

        regex!(
            bucket,
            "year",
            r"([\[\(]?(?P<year>(?:1[89]|20)\d\d)[\]\)]?)",
            false,
            true
        );

        regex!(bucket, "website", r"^(\[ ?([^\]]+?) ?\]) ?", true, false);

        bucket
    };
}

use regex::{Captures, Regex};
use std::collections::HashMap;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Pattern {
    regex: Regex,
    before_title: bool,
    capture_last: bool,
    no_numbers_surrounding: bool,
}

macro_rules! regex {
    ($mapping:expr, $name:expr, $pattern:expr, $before_title:expr, $capture_last:expr, $no_numbers_surrounding:expr) => {
        $mapping.insert(
            $name,
            Pattern::new(
                Regex::new($pattern).unwrap(),
                $before_title,
                $capture_last,
                $no_numbers_surrounding,
            ),
        )
    };

    ($mapping:expr, $name:expr, $pattern:expr) => {
        regex!($mapping, $name, $pattern, false, false, false);
    };
}

impl Pattern {
    pub fn new(
        regex: Regex,
        before_title: bool,
        capture_last: bool,
        no_numbers_surrounding: bool,
    ) -> Self {
        Self {
            regex,
            before_title,
            capture_last,
            no_numbers_surrounding,
        }
    }
    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        let mut it = self.regex.captures_iter(text).filter(|cap| {
            if self.no_numbers_surrounding {
                let mat = cap.get(0).unwrap();
                let start = mat.start();
                let end = mat.end();
                if start > 0 && text.chars().nth(start - 1).unwrap().is_digit(10) {
                    return false;
                }
                if end < text.chars().count() && text.chars().nth(end).unwrap().is_digit(10) {
                    return false;
                }
                true
            } else {
                true
            }
        });

        if self.capture_last {
            it.last()
        } else {
            it.next()
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

const ALL_RAW_PATTERNS: [(&str, &str); 19] = [
    (
        "season",
        r"[Ss]?(?P<short>\d+) ?[Eex]|(Season|SEASON)(?:[^\d]|$)(?P<long>\d+)|S(?P<dash>\d+) - \d+",
    ),
    (
        "episode",
        r"[Ee](?P<short>\d+)(?:[^\d]|$)|(Episode|EPISODE)(?:[^\d]|$)(?P<long>\d+)|\d+x(?P<cross>\d+)|S\d+ - (?P<dash>\d+)",
    ),
    ("resolution", r"((\d{3,4}p))[^M]"),
    (
        "quality",
        r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?(DL)?(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv",
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
    ("imdb", r"tt\d{7}"),
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
            r"(?P<year>(1[89]|20)\d\d)",
            false,
            true,
            true
        );

        regex!(
            bucket,
            "website",
            r"^(\[ ?([^\]]+?) ?\]) ?",
            true,
            false,
            false
        );

        bucket
    };
}

use regex::{Captures, Regex};
use std::convert::TryInto;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Pattern {
    regex: Regex,
    before_title: bool,
    capture_last: bool,
    no_numbers_surrounding: bool,
}

macro_rules! regex {
    ($pattern:expr, $before_title:expr, $capture_last:expr, $no_numbers_surrounding:expr) => {
        Pattern::new(
            Regex::new($pattern).unwrap(),
            $before_title,
            $capture_last,
            $no_numbers_surrounding,
        )
    };

    ($pattern:expr) => {
        regex!($pattern, false, false, false)
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
                if start > 0 {
                    // find previous char start
                    let mut prev = start - 1;
                    while !text.is_char_boundary(prev) {
                        prev -= 1;
                    }

                    let prev_char = text[prev..].chars().next().unwrap();
                    if prev_char.is_digit(10) {
                        return false;
                    }
                }

                let end = mat.end();
                if end < text.len() {
                    // find next char start
                    let mut next = end;
                    while !text.is_char_boundary(end) {
                        next += 1;
                    }
                    let next_char = text[next..].chars().next().unwrap();
                    if next_char.is_digit(10) {
                        return false;
                    }
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PatternName {
    Season,
    Episode,
    Resolution,
    Quality,
    Codec,
    Audio,
    Group,
    Region,
    Extended,
    Hardcoded,
    Proper,
    Repack,
    Container,
    Widescreen,
    ThreeD,
    Unrated,
    Language,
    Garbage,
    Imdb,
    Year,
    Website,
}

const ALL_RAW_PATTERNS: [(PatternName, &str); 19] = [
    (
        PatternName::Season,
        r"[Ss]?(?P<short>\d+) ?[Eex]|(Season|SEASON)(?:[^\d]|$)(?P<long>\d+)|S(?P<dash>\d+) - \d+",
    ),
    (
        PatternName::Episode,
        r"[Ee](?P<short>\d+)(?:[^\d]|$)|(Episode|EPISODE)(?:[^\d]|$)(?P<long>\d+)|\d+x(?P<cross>\d+)|S\d+ - (?P<dash>\d+)",
    ),
    (PatternName::Resolution, r"((\d{3,4}p))[^M]"),
    (
        PatternName::Quality,
        r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?(DL)?(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv",
    ),
    (
        PatternName::Codec,
        r"[Xx][Vv][Ii][Dd]|[Xx]264|[hH]\.?264/?|[Xx]265|[Hh]\.?265|[Hh][Ee][Vv][Cc]?",
    ),
    (
        PatternName::Audio,
        r"MP3|DD5\.?1|Dual[\- ]Audio|LiNE|DTS|AAC(?:\.?2\.0)?|AC3(?:\.5\.1)?",
    ),
    (PatternName::Group, r"(- ?([^ -]+(?:-=\{[^ -]+-?$)?))$"),
    (PatternName::Region, r"R\d"),
    (PatternName::Extended, r"EXTENDED"),
    (PatternName::Hardcoded, r"HC"),
    (PatternName::Proper, r"PROPER"),
    (PatternName::Repack, r"REPACK"),
    (PatternName::Container, r"MKV|AVI"),
    (PatternName::Widescreen, r"WS"),
    (PatternName::ThreeD, r"3D"),
    (PatternName::Unrated, r"UNRATED"),
    (PatternName::Language, r"rus\.eng|US"),
    (PatternName::Garbage, r"1400Mb|3rd Nov|((Rip)) "),
    (PatternName::Imdb, r"tt\d{7}"),
];

lazy_static! {
    pub static ref PATTERNS: [(PatternName, Pattern); 21] = {
        let mut bucket: Vec<_> = ALL_RAW_PATTERNS
            .iter()
            .map(|(name, regex)| (*name, regex!(regex)))
            .collect();
        bucket.push((
            PatternName::Year,
            regex!(r"(?P<year>(1[89]|20)\d\d)", false, true, true),
        ));
        bucket.push((
            PatternName::Website,
            regex!(r"^(\[ ?([^\]]+?) ?\]) ?", true, false, false),
        ));
        bucket.try_into().unwrap()
    };
}

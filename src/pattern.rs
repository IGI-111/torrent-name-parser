use regex::{Captures, Regex};
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

lazy_static! {
    pub static ref SEASON: Pattern =
        regex!(r"(?i)s?(?P<short>\d+) ?[ex]|(season)(?:[^\d]|$)(?P<long>\d+)|s(?P<dash>\d+) - \d+");
    pub static ref EPISODE: Pattern = regex!(
        r"(?i)e(?P<short>\d+)(?:[^\d]|$)|(episode)(?:[^\d]|$)(?P<long>\d+)|\d+x(?P<cross>\d+)|s\d+ - (?P<dash>\d+)"
    );
    pub static ref FILE_EXTENSION: Pattern =
        regex!(r"(?:\.)(?P<extension>[a-z]{2,4}(?:\d)?|m4v|3gp|h26[45])$");
    pub static ref RESOLUTION: Pattern = regex!(r"((\d{3,4}p))[^M]");
    pub static ref QUALITY: Pattern = regex!(
        r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?(DL)?(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv"
    );
    pub static ref CODEC: Pattern = regex!(r"(?i)xvid|x264|h\.?264/?|x265|h\.?265|hevc?");
    pub static ref AUDIO: Pattern =
        regex!(r"MP3|DD5\.?1|Dual[\- ]Audio|LiNE|DTS|AAC(?:\.?2\.0)?|AC3(?:\.5\.1)?");
    pub static ref GROUP: Pattern = regex!(r"(- ?([^ -]+(?:-=\{[^ -]+-?$)?))$");
    pub static ref REGION: Pattern = regex!(r"R\d");
    pub static ref EXTENDED: Pattern = regex!(r"EXTENDED");
    pub static ref HARDCODED: Pattern = regex!(r"HC");
    pub static ref PROPER: Pattern = regex!(r"PROPER");
    pub static ref REPACK: Pattern = regex!(r"REPACK");
    pub static ref CONTAINER: Pattern = regex!(r"MKV|AVI");
    pub static ref WIDESCREEN: Pattern = regex!(r"WS");
    pub static ref THREE_D: Pattern = regex!(r"3D");
    pub static ref UNRATED: Pattern = regex!(r"UNRATED");
    pub static ref LANGUAGE: Pattern = regex!(r"rus\.eng|US");
    pub static ref GARBAGE: Pattern = regex!(r"1400Mb|3rd Nov|((Rip)) ");
    pub static ref IMDB: Pattern = regex!(r"tt\d{7}");
    pub static ref YEAR: Pattern = regex!(r"(?P<year>(1[89]|20)\d\d)", false, true, true);
    pub static ref WEBSITE: Pattern = regex!(r"^(\[ ?([^\]]+?) ?\]) ?", true, false, false);
}

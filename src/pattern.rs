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
    pub static ref SEASON: Pattern = regex!(
        r"(?i)s?(?P<short>\d+) ?[ex]|(?:season)(?:[^\d]|$)(?P<long>\d+)|s(?P<dash>\d+) - \d+|\.s(?P<collection>\d){1,2}\."
    );
    pub static ref EPISODE: Pattern = regex!(
        r"(?i)(?:e|episode)[^.\d]?(?P<short>\d{1,3})|\d+x(?P<cross>\d+)|s\d+ - (?P<dash>\d+)"
    );
    pub static ref LAST_EPISODE: Pattern = regex!(r"(?i)(?:e)(?:\d+)(?:[- ]+)?(?:e(?P<last>\d+))+");
    pub static ref FILE_EXTENSION: Pattern =
        regex!(r"(?i)(?:\.)(?P<extension>[a-z]{2,4}(?:\d)?|m4v|3gp|h26[45])$");
    pub static ref RESOLUTION: Pattern = regex!(r"((\d{3,4}p))[^M]");
    pub static ref QUALITY: Pattern = regex!(
        r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?(DL)?(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv"
    );
    pub static ref CODEC: Pattern = regex!(r"(?i)xvid|x264|h\.?264/?|x265|h\.?265|hevc?");
    pub static ref AUDIO: Pattern =
        regex!(r"MP3|DD5\.?1|Dual[\- ]Audio|LiNE|DTS|AAC(?:\.?2\.0)?|AC3(?:\.5\.1)?");
    pub static ref GROUP: Pattern = regex!(r"(- ?([^ -]+(?:-=\{[^ -]+-?$)?))$");
    pub static ref COUNTRY: Pattern = regex!(
        r"\W[(]?(?P<country>(?:U(?:A|G|K|M|S|Y|Z)|(?:A(?:D|E|F|G|I|L|M|N|O|R|S|T|Q|U|W|X|Z))|(?:B(?:A|B|D|E|F|G|H|I|J|L|M|N|O|R|S|T|V|W|Y|Z))|(?:C(?:A|C|D|F|G|H|I|K|L|M|N|O|R|U|V|X|Y|Z))|(?:D(?:E|J|K|M|O|Z))|(?:E(C|E|G|H|R|S|T))|(?:F(?:I|J|K|M|O|R))|(?:G(?:A|B|D|E|F|G|H|I|L|M|N|P|Q|R|S|T|U|W|Y))|(?:H(?:K|M|N|R|T|U))|(?:I(D|E|Q|L|M|N|O|R|S|T))|(?:J(?:E|M|O|P))|(?:K(E|G|H|I|M|N|P|R|W|Y|Z))|(?:L(?:A|B|C|I|K|R|S|T|U|V|Y))|(?:M(?:A|C|D|E|F|G|H|K|L|M|N|O|Q|P|R|S|T|U|V|W|X|Y|Z))|(?:N(?:A|C|E|F|G|I|L|O|P|R|U|Z))|(?:OM)|(?:P(?:A|E|F|G|H|K|L|M|N|R|S|T|W|Y))|(?:QA)|(?:R(?:E|O|S|U|W))|(?:S(?:A|B|C|D|E|G|H|I|J|K|L|M|N|O|R|T|V|Y|Z))|(?:T(?:C|D|F|G|H|J|K|L|M|N|O|R|T|V|W|Z))|(?:V(?:A|C|E|G|I|N|U))|(?:W(F|S))|(?:Y(E|T))|(?:Z(?:A|M|W)))|(?:u(?:a|g|k|m|s|y|z)|(?:a(?:d|e|f|g|i|l|m|n|o|r|s|t|q|u|w|x|z))|(?:b(?:a|b|d|e|f|g|h|i|j|l|m|n|o|r|s|t|v|w|y|z))|(?:c(?:a|c|d|f|g|h|i|k|l|m|n|o|r|u|v|x|y|z))|(?:d(?:e|j|k|m|o|z))|(?:e(c|e|g|h|r|s|t))|(?:f(?:i|j|k|m|o|r))|(?:g(?:a|b|d|e|f|g|h|i|l|m|n|p|q|r|s|t|u|w|y))|(?:h(?:k|m|n|r|t|u))|(?:i(d|e|q|l|m|n|o|r|s|t))|(?:j(?:e|m|o|p))|(?:k(e|g|h|i|m|n|p|r|w|y|z))|(?:l(?:a|b|c|i|k|r|s|t|u|v|y))|(?:m(?:a|c|d|e|f|g|h|k|l|m|n|o|q|p|r|s|t|u|v|w|x|y|z))|(?:n(?:a|c|e|f|g|i|l|o|p|r|u|z))|(?:om)|(?:p(?:a|e|f|g|h|k|l|m|n|r|s|t|w|y))|(?:qa)|(?:r(?:e|o|s|u|w))|(?:s(?:a|b|c|d|e|g|h|i|j|k|l|m|n|o|r|t|v|y|z))|(?:t(?:c|d|f|g|h|j|k|l|m|n|o|r|t|v|w|z))|(?:v(?:a|c|e|g|i|n|u))|(?:w(f|s))|(?:y(e|t))|(?:z(?:a|m|w))))[)]?\.S\d"
    );
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

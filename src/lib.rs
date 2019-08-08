#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::min;
use std::collections::HashMap;

#[cfg(test)]
mod test;

lazy_static! {
    static ref PATTERNS: HashMap<&'static str, Regex> = {
        let mut m = HashMap::new();

        m.insert(
            "season",
            Regex::new(r"[Ss]?(?P<short>[0-9]+)[Eex]|(Season|SEASON)(?:[^0-9]|$)(?P<long>[0-9]+)")
                .unwrap(),
        );
        m.insert("episode", Regex::new(r"[Ee](?P<short>[0-9]+)(?:[^0-9]|$)|(Episode|EPISODE)(?:[^0-9]|$)(?P<long>[0-9]+)|[0-9]+x(?P<cross>[0-9]+)").unwrap());
        m.insert(
            "year",
            Regex::new(r"([\[\(]?(?P<year>(?:19[0-9]|20[01])[0-9])[\]\)]?)").unwrap(),
        );
        m.insert("resolution", Regex::new(r"(([0-9]{3,4}p))[^M]").unwrap());
        m.insert("quality", Regex::new(r"(?:PPV\.)?[HP]DTV|(?:HD)?CAM|B[rR]Rip|TS|(?:PPV )?WEB-?DL(?: DVDRip)?|H[dD]Rip|DVDRip|DVDRiP|DVDRIP|CamRip|W[EB]B[rR]ip|[Bb]lu[Rr]ay|DvDScr|hdtv").unwrap());
        m.insert(
            "codec",
            Regex::new(r"[Xx][Vv][Ii][Dd]|x264|[hH]\.?264/?").unwrap(),
        );
        m.insert(
            "audio",
            Regex::new(r"MP3|DD5\.?1|Dual[\- ]Audio|LiNE|DTS|AAC(?:\.?2\.0)?|AC3(?:\.5\.1)?")
                .unwrap(),
        );
        m.insert(
            "group",
            Regex::new(r"(- ?([^-]+(?:-=\{[^-]+-?$)?))$").unwrap(),
        );
        m.insert("region", Regex::new(r"R[0-9]").unwrap());
        m.insert("extended", Regex::new(r"EXTENDED").unwrap());
        m.insert("hardcoded", Regex::new(r"HC").unwrap());
        m.insert("proper", Regex::new(r"PROPER").unwrap());
        m.insert("repack", Regex::new(r"REPACK").unwrap());
        m.insert("container", Regex::new(r"MKV|AVI").unwrap());
        m.insert("widescreen", Regex::new(r"WS").unwrap());
        m.insert("website", Regex::new(r"^(\[ ?([^\]]+?) ?\])").unwrap());
        m.insert("language", Regex::new(r"rus\.eng|US").unwrap());
        m.insert("three_d", Regex::new(r"3D").unwrap());
        m.insert("unrated", Regex::new(r"UNRATED").unwrap());
        m.insert("garbage", Regex::new(r"1400Mb|3rd Nov| ((Rip))").unwrap());
        m
    };
}

pub struct Metadata {
    title: String,
    season: Option<i32>,
    episode: Option<i32>,
    year: Option<i32>,
    resolution: Option<String>,
    quality: Option<String>,
    codec: Option<String>,
    audio: Option<String>,
    group: Option<String>,
    extended: bool,
    hardcoded: bool,
    proper: bool,
    repack: bool,
    widescreen: bool,
    unrated: bool,
    three_d: bool,
}

impl Metadata {
    pub fn from(name: &str) -> Self {
        let mut title_end = name.len();
        for p in PATTERNS.values() {
            p.captures(name).map(|caps| {
                caps.get(0).map(|m| title_end = min(title_end, m.start()));
            });
        }

        let mut title = name[..title_end - 1].to_string();
        if let Some(pos) = title.find('(') {
            title = title.split_at(pos).0.to_string();
        }
        title = title.trim_start_matches(" -").to_string();
        if !title.contains(' ') && title.contains('.') {
            title = title.replace(".", " ")
        }
        title = title
            .replace("_", " ")
            .replacen("(", "", 1)
            .replacen("- ", "", 1)
            .trim()
            .to_string();

        let season = PATTERNS["season"].captures(name).and_then(|caps| {
            caps.name("short")
                .or(caps.name("long"))
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let episode = PATTERNS["episode"].captures(name).and_then(|caps| {
            caps.name("short")
                .or(caps.name("long"))
                .or(caps.name("cross"))
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let year = PATTERNS["year"].captures(name).and_then(|caps| {
            caps.name("year")
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let resolution = PATTERNS["resolution"]
            .captures(name)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
        let quality = PATTERNS["quality"]
            .captures(name)
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let codec = PATTERNS["codec"]
            .captures(name)
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let audio = PATTERNS["audio"]
            .captures(name)
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let group = PATTERNS["group"]
            .captures(name)
            .and_then(|caps| caps.get(2).map(|m| m.as_str().to_string()));

        let extended = PATTERNS["extended"].captures(name).is_some();
        let hardcoded = PATTERNS["hardcoded"].captures(name).is_some();
        let proper = PATTERNS["proper"].captures(name).is_some();
        let repack = PATTERNS["repack"].captures(name).is_some();
        let widescreen = PATTERNS["widescreen"].captures(name).is_some();
        let unrated = PATTERNS["unrated"].captures(name).is_some();
        let three_d = PATTERNS["three_d"].captures(name).is_some();

        Metadata {
            title,
            season,
            episode,
            year,
            resolution,
            quality,
            codec,
            audio,
            group,
            extended,
            hardcoded,
            proper,
            repack,
            widescreen,
            unrated,
            three_d,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn season(&self) -> Option<i32> {
        self.season
    }
    pub fn episode(&self) -> Option<i32> {
        self.episode
    }
    pub fn year(&self) -> Option<i32> {
        self.year
    }
    pub fn resolution(&self) -> Option<&str> {
        self.resolution.as_ref().map(String::as_str)
    }
    pub fn quality(&self) -> Option<&str> {
        self.quality.as_ref().map(String::as_str)
    }
    pub fn codec(&self) -> Option<&str> {
        self.codec.as_ref().map(String::as_str)
    }
    pub fn audio(&self) -> Option<&str> {
        self.audio.as_ref().map(String::as_str)
    }
    pub fn group(&self) -> Option<&str> {
        self.group.as_ref().map(String::as_str)
    }
    pub fn extended(&self) -> bool {
        self.extended
    }
    pub fn hardcoded(&self) -> bool {
        self.hardcoded
    }
    pub fn proper(&self) -> bool {
        self.proper
    }
    pub fn repack(&self) -> bool {
        self.repack
    }
    pub fn widescreen(&self) -> bool {
        self.widescreen
    }
    pub fn unrated(&self) -> bool {
        self.unrated
    }
    pub fn three_d(&self) -> bool {
        self.three_d
    }
}

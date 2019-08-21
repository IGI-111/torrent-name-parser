use crate::pattern::{all_patterns, pattern};
use std::cmp::{max, min};

#[derive(Debug)]
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
        let mut title_start = 0;
        let mut title_end = name.len();
        for p in all_patterns() {
            if p.before_title() {
                p.captures(name).map(|caps| {
                    caps.get(0).map(|m| title_start = max(title_start, m.end()));
                });
            } else {
                p.captures(name).map(|caps| {
                    caps.get(0).map(|m| title_end = min(title_end, m.start()));
                });
            }
        }

        println!("{:?}", title_end);
        let mut title = name[title_start..title_end - 1].to_string();
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

        let season = pattern("season").unwrap().captures(name).and_then(|caps| {
            caps.name("short")
                .or(caps.name("long"))
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let episode = pattern("episode").unwrap().captures(name).and_then(|caps| {
            caps.name("short")
                .or(caps.name("long"))
                .or(caps.name("cross"))
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let year = pattern("year").unwrap().captures(name).and_then(|caps| {
            caps.name("year")
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let resolution = pattern("resolution")
            .unwrap()
            .captures(name)
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
        let quality = pattern("quality")
            .unwrap()
            .captures(name)
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let codec = pattern("codec")
            .unwrap()
            .captures(name)
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let audio = pattern("audio")
            .unwrap()
            .captures(name)
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let group = pattern("group")
            .unwrap()
            .captures(name)
            .and_then(|caps| caps.get(2).map(|m| m.as_str().to_string()));

        let extended = pattern("extended").unwrap().captures(name).is_some();
        let hardcoded = pattern("hardcoded").unwrap().captures(name).is_some();
        let proper = pattern("proper").unwrap().captures(name).is_some();
        let repack = pattern("repack").unwrap().captures(name).is_some();
        let widescreen = pattern("widescreen").unwrap().captures(name).is_some();
        let unrated = pattern("unrated").unwrap().captures(name).is_some();
        let three_d = pattern("three_d").unwrap().captures(name).is_some();

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

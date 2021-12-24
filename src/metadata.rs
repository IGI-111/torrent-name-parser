use crate::error::ErrorMatch;
use crate::pattern::all_patterns;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    imdb: Option<String>,
}

impl Metadata {
    pub fn from(name: &str) -> Result<Self, ErrorMatch> {
        let mut title_start = 0;
        let mut title_end = name.len();

        let patterns = all_patterns().collect::<Vec<_>>();
        let mut captures = HashMap::with_capacity(patterns.len());
        for (pname, p) in patterns {
            if let Some(m) = p.captures(name) {
                if let Some(cap) = m.get(0) {
                    if p.before_title() {
                        title_start = max(title_start, cap.end());
                    } else {
                        title_end = min(title_end, cap.start());
                    }
                }
                captures.insert(*pname, m);
            }
        }

        if title_start >= title_end {
            return Err(ErrorMatch::new(
                captures.into_iter().map(|(k, v)| (k, v.get(0))).collect(),
            ));
        }

        let mut title = name[title_start..title_end].to_string();
        if let Some(pos) = title.find('(') {
            title = title.split_at(pos).0.to_string();
        }
        title = title.trim_start_matches(" -").to_string();
        title = title.trim_end_matches(" -").to_string();
        if !title.contains(' ') && title.contains('.') {
            title = title.replace('.', " ")
        }
        title = title
            .replace('_', " ")
            .replacen('(', "", 1)
            .replacen("- ", "", 1)
            .trim()
            .to_string();

        let season = captures.get("season").and_then(|caps| {
            caps.name("short")
                .or_else(|| caps.name("long"))
                .or_else(|| caps.name("dash"))
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let episode = captures.get("episode").and_then(|caps| {
            caps.name("short")
                .or_else(|| caps.name("long"))
                .or_else(|| caps.name("cross"))
                .or_else(|| caps.name("dash"))
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let year = captures.get("year").and_then(|caps| {
            caps.name("year")
                .map(|m| m.as_str())
                .map(|s| s.parse().unwrap())
        });
        let resolution = captures
            .get("resolution")
            .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));
        let quality = captures
            .get("quality")
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let codec = captures
            .get("codec")
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let audio = captures
            .get("audio")
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));
        let group = captures
            .get("group")
            .and_then(|caps| caps.get(2).map(|m| m.as_str().to_string()));
        let imdb = captures
            .get("imdb")
            .and_then(|caps| caps.get(0).map(|m| m.as_str().to_string()));

        let extended = captures.contains_key("extended");
        let hardcoded = captures.contains_key("hardcoded");
        let proper = captures.contains_key("proper");
        let repack = captures.contains_key("repack");
        let widescreen = captures.contains_key("widescreen");
        let unrated = captures.contains_key("unrated");
        let three_d = captures.contains_key("three_d");

        Ok(Metadata {
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
            imdb,
        })
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
        self.resolution.as_deref()
    }
    pub fn quality(&self) -> Option<&str> {
        self.quality.as_deref()
    }
    pub fn codec(&self) -> Option<&str> {
        self.codec.as_deref()
    }
    pub fn audio(&self) -> Option<&str> {
        self.audio.as_deref()
    }
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }
    pub fn imdb_tag(&self) -> Option<&str> {
        self.imdb.as_deref()
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

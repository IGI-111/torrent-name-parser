pub mod extension;
use crate::error::ErrorMatch;
use crate::pattern;
use crate::pattern::Pattern;
use regex::Captures;
use std::borrow::Cow;
use std::cmp::{max, min};

use std::{convert::TryFrom, str::FromStr};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Metadata {
    title: String,
    season: Option<i32>,
    episode: Option<i32>,
    episodes: Vec<i32>,
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
    extension: Option<String>,
    is_subtitle: bool,
}

fn check_pattern_and_extract<'a>(
    pattern: &Pattern,
    torrent_name: &'a str,
    title_start: &mut usize,
    title_end: &mut usize,
    extract_value: impl Fn(Captures<'a>) -> Option<&'a str>,
) -> Option<&'a str> {
    pattern.captures(torrent_name).and_then(|caps| {
        if let Some(cap) = caps.get(0) {
            if pattern.before_title() {
                *title_start = max(*title_start, cap.end());
            } else {
                *title_end = min(*title_end, cap.start());
            }
        }
        extract_value(caps)
    })
}

fn check_pattern<'a>(
    pattern: &Pattern,
    torrent_name: &'a str,
    title_start: &mut usize,
    title_end: &mut usize,
) -> Option<Captures<'a>> {
    pattern.captures(torrent_name).map(|caps| {
        if let Some(cap) = caps.get(0) {
            if pattern.before_title() {
                *title_start = max(*title_start, cap.end());
            } else {
                *title_end = min(*title_end, cap.start());
            }
        }
        caps
    })
}

fn capture_to_string(caps: Option<Captures<'_>>) -> Option<String> {
    caps.and_then(|c| c.get(0)).map(|m| m.as_str().to_string())
}

impl Metadata {
    ///```
    /// use torrent_name_parser::Metadata;
    ///
    /// if let Ok(m) = Metadata::from("Doctor.Who.(2003).S01E01.avi") {
    ///   assert_eq!(m.title(), "Doctor Who");
    ///   assert_eq!(m.season(), Some(1));
    ///   assert_eq!(m.extension(), Some("avi"));
    ///   assert_eq!(m.ext_is_subtitle(), false);
    ///   assert_eq!(m.is_show(), true);
    ///   // Season is not 0 (zero) meaning it is not a Season Special. Eg: Christmas Special
    ///   assert_eq!(m.is_special(), false);

    /// }
    ///```
    pub fn from(name: &str) -> Result<Self, ErrorMatch> {
        Metadata::from_str(name)
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
    /// Contains a `Vec` of episode numbers detected.
    /// # Examples:
    /// No matches -> `[]`  
    /// `E01` -> `[1]`  
    /// `E03e04` -> `[3,4]`  
    /// `e03E07` -> `[3,4,5,6,7]`  
    pub fn episodes(&self) -> &Vec<i32> {
        &self.episodes
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
    pub fn extension(&self) -> Option<&str> {
        self.extension.as_deref()
    }
    pub fn ext_is_subtitle(&self) -> bool {
        self.is_subtitle
    }
    pub fn is_show(&self) -> bool {
        self.season.is_some()
    }
    pub fn is_special(&self) -> bool {
        self.season.map(|s| s < 1).unwrap_or(false)
    }
}

impl FromStr for Metadata {
    type Err = ErrorMatch;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let mut title_start = 0;
        let mut title_end = name.len();
        let mut is_subtitle = false;
        let mut episodes: Vec<i32> = Vec::new();
        let interim_last_episode;

        let season = check_pattern_and_extract(
            &pattern::SEASON,
            name,
            &mut title_start,
            &mut title_end,
            |caps| {
                caps.name("short")
                    .or_else(|| caps.name("long"))
                    .or_else(|| caps.name("dash"))
                    .or_else(|| caps.name("collection"))
                    .map(|m| m.as_str())
            },
        );

        let episode = check_pattern_and_extract(
            &pattern::EPISODE,
            name,
            &mut title_start,
            &mut title_end,
            |caps| {
                caps.name("short")
                    .or_else(|| caps.name("cross"))
                    .or_else(|| caps.name("dash"))
                    .map(|m| m.as_str())
            },
        );
        // Only look for a last episode if pattern::EPISODE returned a value.
        if let Some(first_episode) = episode {
            episodes.push(first_episode.parse().unwrap());
            interim_last_episode = check_pattern_and_extract(
                &pattern::LAST_EPISODE,
                name,
                &mut title_start,
                &mut title_end,
                |caps| caps.get(1).map(|m| m.as_str()),
            );
            if let Some(last_episode) = interim_last_episode {
                // Sanity check that last_episode does not contain a value or 0 (Zero)
                if last_episode.len() == 1 && last_episode.contains('0') {
                    // Treat a string ending with '0' (zero) as invalid and skip further work
                } else {
                    // Populate Vec with each episode number
                    for number_of_episode in
                        first_episode.parse::<i32>().unwrap() + 1..=last_episode.parse().unwrap()
                    {
                        episodes.push(number_of_episode);
                    }
                }
            }
        }
        let year = check_pattern_and_extract(
            &pattern::YEAR,
            name,
            &mut title_start,
            &mut title_end,
            |caps: Captures<'_>| caps.name("year").map(|m| m.as_str()),
        );

        let resolution = check_pattern_and_extract(
            &pattern::RESOLUTION,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(1).map(|m| m.as_str()),
        )
        .map(String::from);
        let quality = check_pattern_and_extract(
            &pattern::QUALITY,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let codec = check_pattern_and_extract(
            &pattern::CODEC,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let audio = check_pattern_and_extract(
            &pattern::AUDIO,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let group = check_pattern_and_extract(
            &pattern::GROUP,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(2).map(|m| m.as_str()),
        )
        .map(String::from);
        let imdb = check_pattern_and_extract(
            &pattern::IMDB,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let extension = check_pattern_and_extract(
            &pattern::FILE_EXTENSION,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(1).map(|m| m.as_str()),
        )
        .map(String::from);
        if let Some(ext) = &extension {
            is_subtitle = extension::set_is_subtitle(&ext);
        }

        let extended = check_pattern(&pattern::EXTENDED, name, &mut title_start, &mut title_end);
        let hardcoded = check_pattern(&pattern::HARDCODED, name, &mut title_start, &mut title_end);
        let proper = check_pattern(&pattern::PROPER, name, &mut title_start, &mut title_end);
        let repack = check_pattern(&pattern::REPACK, name, &mut title_start, &mut title_end);
        let widescreen =
            check_pattern(&pattern::WIDESCREEN, name, &mut title_start, &mut title_end);
        let unrated = check_pattern(&pattern::UNRATED, name, &mut title_start, &mut title_end);
        let three_d = check_pattern(&pattern::THREE_D, name, &mut title_start, &mut title_end);

        let region = check_pattern(&pattern::REGION, name, &mut title_start, &mut title_end);
        let container = check_pattern(&pattern::CONTAINER, name, &mut title_start, &mut title_end);
        let language = check_pattern(&pattern::LANGUAGE, name, &mut title_start, &mut title_end);
        let garbage = check_pattern(&pattern::GARBAGE, name, &mut title_start, &mut title_end);
        let website = check_pattern(&pattern::WEBSITE, name, &mut title_start, &mut title_end);

        if title_start >= title_end {
            return Err(ErrorMatch::new(vec![
                ("season", season.map(String::from)),
                ("episode", episode.map(String::from)),
                ("year", year.map(String::from)),
                ("extension", extension.map(String::from)),
                ("resolution", resolution),
                ("quality", quality),
                ("codec", codec),
                ("audio", audio),
                ("group", group),
                ("imdb", imdb),
                ("extended", capture_to_string(extended)),
                ("proper", capture_to_string(proper)),
                ("repack", capture_to_string(repack)),
                ("widescreen", capture_to_string(widescreen)),
                ("unrated", capture_to_string(unrated)),
                ("three_d", capture_to_string(three_d)),
                ("region", capture_to_string(region)),
                ("container", capture_to_string(container)),
                ("language", capture_to_string(language)),
                ("garbage", capture_to_string(garbage)),
                ("website", capture_to_string(website)),
            ]));
        }

        let mut title = &name[title_start..title_end];
        if let Some(pos) = title.find('(') {
            title = title.split_at(pos).0;
        }
        title = title.trim_start_matches(" -");
        title = title.trim_end_matches(" -");
        let title = match !title.contains(' ') && title.contains('.') {
            true => Cow::Owned(title.replace('.', " ")),
            false => Cow::Borrowed(title),
        };
        let title = title
            .replace('_', " ")
            .replacen('(', "", 1)
            .replacen("- ", "", 1)
            .trim()
            .to_string();

        Ok(Metadata {
            title,
            season: season.map(|s| s.parse().unwrap()),
            episode: episode.map(|s| s.parse().unwrap()),
            episodes,
            year: year.map(|s| s.parse().unwrap()),
            resolution,
            quality,
            codec,
            audio,
            group,
            extended: extended.is_some(),
            hardcoded: hardcoded.is_some(),
            proper: proper.is_some(),
            repack: repack.is_some(),
            widescreen: widescreen.is_some(),
            unrated: unrated.is_some(),
            three_d: three_d.is_some(),
            imdb,
            extension,
            is_subtitle,
        })
    }
}

impl TryFrom<&str> for Metadata {
    type Error = ErrorMatch;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Metadata::from_str(s)
    }
}

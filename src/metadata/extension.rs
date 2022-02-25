#[derive(Debug, Eq, PartialEq)]
pub enum ExtensionError {
    Err(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::Err(s) => write!(f, "invalid file extension: {}", s),
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SubtitleExtension {
    #[doc(hidden)]
    SRT,
    #[doc(hidden)]
    SSA,
    #[doc(hidden)]
    SVB,
    #[doc(hidden)]
    VTT,
}

impl std::str::FromStr for SubtitleExtension {
    type Err = ExtensionError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "srt" => Ok(Self::SRT),
            "ssa" => Ok(Self::SSA),
            "svb" => Ok(Self::SVB),
            "vtt" => Ok(Self::VTT),
            _ => Err(ExtensionError::Err(input.to_string())),
        }
    }
}

impl std::fmt::Display for SubtitleExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubtitleExtension::SRT => write!(f, "srt"),
            SubtitleExtension::SSA => write!(f, "ssa"),
            SubtitleExtension::SVB => write!(f, "svb"),
            SubtitleExtension::VTT => write!(f, "vtt"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VideoExtension {
    #[doc(hidden)]
    AVI,
    #[doc(hidden)]
    M4V,
}

impl std::str::FromStr for VideoExtension {
    type Err = ExtensionError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "avi" => Ok(Self::AVI),
            "m4v" => Ok(Self::M4V),
            _ => Err(ExtensionError::Err(input.to_owned())),
        }
    }
}

impl std::fmt::Display for VideoExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoExtension::AVI => write!(f, "avi"),
            VideoExtension::M4V => write!(f, "m4v"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FileExtension {
    #[doc(hidden)]
    Subtitle(SubtitleExtension),
    #[doc(hidden)]
    Video(VideoExtension),
}

impl std::str::FromStr for FileExtension {
    type Err = ExtensionError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "srt" | "ssa" | "svb" | "vtt" => {
                Ok(FileExtension::Subtitle(SubtitleExtension::from_str(input)?))
            }
            "m4v" | "avi" => Ok(FileExtension::Video(VideoExtension::from_str(input)?)),
            _ => Err(ExtensionError::Err(input.to_owned())),
        }
    }
}

impl std::fmt::Display for FileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileExtension::Subtitle(value) => write!(f, "{}", value),
            FileExtension::Video(value) => write!(f, "{}", value),
        }
    }
}
impl FileExtension {
    pub fn is_subtitle(&self) -> bool {
        matches!(self, FileExtension::Subtitle(_))
    }
    pub fn is_video(&self) -> bool {
        matches!(self, FileExtension::Video(_))
    }
}

#[cfg(test)]
#[test]
fn str() {
    let ext: SubtitleExtension = "srt".parse().unwrap();
    assert_eq!(ext.to_string(), "srt".to_string());
    let ext: SubtitleExtension = "ssa".parse().unwrap();
    assert_eq!(ext.to_string(), "ssa".to_string());
    let ext: Result<SubtitleExtension, ExtensionError> = "test".parse();
    assert_eq!(ext.is_err(), true);
    let err = ext.unwrap_err();
    assert_eq!(err.to_string(), "invalid file extension: test");

    let ext: VideoExtension = "avi".parse().unwrap();
    assert_eq!(ext.to_string(), "avi");
    let ext: VideoExtension = "m4v".parse().unwrap();
    assert_eq!(ext.to_string(), "m4v");

    let ext: FileExtension = "srt".parse().unwrap();
    assert_eq!(ext.is_subtitle(), true);
    assert_eq!(ext.is_video(), false);
    println!("{:?}", ext);
    let ext: FileExtension = "avi".parse().unwrap();
    assert_eq!(ext.is_subtitle(), false);
    assert_eq!(ext.is_video(), true);
    println!("{:?}", ext);
    println!("test: {}", ext);
}

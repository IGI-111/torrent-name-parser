pub(crate) fn set_is_subtitle(str: &str) -> bool {
    match str.to_ascii_lowercase().as_str() {
        "srt" | "ssa" | "svb" | "vtt" | "ttml" | "dfxp" => true,
        _ => false,
    }
}

#[cfg(test)]
#[test]
fn str() {
    let ext = set_is_subtitle("srt");
    assert_eq!(ext, true);
    let ext = set_is_subtitle("ssa");
    assert_eq!(ext, true);
    let ext = set_is_subtitle("svb");
    assert_eq!(ext, true);
    let ext = set_is_subtitle("vtt");
    assert_eq!(ext, true);
    let ext = set_is_subtitle("ttml");
    assert_eq!(ext, true);
    let ext = set_is_subtitle("DfXp");
    assert_eq!(ext, true);
}

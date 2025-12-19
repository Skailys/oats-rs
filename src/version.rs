#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
enum VERSION {
    ORIGINAL, // TODO re first two versions with ^[0-9a-fA-FxX][0-9a-fA-F]([0-9a-fA-FxX][0-9a-fA-F])?[0-9A-Za-z_-]{11}$
    EXPANDED,

    #[default]
    UNKNOWN // fallback
}

fn is_valid_original(s: &str) -> bool {
    let mut cs = s.chars();
    let mut count = 0;

    // Test 1: Check if the first two chars are hexdigits
    if !cs.by_ref().take(2).enumerate().all(|(idx, c)| {count += 1; c.is_ascii_hexdigit() || (idx % 2 == 0 && c.eq_ignore_ascii_case(&'x'))}) {
        return false;
    }

    // Test 2: Check if the rest of the digits are 
    if !cs.take(11).all(|c| {count += 1; c.is_ascii_alphanumeric() || c == '-' || c == '_'}) {
        return false;
    }

    // Test 3: Check length
    count == 13
}

fn is_valid_expanded(s: &str) -> bool {
    let mut cs = s.chars();
    let mut count = 0;

    // Test 1: Check if the first two chars are hexdigits
    if !cs.by_ref().take(4).enumerate().all(|(idx, c)| {count += 1; c.is_ascii_hexdigit() || (idx % 2 == 0 && c.eq_ignore_ascii_case(&'x'))}) {
        return false;
    }

    // Test 2: Check if the rest of the digits are 
    if !cs.take(11).all(|c| {count += 1; c.is_ascii_alphanumeric() || c == '-' || c == '_'}) {
        return false;
    }

    // Test 3: Check length
    count == 15
}

pub(crate) fn detect_version(s: &str) -> VERSION {
    if is_valid_original(s) {
        return VERSION::ORIGINAL;
    };

    if is_valid_expanded(s) {
        return VERSION::EXPANDED;
    }

    VERSION::UNKNOWN
}
/// Simple segmentation that preserves punctuation, numbers, and non-CJK.
pub fn segment(text: &str) -> Vec<String> {
    let mut segments = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !current.is_empty() {
                segments.push(current.clone());
                current.clear();
            }
            continue;
        }
        if is_cjk(ch) {
            if !current.is_empty() && !is_cjk(current.chars().next().unwrap()) {
                segments.push(current.clone());
                current.clear();
            }
            if !current.is_empty() {
                segments.push(current.clone());
                current.clear();
            }
            current.push(ch);
        } else {
            if !current.is_empty() && is_cjk(current.chars().next().unwrap()) {
                segments.push(current.clone());
                current.clear();
            }
            current.push(ch);
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }
    segments
}

fn is_cjk(ch: char) -> bool {
    (ch >= '\u{4E00}' && ch <= '\u{9FFF}')
        || (ch >= '\u{3400}' && ch <= '\u{4DBF}')
        || (ch >= '\u{20000}' && ch <= '\u{2A6DF}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_basic() {
        let segs = segment("单于夜遁逃");
        assert_eq!(segs, vec!["单", "于", "夜", "遁", "逃"]);
    }
}

use sanitizer::prelude::StringSanitizer;

pub fn sanitize_string(string: &mut String, max_length: usize) {
    let s_slice = string.as_str();
    let mut sanitizer = StringSanitizer::from(s_slice);

    // remove non-numeric characters from the string
    sanitizer.numeric();
    *string = sanitizer.get();

    // Remove leading 0
    while string.len() > 1 && string.starts_with('0') {
        string.remove(0);
    }

    // truncate string
    if string.len() >= max_length {
        string.truncate(max_length);
    };
}

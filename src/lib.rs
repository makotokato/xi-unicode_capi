/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

extern crate xi_unicode;

use xi_unicode::LineBreakIterator;

fn break_all_breaker(source: &str, break_buffer: &mut [u8])
{
    let mut current = 0;
    for ch in source.chars() {
        break_buffer[current] = 1;
        current += ch.len_utf16();
    }
}

#[no_mangle]
pub unsafe extern fn xi_unicode_breaker(source: *const u16, length: usize,
                                        word_break_type: u32,
                                        break_output: *mut u8)
{
    let str_buffer = std::slice::from_raw_parts(source, length);
    let mut break_buffer = std::slice::from_raw_parts_mut(break_output, length);
    let source_str = std::string::String::from_utf16_lossy(str_buffer);

    if word_break_type == 0 {
        let mut source_chars = source_str.chars();
        let mut last_index = 0;
        let mut utf16_index = 0;

        for (index, _) in LineBreakIterator::new(source_str.as_ref()) {
            // index is utf-8 index, but we require utf-16 index
            let mut count = 0;
            while last_index < index {
                let ch = source_chars.next().unwrap();
                count += ch.len_utf16();
                last_index += ch.len_utf8();
            }
            utf16_index += count;
            if utf16_index < length {
                break_buffer[utf16_index] = 1;
            }
        }
    } else if word_break_type == 1 {
        // nsILineBreaker::kWordBreak_BreakAll
        break_all_breaker(source_str.as_ref(), break_buffer);
    }
}

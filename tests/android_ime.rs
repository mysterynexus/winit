#[path = "../src/platform_impl/android/ime.rs"]
mod ime;

use ime::{ImeState, TextEdit};

#[test]
fn soft_keyboard_insertion_deletion_and_unchanged_text() {
    let mut ime = ImeState::default();

    assert_eq!(ime.update_text("hello"), Some(TextEdit { backspaces: 0, text: "hello".into() }));
    assert_eq!(ime.update_text("hello!"), Some(TextEdit { backspaces: 0, text: "!".into() }));
    assert_eq!(ime.update_text("hello"), Some(TextEdit { backspaces: 1, text: String::new() }));
    assert_eq!(ime.update_text("hello"), Some(TextEdit { backspaces: 0, text: String::new() }));
    assert_eq!(ime.update_text(""), Some(TextEdit { backspaces: 5, text: String::new() }));
}

#[test]
fn unicode_edits_count_characters_instead_of_bytes() {
    let mut ime = ImeState::default();

    assert_eq!(ime.update_text("😀é"), Some(TextEdit { backspaces: 0, text: "😀é".into() }));
    assert_eq!(ime.update_text("😀β"), Some(TextEdit { backspaces: 1, text: "β".into() }));
    assert_eq!(ime.update_text(""), Some(TextEdit { backspaces: 2, text: String::new() }));
}

#[test]
fn composing_text_replacement_rewrites_the_changed_suffix() {
    let mut ime = ImeState::default();

    ime.update_text("I like teh cat");
    assert_eq!(
        ime.update_text("I like the cat"),
        Some(TextEdit { backspaces: 6, text: "he cat".into() })
    );
}

#[test]
fn key_text_suppresses_duplicate_text_updates_and_synchronizes_history() {
    let mut ime = ImeState::default();

    ime.update_text("a");
    ime.begin_input_batch();
    ime.record_key_event(true, false);
    ime.record_key_event(false, false);
    assert_eq!(ime.update_text("ab"), None);
    assert_eq!(ime.update_text("abc"), None);

    ime.begin_input_batch();
    assert_eq!(ime.update_text("abcd"), Some(TextEdit { backspaces: 0, text: "d".into() }));
}

#[test]
fn edit_keys_suppress_duplicates_without_character_text() {
    let mut ime = ImeState::default();

    ime.update_text("ab");
    ime.begin_input_batch();
    ime.record_key_event(false, true);
    assert_eq!(ime.update_text("a"), None);

    ime.begin_input_batch();
    ime.record_key_event(false, false);
    assert_eq!(ime.update_text("ac"), Some(TextEdit { backspaces: 0, text: "c".into() }));
}

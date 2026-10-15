// GameActivity delivers key events before its text-state update. Keep their text from being
// emitted twice, while remembering the latest state for the next input batch.
#[derive(Default)]
pub(crate) struct ImeState {
    previous_text: String,
    key_event_produced_text: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TextEdit {
    pub(crate) backspaces: usize,
    pub(crate) text: String,
}

impl ImeState {
    pub(crate) fn begin_input_batch(&mut self) {
        self.key_event_produced_text = false;
    }

    pub(crate) fn record_key_event(&mut self, has_text: bool, is_edit_key: bool) {
        self.key_event_produced_text |= has_text || is_edit_key;
    }

    pub(crate) fn update_text(&mut self, new: &str) -> Option<TextEdit> {
        if self.key_event_produced_text {
            self.previous_text = new.to_owned();
            return None;
        }

        let common_prefix =
            self.previous_text.chars().zip(new.chars()).take_while(|(old, new)| old == new).count();
        let edit = TextEdit {
            backspaces: self.previous_text.chars().count() - common_prefix,
            text: new.chars().skip(common_prefix).collect(),
        };
        self.previous_text = new.to_owned();
        Some(edit)
    }
}

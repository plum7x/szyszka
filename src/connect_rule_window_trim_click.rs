use crate::example_fields::update_examples;
use crate::gui_data::GuiData;
use crate::help_function::validate_name;
use gtk4::prelude::*;

pub fn connect_rule_window_trim_click(gui_data: &GuiData) {
    let window_rules = gui_data.window_rules.clone();

    let check_button_trim_name_start = gui_data.window_rules.trim.check_button_trim_name_start.clone();
    let check_button_trim_name_end = gui_data.window_rules.trim.check_button_trim_name_end.clone();
    let check_button_trim_extension_start = gui_data.window_rules.trim.check_button_trim_extension_start.clone();
    let check_button_trim_extension_end = gui_data.window_rules.trim.check_button_trim_extension_end.clone();
    let check_button_trim_case_insensitive = gui_data.window_rules.trim.check_button_trim_case_insensitive.clone();
    let check_button_trim_case_sensitive = gui_data.window_rules.trim.check_button_trim_case_sensitive.clone();

    let entry_add_text_text_to_trim = gui_data.window_rules.trim.entry_add_text_text_to_trim.clone();

    check_button_trim_name_start.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_trim_name_end.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_trim_extension_start.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_trim_extension_end.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_trim_case_insensitive.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    check_button_trim_case_sensitive.connect_toggled(move |_e| {
        update_examples(&window_rules, None);
    });
    let window_rules = gui_data.window_rules.clone();
    entry_add_text_text_to_trim.connect_changed(move |e| {
        let old_name = e.text().to_string();
        let validate_name = validate_name(old_name.clone());
        if validate_name != old_name {
            e.set_text(&validate_name);
        }
        update_examples(&window_rules, None);
    });
}

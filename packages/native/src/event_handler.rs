use std::collections::HashMap;

use dioxus_html::{FormValue, HasFileData, HasFormData, HtmlEventConverter, PlatformEventData};

use super::keyboard_event::BlitzKeyboardData;

#[derive(Clone)]
pub struct NativeClickData;

impl dioxus_html::point_interaction::InteractionLocation for NativeClickData {
    fn client_coordinates(&self) -> dioxus_html::geometry::ClientPoint {
        todo!()
    }

    fn screen_coordinates(&self) -> dioxus_html::geometry::ScreenPoint {
        todo!()
    }

    fn page_coordinates(&self) -> dioxus_html::geometry::PagePoint {
        todo!()
    }
}
impl dioxus_html::point_interaction::InteractionElementOffset for NativeClickData {
    fn element_coordinates(&self) -> dioxus_html::geometry::ElementPoint {
        todo!()
    }
}
impl dioxus_html::point_interaction::ModifiersInteraction for NativeClickData {
    fn modifiers(&self) -> keyboard_types::Modifiers {
        todo!()
    }
}

impl dioxus_html::point_interaction::PointerInteraction for NativeClickData {
    fn trigger_button(&self) -> Option<dioxus_html::input_data::MouseButton> {
        todo!()
    }

    fn held_buttons(&self) -> dioxus_html::input_data::MouseButtonSet {
        todo!()
    }
}
impl dioxus_html::HasMouseData for NativeClickData {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }
}

pub struct NativeConverter {}

impl HtmlEventConverter for NativeConverter {
    fn convert_animation_data(&self, _event: &PlatformEventData) -> dioxus_html::AnimationData {
        todo!()
    }

    fn convert_clipboard_data(&self, _event: &PlatformEventData) -> dioxus_html::ClipboardData {
        todo!()
    }

    fn convert_composition_data(&self, _event: &PlatformEventData) -> dioxus_html::CompositionData {
        todo!()
    }

    fn convert_drag_data(&self, _event: &PlatformEventData) -> dioxus_html::DragData {
        todo!()
    }

    fn convert_focus_data(&self, _event: &PlatformEventData) -> dioxus_html::FocusData {
        todo!()
    }

    fn convert_form_data(&self, event: &PlatformEventData) -> dioxus_html::FormData {
        let o = event.downcast::<NativeFormData>().unwrap().clone();
        dioxus_html::FormData::from(o)
    }

    fn convert_image_data(&self, _event: &PlatformEventData) -> dioxus_html::ImageData {
        todo!()
    }

    fn convert_keyboard_data(&self, event: &PlatformEventData) -> dioxus_html::KeyboardData {
        let data = event.downcast::<BlitzKeyboardData>().unwrap().clone();
        dioxus_html::KeyboardData::from(data)
    }

    fn convert_media_data(&self, _event: &PlatformEventData) -> dioxus_html::MediaData {
        todo!()
    }

    fn convert_mounted_data(&self, _event: &PlatformEventData) -> dioxus_html::MountedData {
        todo!()
    }

    fn convert_mouse_data(&self, event: &PlatformEventData) -> dioxus_html::MouseData {
        let o = event.downcast::<NativeClickData>().unwrap().clone();
        dioxus_html::MouseData::from(o)
    }

    fn convert_pointer_data(&self, _event: &PlatformEventData) -> dioxus_html::PointerData {
        todo!()
    }

    fn convert_scroll_data(&self, _event: &PlatformEventData) -> dioxus_html::ScrollData {
        todo!()
    }

    fn convert_selection_data(&self, _event: &PlatformEventData) -> dioxus_html::SelectionData {
        todo!()
    }

    fn convert_toggle_data(&self, _event: &PlatformEventData) -> dioxus_html::ToggleData {
        todo!()
    }

    fn convert_touch_data(&self, _event: &PlatformEventData) -> dioxus_html::TouchData {
        todo!()
    }

    fn convert_transition_data(&self, _event: &PlatformEventData) -> dioxus_html::TransitionData {
        todo!()
    }

    fn convert_wheel_data(&self, _event: &PlatformEventData) -> dioxus_html::WheelData {
        todo!()
    }

    fn convert_resize_data(&self, _event: &PlatformEventData) -> dioxus_html::ResizeData {
        todo!()
    }

    fn convert_visible_data(&self, _event: &PlatformEventData) -> dioxus_html::VisibleData {
        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct NativeFormData {
    pub value: String,
    pub values: HashMap<String, FormValue>,
}

impl HasFormData for NativeFormData {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn value(&self) -> String {
        self.value.clone()
    }

    fn values(&self) -> HashMap<String, FormValue> {
        self.values.clone()
    }
}

impl HasFileData for NativeFormData {}

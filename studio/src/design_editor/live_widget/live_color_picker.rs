use {
    makepad_render::*,
    makepad_widget::color_picker::*,
    crate::{
        code_editor::{
            live_widget::*
        }
    }
};

live_register!{
    LiveColorPicker: {{LiveColorPicker}} {
    }
}

fn register_factory(cx: &mut Cx) {
    struct Factory();
    impl LiveWidgetFactory for Factory {
        fn new_live_widget(&self, cx: &mut Cx) -> Box<dyn LiveWidget> {
            Box::new(LiveColorPicker::new(cx))
        }
        
        fn can_edit_value(&self, _live_registry: &LiveRegistry, node: &LiveNode) -> CanEdit {
            if let LiveValue::Color(_) = &node.value{
                return CanEdit::Yes(100.0)
            }
            CanEdit::No
        }
    }
    let live_type_info = LiveColorPicker::live_type_info(cx);
    cx.registries.register_live_widget(
        live_type_info,
        Box::new(Factory()),
        LiveId::from_str("color_picker").unwrap(),
    )
}

impl LiveWidget for LiveColorPicker {
    fn handle_widget_event(&mut self, _cx: &mut Cx, _event: &mut Event) -> LiveWidgetAction {
        LiveWidgetAction::None
    }
    
    fn draw_widget(&mut self, cx: &mut Cx) {
        self.color_picker.size = 100.0;
        self.color_picker.draw(cx, Vec4::default(), 1.0);
    }
}

#[derive(Live, LiveHook)]
#[live_register_hook(register_factory)]
pub struct LiveColorPicker {
    color_picker: ColorPicker
}

impl LiveColorPicker {
    pub fn draw(&mut self, _cx: &mut Cx) {
    }
    
    pub fn handle_event(
        &mut self,
        _cx: &mut Cx,
        _event: &mut Event,
    )->LiveWidgetAction{
        
        LiveWidgetAction::None
    }
}
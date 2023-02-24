use iced::{
    application, color,
    widget::{button, container, text},
    Color, Vector,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct Theme {
}


impl application::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: color!(0x28, 0x28, 0x28),
            text_color: Color::WHITE,
        }
    }
}

impl text::StyleSheet for Theme {
    type Style = ();

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            //color: color!(0xeb, 0xdb, 0xb2).into(),
            color: Color {
                r: 235., 
                g: 219., 
                b: 178.,
                a: 1.
            }.into(),
        }
    }
}



#[derive(Debug, Clone, Copy, Default)]
pub enum Container {
    #[default]
    Default,
    Bordered,
}
impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match style {
            Container::Default => container::Appearance::default(),
            Container::Bordered => container::Appearance {
                border_color: color!(0x45, 0x85, 0x88),
                border_width: 1.0,
                border_radius: 4.0,
                ..Default::default()
            },
        }
    }
}



#[derive(Debug, Clone, Copy, Default)]
pub enum Button {
    #[default]
    Primary,
}
impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: color!(0x28, 0x28, 0x28).into(),
            shadow_offset: Vector::default(),
            text_color: Color::BLACK,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            background: color!(0x18, 0x18, 0x18).into(),
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            background: color!(0x10, 0x10, 0x10).into(),
            shadow_offset: active.shadow_offset + Vector::new(0.0, 1.0),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..self.active(style)
        }
    }
}

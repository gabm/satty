use std::{borrow::Cow, collections::HashMap};

use crate::{
    configuration::APP_CONFIG,
    style::{Color, Size},
    tools::Tools,
};

use gdk_pixbuf::{
    gio::SimpleAction,
    glib::{Variant, VariantTy},
    Pixbuf,
};
use gtk::ToggleButton;
use relm4::{
    actions::{ActionablePlus, RelmAction, RelmActionGroup},
    gtk::{gdk::RGBA, prelude::*, Align, ColorChooserDialog, ResponseType, Window},
    prelude::*,
};

pub struct ToolsToolbar {
    visible: bool,
    active_button: Option<ToggleButton>,
    tool_buttons: HashMap<Tools, ToggleButton>,
}

pub struct StyleToolbar {
    custom_color: Color,
    custom_color_pixbuf: Pixbuf,
    color_action: SimpleAction,
    visible: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum ToolbarEvent {
    ToolSelected(Tools),
    ColorSelected(Color),
    SizeSelected(Size),
    Redo,
    Undo,
    SaveFile,
    CopyClipboard,
    ToggleFill,
}

#[derive(Debug, Copy, Clone)]
pub enum ToolsToolbarInput {
    ToggleVisibility,
    SwitchSelectedTool(Tools),
}

#[derive(Debug, Copy, Clone)]
pub enum StyleToolbarInput {
    ColorButtonSelected(ColorButtons),
    ShowColorDialog,
    ColorDialogFinished(Option<Color>),
    ToggleVisibility,
}

fn create_icon_pixbuf(color: Color) -> Pixbuf {
    let pixbuf = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, 40, 40).unwrap();
    pixbuf.fill(color.to_rgba_u32());
    pixbuf
}
fn create_icon(color: Color) -> gtk::Image {
    gtk::Image::from_pixbuf(Some(&create_icon_pixbuf(color)))
}

#[relm4::component(pub)]
impl SimpleComponent for ToolsToolbar {
    type Init = ();
    type Input = ToolsToolbarInput;
    type Output = ToolbarEvent;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 2,
            set_valign: Align::Start,
            set_halign: Align::Center,
            add_css_class: "toolbar",
            add_css_class: "toolbar-top",

            #[watch]
            set_visible: model.visible,


            gtk::Button {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "arrow-undo-filled",
                set_tooltip: "Undo (Ctrl-Z)",
                connect_clicked[sender] => move |_| {sender.output_sender().emit(ToolbarEvent::Undo);},
            },
            gtk::Button {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "arrow-redo-filled",
                set_tooltip: "Redo (Ctrl-Y)",
                connect_clicked[sender] => move |_| {sender.output_sender().emit(ToolbarEvent::Redo);},
            },
            gtk::Separator {},
            #[name(pointer_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "cursor-regular",
                set_tooltip: "Pointer (P)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Pointer,
            },
            #[name(crop_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "crop-filled",
                set_tooltip: "Crop (C)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Crop,
            },
            #[name(brush_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "pen-regular",
                set_tooltip: "Brush tool (B)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Brush,
            },
            #[name(line_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "minus-large",
                set_tooltip: "Line tool (L)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Line,
            },
            #[name(arrow_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "arrow-up-right-filled",
                set_tooltip: "Arrow tool (A)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Arrow,
            },
            #[name(rectangle_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "checkbox-unchecked-regular",
                set_tooltip: "Rectangle tool (R)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Rectangle,
            },
            #[name(ellipse_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "circle-regular",
                set_tooltip: "Ellipse tool (E)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Ellipse,
            },
            #[name(text_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "text-case-title-regular",
                set_tooltip: "Text tool (T)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Text,
            },
            #[name(marker_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "number-circle-1-regular",
                set_tooltip: "Numbered Marker (M)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Marker,
            },
            #[name(blur_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "drop-regular",
                set_tooltip: "Blur (U)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Blur,
            },
            #[name(highlight_button)]
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "highlight-regular",
                set_tooltip: "Highlight (H)",
                ActionablePlus::set_action::<ToolsAction>: Tools::Highlight,
            },
            gtk::Separator {},
            gtk::Button {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "copy-regular",
                set_tooltip: "Copy to clipboard (Ctrl+C)",
                connect_clicked[sender] => move |_| {sender.output_sender().emit(ToolbarEvent::CopyClipboard);},
            },
            gtk::Button {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "save-regular",
                set_tooltip: "Save (Ctrl+S)",
                connect_clicked[sender] => move |_| {sender.output_sender().emit(ToolbarEvent::SaveFile);},

                set_visible: APP_CONFIG.read().output_filename().is_some()
            },

        },
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            ToolsToolbarInput::ToggleVisibility => {
                self.visible = !self.visible;
            }
            ToolsToolbarInput::SwitchSelectedTool(tool) => {
                if let Some(previous_button) = &self.active_button {
                    previous_button.set_active(false);
                }

                if let Some(selected_tool_button) = self.tool_buttons.get(&tool) {
                    selected_tool_button.set_active(true);
                    self.active_button = Some(selected_tool_button.clone());
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = ToolsToolbar {
            visible: !APP_CONFIG.read().default_hide_toolbars(),
            active_button: None,
            tool_buttons: HashMap::new(),
        };
        let widgets = view_output!();

        model.tool_buttons = HashMap::from([
            (Tools::Pointer, widgets.pointer_button.clone()),
            (Tools::Crop, widgets.crop_button.clone()),
            (Tools::Brush, widgets.brush_button.clone()),
            (Tools::Line, widgets.line_button.clone()),
            (Tools::Arrow, widgets.arrow_button.clone()),
            (Tools::Rectangle, widgets.rectangle_button.clone()),
            (Tools::Ellipse, widgets.ellipse_button.clone()),
            (Tools::Text, widgets.text_button.clone()),
            (Tools::Marker, widgets.marker_button.clone()),
            (Tools::Blur, widgets.blur_button.clone()),
            (Tools::Highlight, widgets.highlight_button.clone()),
        ]);

        model.active_button = Some(widgets.pointer_button.clone());

        // Tools Action for selecting tools
        let sender_tmp: ComponentSender<ToolsToolbar> = sender.clone();
        let tool_action: RelmAction<ToolsAction> = RelmAction::new_stateful_with_target_value(
            &APP_CONFIG.read().initial_tool(),
            move |_, state, value| {
                *state = value;
                sender_tmp
                    .output_sender()
                    .emit(ToolbarEvent::ToolSelected(*state));
                sender_tmp
                    .input_sender()
                    // Updates the active_button from the toolbar UI with the newly selected tool whenever a mouse
                    // click occurs, so that selection by mouse and by keyboard don't coexist in the UI
                    .emit(ToolsToolbarInput::SwitchSelectedTool(*state));
            },
        );

        let mut group = RelmActionGroup::<ToolsToolbarActionGroup>::new();
        group.add_action(tool_action);
        group.register_for_widget(&widgets.root);

        ComponentParts { model, widgets }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ColorButtons {
    Palette(u64),
    Custom,
}

impl StyleToolbar {
    fn show_color_dialog(&self, sender: ComponentSender<StyleToolbar>, root: Option<Window>) {
        let current_color: RGBA = self.custom_color.into();
        relm4::spawn_local(async move {
            let mut builder = ColorChooserDialog::builder()
                .modal(true)
                .title("Choose Color")
                .hide_on_close(true)
                .rgba(&current_color);

            if let Some(w) = root {
                builder = builder.transient_for(&w);
            }

            // build dialog and configure further
            let dialog = builder.build();
            dialog.set_use_alpha(true);

            let custom_colors = APP_CONFIG
                .read()
                .color_palette()
                .custom()
                .iter()
                .copied()
                .map(RGBA::from)
                .collect::<Vec<_>>();

            if !custom_colors.is_empty() {
                dialog.add_palette(
                    gtk::Orientation::Horizontal,
                    8, // A more or less arbitrary, but reasonable value.
                    &custom_colors,
                );
            }

            // set callback for result
            let dialog_copy = dialog.clone();
            dialog.connect_response(move |_, r| {
                if r == ResponseType::Ok {
                    dialog_copy.hide();
                    let color = Color::from_gdk(dialog_copy.rgba());
                    sender.input(StyleToolbarInput::ColorDialogFinished(Some(color)));
                }
            });

            dialog.show();
        });
    }

    fn map_button_to_color(&self, button: ColorButtons) -> Color {
        let config = APP_CONFIG.read();
        match button {
            ColorButtons::Palette(n) => config.color_palette().palette()[n as usize],
            ColorButtons::Custom => self.custom_color,
        }
    }
}

#[relm4::component(pub)]
impl Component for StyleToolbar {
    type Init = ();
    type Input = StyleToolbarInput;
    type Output = ToolbarEvent;
    type CommandOutput = ();

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 2,
            set_valign: Align::End,
            set_halign: Align::Center,
            add_css_class: "toolbar",
            add_css_class: "toolbar-bottom",

            #[watch]
            set_visible: model.visible,

            gtk::Separator {},
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                gtk::Image::from_pixbuf(Some(&model.custom_color_pixbuf)) {
                    #[watch]
                    set_from_pixbuf: Some(&model.custom_color_pixbuf)
                },
                ActionablePlus::set_action::<ColorAction>: ColorButtons::Custom,
            },
            gtk::Button {

                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "color-regular",
                set_tooltip: "Pick custom color",

                connect_clicked => StyleToolbarInput::ShowColorDialog,
            },
            gtk::Separator {},
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_label: "S",
                set_tooltip: "Small size",
                ActionablePlus::set_action::<SizeAction>: Size::Small,
            },
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_label: "M",
                set_tooltip: "Medium size",
                ActionablePlus::set_action::<SizeAction>: Size::Medium,
            },
            gtk::ToggleButton {
                set_focusable: false,
                set_hexpand: false,

                set_label: "L",
                set_tooltip: "Large size",
                ActionablePlus::set_action::<SizeAction>: Size::Large,
            },
            gtk::Button {
                set_focusable: false,
                set_hexpand: false,

                set_icon_name: "paint-bucket-regular",
                set_tooltip: "Fill shape",
                connect_clicked[sender] => move |button| {
                    sender.output_sender().emit(ToolbarEvent::ToggleFill);
                    let new_icon = if button.icon_name() == Some("paint-bucket-regular".into()) {
                        "paint-bucket-filled"
                    } else {
                        "paint-bucket-regular"
                    };
                    button.set_icon_name(new_icon);
                },
            },
        },
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            StyleToolbarInput::ShowColorDialog => {
                self.show_color_dialog(sender, root.toplevel_window());
            }
            StyleToolbarInput::ColorDialogFinished(color) => {
                if let Some(color) = color {
                    self.custom_color = color;
                    self.custom_color_pixbuf = create_icon_pixbuf(color);

                    // set the custom button active
                    self.color_action
                        .change_state(&ColorButtons::Custom.to_variant());

                    // set new color
                    sender
                        .output_sender()
                        .emit(ToolbarEvent::ColorSelected(color));
                }
            }
            StyleToolbarInput::ColorButtonSelected(button) => {
                let color = self.map_button_to_color(button);
                sender
                    .output_sender()
                    .emit(ToolbarEvent::ColorSelected(color));
            }

            StyleToolbarInput::ToggleVisibility => {
                self.visible = !self.visible;
            }
        }
    }
    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        for (i, &color) in APP_CONFIG
            .read()
            .color_palette()
            .palette()
            .iter()
            .enumerate()
            .rev()
        {
            let btn = gtk::ToggleButton::builder()
                .focusable(false)
                .hexpand(false)
                .child(&create_icon(color))
                .build();
            btn.set_action::<ColorAction>(ColorButtons::Palette(i as u64));
            root.prepend(&btn);
        }

        // Color Action for selecting colors
        let sender_tmp: ComponentSender<StyleToolbar> = sender.clone();
        let color_action: RelmAction<ColorAction> = RelmAction::new_stateful_with_target_value(
            &ColorButtons::Palette(0),
            move |_, state, value| {
                *state = value;

                sender_tmp.input(StyleToolbarInput::ColorButtonSelected(value));
            },
        );

        // Size Action for selecting sizes
        let sender_tmp = sender.clone();
        let size_action: RelmAction<SizeAction> =
            RelmAction::new_stateful_with_target_value(&Size::Medium, move |_, state, value| {
                *state = value;
                sender_tmp
                    .output_sender()
                    .emit(ToolbarEvent::SizeSelected(*state));
            });

        let custom_color = APP_CONFIG
            .read()
            .color_palette()
            .custom()
            .first()
            .copied()
            .unwrap_or(Color::red());
        let custom_color_pixbuf = create_icon_pixbuf(custom_color);

        // create model
        let model = StyleToolbar {
            custom_color,
            custom_color_pixbuf,
            color_action: SimpleAction::from(color_action.clone()),
            visible: !APP_CONFIG.read().default_hide_toolbars(),
        };

        // create widgets
        let widgets = view_output!();

        let mut group = RelmActionGroup::<StyleToolbarActionGroup>::new();
        group.add_action(color_action);
        group.add_action(size_action);

        group.register_for_widget(&widgets.root);

        ComponentParts { model, widgets }
    }
}
relm4::new_action_group!(ToolsToolbarActionGroup, "tools-toolbars");
relm4::new_stateful_action!(ToolsAction, ToolsToolbarActionGroup, "tools", Tools, Tools);

relm4::new_action_group!(StyleToolbarActionGroup, "style-toolbars");
relm4::new_stateful_action!(
    ColorAction,
    StyleToolbarActionGroup,
    "colors",
    ColorButtons,
    ColorButtons
);

impl Clone for ColorAction {
    fn clone(&self) -> Self {
        Self {}
    }
}

relm4::new_stateful_action!(SizeAction, StyleToolbarActionGroup, "sizes", Size, Size);

impl StaticVariantType for ColorButtons {
    fn static_variant_type() -> Cow<'static, VariantTy> {
        Cow::Borrowed(VariantTy::UINT64)
    }
}

impl ToVariant for ColorButtons {
    fn to_variant(&self) -> Variant {
        Variant::from(match *self {
            Self::Palette(i) => i,
            Self::Custom => u64::MAX,
        })
    }
}

impl FromVariant for ColorButtons {
    fn from_variant(variant: &Variant) -> Option<Self> {
        <u64>::from_variant(variant).map(|v| match v {
            std::u64::MAX => Self::Custom,
            _ => Self::Palette(v),
        })
    }
}

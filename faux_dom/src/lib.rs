use bevy::{
    a11y::accesskit::Node,
    asset::Handle,
    ecs::{entity::Entity, system::Commands},
    hierarchy::BuildChildren,
    text::{Font, Text, TextStyle},
    ui::node_bundles::{NodeBundle, TextBundle},
};
pub use faux_dom_shared::FauxNode;
pub use faux_dom_shared::Properties;

pub struct FauxStyle {
    pub style: bevy::ui::Style,
    pub background_color: Option<bevy::ui::BackgroundColor>,
    pub font_size: Option<f32>,
    pub color: Option<bevy::render::color::Color>,
}

#[derive(Clone)]
struct CascadingStyle {
    font: Handle<Font>,
    font_size: Option<f32>,
    color: Option<bevy::render::color::Color>,
}

pub trait Stylesheet {
    fn get_styles(&self, names: &str) -> FauxStyle;
}

pub fn render(
    node: FauxNode,
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    stylesheet: &impl Stylesheet,
) {
    return render_cascading(
        node,
        commands,
        parent,
        stylesheet,
        &CascadingStyle {
            font: font.clone(),
            font_size: None,
            color: None,
        },
    );
}

fn render_cascading(
    node: FauxNode,
    commands: &mut Commands,
    parent: Entity,
    stylesheet: &impl Stylesheet,
    cascading_styles: &CascadingStyle,
) {
    match node {
        FauxNode::Text(text) => {
            commands.entity(parent).with_children(|parent| {
                let mut bundle = TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: cascading_styles.font.clone(),
                            font_size: cascading_styles
                                .font_size
                                .unwrap_or(TextStyle::default().font_size),
                            color: cascading_styles.color.unwrap_or(TextStyle::default().color),
                        },
                    ),
                    ..Default::default()
                };

                parent.spawn(bundle);
            });
        }
        FauxNode::Div(children, properties) => {
            let mut bundle = NodeBundle {
                ..Default::default()
            };

            let mut next_cascading_styles = cascading_styles.clone();

            if let Some(class) = properties.class {
                let styles = stylesheet.get_styles(class.as_str());
                bundle.style = styles.style;

                if let Some(background_color) = styles.background_color {
                    bundle.background_color = background_color.clone();
                }

                if styles.font_size.is_some() {
                    next_cascading_styles.font_size = styles.font_size;
                }

                if styles.color.is_some() {
                    next_cascading_styles.color = styles.color;
                }
            }

            let div_entity = commands.spawn(bundle).id();

            commands.entity(parent).add_child(div_entity);

            for child in children {
                render_cascading(
                    child,
                    commands,
                    div_entity,
                    stylesheet,
                    &next_cascading_styles,
                );
            }
        }
        FauxNode::Fragment(children) => {
            for child in children {
                render_cascading(child, commands, parent, stylesheet, cascading_styles);
            }
        }
        FauxNode::Expr(_) => panic!("Expr not supported"),
    }
}

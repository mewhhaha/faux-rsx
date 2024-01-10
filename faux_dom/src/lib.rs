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

pub trait Stylesheet {
    fn get_style(&self, names: &str) -> Option<bevy::ui::Style>;
    fn get_background_color(&self, names: &str) -> Option<bevy::ui::BackgroundColor>;
}

pub fn render(
    node: FauxNode,
    commands: &mut Commands,
    parent: Entity,
    font: &Handle<Font>,
    stylesheet: &impl Stylesheet,
) {
    match node {
        FauxNode::Text(text) => {
            commands.entity(parent).with_children(|parent| {
                let mut bundle = TextBundle {
                    text: Text::from_section(
                        text,
                        TextStyle {
                            font: font.clone(),
                            ..Default::default()
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

            if let Some(class) = properties.class {
                if let Some(style) = stylesheet.get_style(class.as_str()) {
                    bundle.style = style.clone();
                }

                if let Some(background_color) = stylesheet.get_background_color(class.as_str()) {
                    bundle.background_color = background_color.clone();
                }
            }

            let div_entity = commands.spawn(bundle).id();

            commands.entity(parent).add_child(div_entity);

            for child in children {
                render(child, commands, div_entity, &font, stylesheet);
            }
        }
        FauxNode::Fragment(children) => {
            for child in children {
                render(child, commands, parent, &font, stylesheet);
            }
        }
        FauxNode::Expr(_) => panic!("Expr not supported"),
    }
}

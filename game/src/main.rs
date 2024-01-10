use std::path::Path;

use bevy::prelude::*;
use faux_dom::{render, FauxNode, Properties, Stylesheet};
use faux_dom_macro::rsx;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font = asset_server.load::<Font>("roboto.ttf");

    let root = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let apple = "banana";

    render(
        rsx! {<div>"hello"<div>"world"</div></div>},
        &mut commands,
        root,
        &font,
        &DefaultStyleSheet {},
    );
}

const ASPECT_VIDEO: f32 = 16.0 / 9.0;
const ASPECT_SQUARE: f32 = 1.0;

struct DefaultStyleSheet {}

const REM: f32 = 16.0;

impl Stylesheet for DefaultStyleSheet {
    fn get_style(&self, names: &str) -> Option<Style> {
        let mut style = Style {
            ..Default::default()
        };

        for name in names.split(" ") {
            match name {
                "flex-row" => style.flex_direction = FlexDirection::Row,
                "flex-col" => style.flex_direction = FlexDirection::Column,
                "flex-row-reverse" => style.flex_direction = FlexDirection::RowReverse,
                "flex-col-reverse" => style.flex_direction = FlexDirection::ColumnReverse,
                "items-start" => style.align_content = AlignContent::FlexStart,
                "items-end" => style.align_content = AlignContent::FlexEnd,
                "items-center" => style.align_content = AlignContent::Center,
                "items-between" => style.align_content = AlignContent::SpaceBetween,
                "items-around" => style.align_content = AlignContent::SpaceAround,
                "items-evenly" => style.align_content = AlignContent::SpaceEvenly,
                "items-stretch" => style.align_content = AlignContent::Stretch,
                "self-start" => style.align_self = AlignSelf::FlexStart,
                "self-end" => style.align_self = AlignSelf::FlexEnd,
                "self-center" => style.align_self = AlignSelf::Center,
                "self-stretch" => style.align_self = AlignSelf::Stretch,
                "self-auto" => style.align_self = AlignSelf::Auto,
                "self-baseline" => style.align_self = AlignSelf::Baseline,
                "justify-normal" => style.justify_content = JustifyContent::Default,
                "justify-start" => style.justify_content = JustifyContent::FlexStart,
                "justify-end" => style.justify_content = JustifyContent::FlexEnd,
                "justify-center" => style.justify_content = JustifyContent::Center,
                "justify-between" => style.justify_content = JustifyContent::SpaceBetween,
                "justify-around" => style.justify_content = JustifyContent::SpaceAround,
                "justify-evenly" => style.justify_content = JustifyContent::SpaceEvenly,
                "justify-stretch" => style.justify_content = JustifyContent::Stretch,
                "aspect-auto" => style.aspect_ratio = None,
                "aspect-square" => style.aspect_ratio = Some(ASPECT_SQUARE),
                "aspect-video" => style.aspect_ratio = Some(ASPECT_VIDEO),
                "grow" => style.flex_grow = 1.0,
                "grow-0" => style.flex_grow = 0.0,
                "shrink" => style.flex_shrink = 1.0,
                "shrink-0" => style.flex_shrink = 0.0,
                "w-0" | "w-px" | "w-0.5" | "w-1" | "w-1.5" | "w-2" | "w-2.5" | "w-3" | "w-3.5"
                | "w-4" | "w-5" | "w-6" | "w-7" | "w-8" | "w-9" | "w-10" | "w-11" | "w-12"
                | "w-14" | "w-16" | "w-20" | "w-24" | "w-28" | "w-32" | "w-36" | "w-40"
                | "w-44" | "w-48" | "w-52" | "w-56" | "w-60" | "w-64" | "w-72" | "w-80"
                | "w-96" => {
                    let value = name["w-".len()..].parse::<f32>().unwrap_or(1.0);
                    style.width = Val::Px(value * REM)
                }
                "w-1/2" | "w-1/3" | "w-2/3" | "w-1/4" | "w-2/4" | "w-3/4" | "w-1/5" | "w-2/5"
                | "w-3/5" | "w-4/5" | "w-1/6" | "w-2/6" | "w-3/6" | "w-4/6" | "w-5/6"
                | "w-1/12" | "w-2/12" | "w-3/12" | "w-4/12" | "w-5/12" | "w-6/12" | "w-7/12"
                | "w-8/12" | "w-9/12" | "w-10/12" | "w-11/12" => {
                    let value = parse_ratio(&name["w-".len()..]).expect("covered by match");
                    style.width = value
                }
                "w-full" => style.width = Val::Percent(100.0),
                "w-screen" => style.width = Val::Vw(100.0),
                "h-0" | "h-px" | "h-0.5" | "h-1" | "h-1.5" | "h-2" | "h-2.5" | "h-3" | "h-3.5"
                | "h-4" | "h-5" | "h-6" | "h-7" | "h-8" | "h-9" | "h-10" | "h-11" | "h-12"
                | "h-14" | "h-16" | "h-20" | "h-24" | "h-28" | "h-32" | "h-36" | "h-40"
                | "h-44" | "h-48" | "h-52" | "h-56" | "h-60" | "h-64" | "h-72" | "h-80"
                | "h-96" => {
                    let value = name["h-".len()..].parse::<f32>().ok()?;
                    style.height = Val::Px(value * REM);
                }
                "h-1/2" | "h-1/3" | "h-2/3" | "h-1/4" | "h-2/4" | "h-3/4" | "h-1/5" | "h-2/5"
                | "h-3/5" | "h-4/5" | "h-1/6" | "h-2/6" | "h-3/6" | "h-4/6" | "h-5/6"
                | "h-1/12" | "h-2/12" | "h-3/12" | "h-4/12" | "h-5/12" | "h-6/12" | "h-7/12"
                | "h-8/12" | "h-9/12" | "h-10/12" | "h-11/12" => {
                    let value = parse_ratio(&name["h-".len()..])?;
                    style.height = value;
                }
                "h-full" => style.height = Val::Percent(100.0),
                "h-screen" => style.height = Val::Vh(100.0),
                width if width.starts_with("w-[") && width.ends_with("]") => {
                    let interpolated_value = &width["w-[".len()..width.len() - 1];
                    if let Some(val) = parse_size(interpolated_value) {
                        style.width = val;
                    }
                }
                height if height.starts_with("h-[") && height.ends_with("]") => {
                    let interpolated_value = &height["h-[".len()..height.len() - 1];
                    if let Some(val) = parse_size(interpolated_value) {
                        style.height = val;
                    }
                }
                "border" | "border-0" | "border-2" | "border-4" | "border-8" => {
                    let value = name["border-".len()..].parse::<f32>().unwrap_or(1.0);
                    style.border = UiRect::all(Val::Px(value))
                }
                "border-t" | "border-t-0" | "border-t-2" | "border-t-4" | "border-t-8" => {
                    let value = name["border-t-".len()..].parse::<f32>().unwrap_or(1.0);
                    style.border.top = Val::Px(value)
                }
                "border-r" | "border-r-0" | "border-r-2" | "border-r-4" | "border-r-8" => {
                    let value = name["border-r-".len()..].parse::<f32>().unwrap_or(1.0);
                    style.border.right = Val::Px(value)
                }
                "border-b" | "border-b-0" | "border-b-2" | "border-b-4" | "border-b-8" => {
                    let value = name["border-b-".len()..].parse::<f32>().unwrap_or(1.0);
                    style.border.bottom = Val::Px(value)
                }
                "border-l" | "border-l-0" | "border-l-2" | "border-l-4" | "border-l-8" => {
                    let value = name["border-l-".len()..].parse::<f32>().unwrap_or(1.0);
                    style.border.left = Val::Px(value)
                }
                "p-0" | "p-px" | "p-0.5" | "p-1" | "p-1.5" | "p-2" | "p-2.5" | "p-3" | "p-3.5"
                | "p-4" | "p-5" | "p-6" | "p-7" | "p-8" | "p-9" | "p-10" | "p-11" | "p-12"
                | "p-14" | "p-16" | "p-20" | "p-24" | "p-28" | "p-32" | "p-36" | "p-40"
                | "p-44" | "p-48" | "p-52" | "p-56" | "p-60" | "p-64" | "p-72" | "p-80"
                | "p-96" => {
                    let value = name["p-".len()..].parse::<f32>();
                    style.padding = UiRect::all(Val::Px(value.map(|v| v * REM).unwrap_or(1.0)))
                }
                "pt-0" | "pt-px" | "pt-0.5" | "pt-1" | "pt-1.5" | "pt-2" | "pt-2.5" | "pt-3"
                | "pt-3.5" | "pt-4" | "pt-5" | "pt-6" | "pt-7" | "pt-8" | "pt-9" | "pt-10"
                | "pt-11" | "pt-12" | "pt-14" | "pt-16" | "pt-20" | "pt-24" | "pt-28" | "pt-32"
                | "pt-36" | "pt-40" | "pt-44" | "pt-48" | "pt-52" | "pt-56" | "pt-60" | "pt-64"
                | "pt-72" | "pt-80" | "pt-96" => {
                    let value = name["pt-".len()..].parse::<f32>();
                    style.padding.top = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "pr-0" | "pr-px" | "pr-0.5" | "pr-1" | "pr-1.5" | "pr-2" | "pr-2.5" | "pr-3"
                | "pr-3.5" | "pr-4" | "pr-5" | "pr-6" | "pr-7" | "pr-8" | "pr-9" | "pr-10"
                | "pr-11" | "pr-12" | "pr-14" | "pr-16" | "pr-20" | "pr-24" | "pr-28" | "pr-32"
                | "pr-36" | "pr-40" | "pr-44" | "pr-48" | "pr-52" | "pr-56" | "pr-60" | "pr-64"
                | "pr-72" | "pr-80" | "pr-96" => {
                    let value = name["pr-".len()..].parse::<f32>();
                    style.padding.right = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "pb-0" | "pb-px" | "pb-0.5" | "pb-1" | "pb-1.5" | "pb-2" | "pb-2.5" | "pb-3"
                | "pb-3.5" | "pb-4" | "pb-5" | "pb-6" | "pb-7" | "pb-8" | "pb-9" | "pb-10"
                | "pb-11" | "pb-12" | "pb-14" | "pb-16" | "pb-20" | "pb-24" | "pb-28" | "pb-32"
                | "pb-36" | "pb-40" | "pb-44" | "pb-48" | "pb-52" | "pb-56" | "pb-60" | "pb-64"
                | "pb-72" | "pb-80" | "pb-96" => {
                    let value = name["pb-".len()..].parse::<f32>();
                    style.padding.bottom = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "pl-0" | "pl-px" | "pl-0.5" | "pl-1" | "pl-1.5" | "pl-2" | "pl-2.5" | "pl-3"
                | "pl-3.5" | "pl-4" | "pl-5" | "pl-6" | "pl-7" | "pl-8" | "pl-9" | "pl-10"
                | "pl-11" | "pl-12" | "pl-14" | "pl-16" | "pl-20" | "pl-24" | "pl-28" | "pl-32"
                | "pl-36" | "pl-40" | "pl-44" | "pl-48" | "pl-52" | "pl-56" | "pl-60" | "pl-64"
                | "pl-72" | "pl-80" | "pl-96" => {
                    let value = name["pl-".len()..].parse::<f32>();
                    style.padding.left = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "px-0" | "px-px" | "px-0.5" | "px-1" | "px-1.5" | "px-2" | "px-2.5" | "px-3"
                | "px-3.5" | "px-4" | "px-5" | "px-6" | "px-7" | "px-8" | "px-9" | "px-10"
                | "px-11" | "px-12" | "px-14" | "px-16" | "px-20" | "px-24" | "px-28" | "px-32"
                | "px-36" | "px-40" | "px-44" | "px-48" | "px-52" | "px-56" | "px-60" | "px-64"
                | "px-72" | "px-80" | "px-96" => {
                    let number = name["px-".len()..].parse::<f32>();
                    let value = Val::Px(number.map(|v| v * REM).unwrap_or(1.0));
                    style.padding.left = value;
                    style.padding.right = value;
                }
                "py-0" | "py-px" | "py-0.5" | "py-1" | "py-1.5" | "py-2" | "py-2.5" | "py-3"
                | "py-3.5" | "py-4" | "py-5" | "py-6" | "py-7" | "py-8" | "py-9" | "py-10"
                | "py-11" | "py-12" | "py-14" | "py-16" | "py-20" | "py-24" | "py-28" | "py-32"
                | "py-36" | "py-40" | "py-44" | "py-48" | "py-52" | "py-56" | "py-60" | "py-64"
                | "py-72" | "py-80" | "py-96" => {
                    let number = name["py-".len()..].parse::<f32>();
                    let value = Val::Px(number.map(|v| v * REM).unwrap_or(1.0));
                    style.padding.top = value;
                    style.padding.bottom = value;
                }
                "m-0" | "m-px" | "m-0.5" | "m-1" | "m-1.5" | "m-2" | "m-2.5" | "m-3" | "m-3.5"
                | "m-4" | "m-5" | "m-6" | "m-7" | "m-8" | "m-9" | "m-10" | "m-11" | "m-12"
                | "m-14" | "m-16" | "m-20" | "m-24" | "m-28" | "m-32" | "m-36" | "m-40"
                | "m-44" | "m-48" | "m-52" | "m-56" | "m-60" | "m-64" | "m-72" | "m-80"
                | "m-96" => {
                    let value = name["m-".len()..].parse::<f32>();
                    style.margin = UiRect::all(Val::Px(value.map(|v| v * REM).unwrap_or(1.0)))
                }
                "mt-0" | "mt-px" | "mt-0.5" | "mt-1" | "mt-1.5" | "mt-2" | "mt-2.5" | "mt-3"
                | "mt-3.5" | "mt-4" | "mt-5" | "mt-6" | "mt-7" | "mt-8" | "mt-9" | "mt-10"
                | "mt-11" | "mt-12" | "mt-14" | "mt-16" | "mt-20" | "mt-24" | "mt-28" | "mt-32"
                | "mt-36" | "mt-40" | "mt-44" | "mt-48" | "mt-52" | "mt-56" | "mt-60" | "mt-64"
                | "mt-72" | "mt-80" | "mt-96" => {
                    let value = name["mt-".len()..].parse::<f32>();
                    style.margin.top = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "mr-0" | "mr-px" | "mr-0.5" | "mr-1" | "mr-1.5" | "mr-2" | "mr-2.5" | "mr-3"
                | "mr-3.5" | "mr-4" | "mr-5" | "mr-6" | "mr-7" | "mr-8" | "mr-9" | "mr-10"
                | "mr-11" | "mr-12" | "mr-14" | "mr-16" | "mr-20" | "mr-24" | "mr-28" | "mr-32"
                | "mr-36" | "mr-40" | "mr-44" | "mr-48" | "mr-52" | "mr-56" | "mr-60" | "mr-64"
                | "mr-72" | "mr-80" | "mr-96" => {
                    let value = name["mr-".len()..].parse::<f32>();
                    style.margin.right = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "mb-0" | "mb-px" | "mb-0.5" | "mb-1" | "mb-1.5" | "mb-2" | "mb-2.5" | "mb-3"
                | "mb-3.5" | "mb-4" | "mb-5" | "mb-6" | "mb-7" | "mb-8" | "mb-9" | "mb-10"
                | "mb-11" | "mb-12" | "mb-14" | "mb-16" | "mb-20" | "mb-24" | "mb-28" | "mb-32"
                | "mb-36" | "mb-40" | "mb-44" | "mb-48" | "mb-52" | "mb-56" | "mb-60" | "mb-64"
                | "mb-72" | "mb-80" | "mb-96" => {
                    let value = name["mb-".len()..].parse::<f32>();
                    style.margin.bottom = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "ml-0" | "ml-px" | "ml-0.5" | "ml-1" | "ml-1.5" | "ml-2" | "ml-2.5" | "ml-3"
                | "ml-3.5" | "ml-4" | "ml-5" | "ml-6" | "ml-7" | "ml-8" | "ml-9" | "ml-10"
                | "ml-11" | "ml-12" | "ml-14" | "ml-16" | "ml-20" | "ml-24" | "ml-28" | "ml-32"
                | "ml-36" | "ml-40" | "ml-44" | "ml-48" | "ml-52" | "ml-56" | "ml-60" | "ml-64"
                | "ml-72" | "ml-80" | "ml-96" => {
                    let value = name["ml-".len()..].parse::<f32>();
                    style.margin.left = Val::Px(value.map(|v| v * REM).unwrap_or(1.0))
                }
                "mx-0" | "mx-px" | "mx-0.5" | "mx-1" | "mx-1.5" | "mx-2" | "mx-2.5" | "mx-3"
                | "mx-3.5" | "mx-4" | "mx-5" | "mx-6" | "mx-7" | "mx-8" | "mx-9" | "mx-10"
                | "mx-11" | "mx-12" | "mx-14" | "mx-16" | "mx-20" | "mx-24" | "mx-28" | "mx-32"
                | "mx-36" | "mx-40" | "mx-44" | "mx-48" | "mx-52" | "mx-56" | "mx-60" | "mx-64"
                | "mx-72" | "mx-80" | "mx-96" => {
                    let number = name["mx-".len()..].parse::<f32>();
                    let value = Val::Px(number.map(|v| v * REM).unwrap_or(1.0));
                    style.margin.left = value;
                    style.margin.right = value;
                }
                "my-0" | "my-px" | "my-0.5" | "my-1" | "my-1.5" | "my-2" | "my-2.5" | "my-3"
                | "my-3.5" | "my-4" | "my-5" | "my-6" | "my-7" | "my-8" | "my-9" | "my-10"
                | "my-11" | "my-12" | "my-14" | "my-16" | "my-20" | "my-24" | "my-28" | "my-32"
                | "my-36" | "my-40" | "my-44" | "my-48" | "my-52" | "my-56" | "my-60" | "my-64"
                | "my-72" | "my-80" | "my-96" => {
                    let number = name["my-".len()..].parse::<f32>();
                    let value = Val::Px(number.map(|v| v * REM).unwrap_or(1.0));
                    style.margin.top = value;
                    style.margin.bottom = value;
                }

                _ => {
                    println!("Unknown name: {}", name)
                }
            }
        }

        Some(style)
    }
    fn get_background_color(&self, names: &str) -> Option<BackgroundColor> {
        for name in names.split(" ") {
            let (color_string, alpha_string) = name.split_once("/").unwrap_or((name, "255"));
            let alpha = alpha_string.parse::<u8>().unwrap_or(255);
            let color = match color_string {
                "bg-black" => Some(BackgroundColor(Color::rgba_u8(0, 0, 0, alpha))),
                "bg-white" => Some(BackgroundColor(Color::rgba_u8(255, 255, 255, alpha))),
                "bg-slate-50" => Some(BackgroundColor(Color::rgba_u8(248, 250, 252, alpha))),
                "bg-slate-100" => Some(BackgroundColor(Color::rgba_u8(241, 245, 249, alpha))),
                "bg-slate-200" => Some(BackgroundColor(Color::rgba_u8(226, 232, 240, alpha))),
                "bg-slate-300" => Some(BackgroundColor(Color::rgba_u8(203, 213, 225, alpha))),
                "bg-slate-400" => Some(BackgroundColor(Color::rgba_u8(148, 163, 184, alpha))),
                "bg-slate-500" => Some(BackgroundColor(Color::rgba_u8(100, 116, 139, alpha))),
                "bg-slate-600" => Some(BackgroundColor(Color::rgba_u8(71, 85, 105, alpha))),
                "bg-slate-700" => Some(BackgroundColor(Color::rgba_u8(51, 65, 85, alpha))),
                "bg-slate-800" => Some(BackgroundColor(Color::rgba_u8(30, 41, 59, alpha))),
                "bg-slate-900" => Some(BackgroundColor(Color::rgba_u8(15, 23, 42, alpha))),
                "bg-slate-950" => Some(BackgroundColor(Color::rgba_u8(2, 6, 23, alpha))),
                "bg-gray-50" => Some(BackgroundColor(Color::rgba_u8(249, 250, 251, alpha))),
                "bg-gray-100" => Some(BackgroundColor(Color::rgba_u8(243, 244, 246, alpha))),
                "bg-gray-200" => Some(BackgroundColor(Color::rgba_u8(229, 231, 235, alpha))),
                "bg-gray-300" => Some(BackgroundColor(Color::rgba_u8(209, 213, 219, alpha))),
                "bg-gray-400" => Some(BackgroundColor(Color::rgba_u8(156, 163, 175, alpha))),
                "bg-gray-500" => Some(BackgroundColor(Color::rgba_u8(107, 114, 128, alpha))),
                "bg-gray-600" => Some(BackgroundColor(Color::rgba_u8(75, 85, 99, alpha))),
                "bg-gray-700" => Some(BackgroundColor(Color::rgba_u8(55, 65, 81, alpha))),
                "bg-gray-800" => Some(BackgroundColor(Color::rgba_u8(31, 41, 55, alpha))),
                "bg-gray-900" => Some(BackgroundColor(Color::rgba_u8(17, 24, 39, alpha))),
                "bg-gray-950" => Some(BackgroundColor(Color::rgba_u8(3, 7, 18, alpha))),
                "bg-zinc-50" => Some(BackgroundColor(Color::rgba_u8(250, 250, 250, alpha))),
                "bg-zinc-100" => Some(BackgroundColor(Color::rgba_u8(244, 244, 245, alpha))),
                "bg-zinc-200" => Some(BackgroundColor(Color::rgba_u8(228, 228, 231, alpha))),
                "bg-zinc-300" => Some(BackgroundColor(Color::rgba_u8(212, 212, 216, alpha))),
                "bg-zinc-400" => Some(BackgroundColor(Color::rgba_u8(161, 161, 170, alpha))),
                "bg-zinc-500" => Some(BackgroundColor(Color::rgba_u8(113, 113, 122, alpha))),
                "bg-zinc-600" => Some(BackgroundColor(Color::rgba_u8(82, 82, 91, alpha))),
                "bg-zinc-700" => Some(BackgroundColor(Color::rgba_u8(63, 63, 70, alpha))),
                "bg-zinc-800" => Some(BackgroundColor(Color::rgba_u8(39, 39, 42, alpha))),
                "bg-zinc-900" => Some(BackgroundColor(Color::rgba_u8(24, 24, 27, alpha))),
                "bg-zinc-950" => Some(BackgroundColor(Color::rgba_u8(9, 9, 11, alpha))),
                "bg-neutral-50" => Some(BackgroundColor(Color::rgba_u8(250, 250, 250, alpha))),
                "bg-neutral-100" => Some(BackgroundColor(Color::rgba_u8(245, 245, 245, alpha))),
                "bg-neutral-200" => Some(BackgroundColor(Color::rgba_u8(229, 229, 229, alpha))),
                "bg-neutral-300" => Some(BackgroundColor(Color::rgba_u8(212, 212, 212, alpha))),
                "bg-neutral-400" => Some(BackgroundColor(Color::rgba_u8(163, 163, 163, alpha))),
                "bg-neutral-500" => Some(BackgroundColor(Color::rgba_u8(115, 115, 115, alpha))),
                "bg-neutral-600" => Some(BackgroundColor(Color::rgba_u8(82, 82, 82, alpha))),
                "bg-neutral-700" => Some(BackgroundColor(Color::rgba_u8(64, 64, 64, alpha))),
                "bg-neutral-800" => Some(BackgroundColor(Color::rgba_u8(38, 38, 38, alpha))),
                "bg-neutral-900" => Some(BackgroundColor(Color::rgba_u8(23, 23, 23, alpha))),
                "bg-neutral-950" => Some(BackgroundColor(Color::rgba_u8(10, 10, 10, alpha))),
                "bg-stone-50" => Some(BackgroundColor(Color::rgba_u8(250, 250, 249, alpha))),
                "bg-stone-100" => Some(BackgroundColor(Color::rgba_u8(245, 245, 244, alpha))),
                "bg-stone-200" => Some(BackgroundColor(Color::rgba_u8(231, 229, 228, alpha))),
                "bg-stone-300" => Some(BackgroundColor(Color::rgba_u8(214, 211, 209, alpha))),
                "bg-stone-400" => Some(BackgroundColor(Color::rgba_u8(168, 162, 158, alpha))),
                "bg-stone-500" => Some(BackgroundColor(Color::rgba_u8(120, 113, 108, alpha))),
                "bg-stone-600" => Some(BackgroundColor(Color::rgba_u8(87, 83, 78, alpha))),
                "bg-stone-700" => Some(BackgroundColor(Color::rgba_u8(68, 64, 60, alpha))),
                "bg-stone-800" => Some(BackgroundColor(Color::rgba_u8(41, 37, 36, alpha))),
                "bg-stone-900" => Some(BackgroundColor(Color::rgba_u8(28, 25, 23, alpha))),
                "bg-stone-950" => Some(BackgroundColor(Color::rgba_u8(12, 10, 9, alpha))),
                "bg-red-50" => Some(BackgroundColor(Color::rgba_u8(254, 242, 242, alpha))),
                "bg-red-100" => Some(BackgroundColor(Color::rgba_u8(254, 226, 226, alpha))),
                "bg-red-200" => Some(BackgroundColor(Color::rgba_u8(254, 202, 202, alpha))),
                "bg-red-300" => Some(BackgroundColor(Color::rgba_u8(252, 165, 165, alpha))),
                "bg-red-400" => Some(BackgroundColor(Color::rgba_u8(248, 113, 113, alpha))),
                "bg-red-500" => Some(BackgroundColor(Color::rgba_u8(239, 68, 68, alpha))),
                "bg-red-600" => Some(BackgroundColor(Color::rgba_u8(220, 38, 38, alpha))),
                "bg-red-700" => Some(BackgroundColor(Color::rgba_u8(185, 28, 28, alpha))),
                "bg-red-800" => Some(BackgroundColor(Color::rgba_u8(153, 27, 27, alpha))),
                "bg-red-900" => Some(BackgroundColor(Color::rgba_u8(127, 29, 29, alpha))),
                "bg-red-950" => Some(BackgroundColor(Color::rgba_u8(69, 10, 10, alpha))),
                "bg-orange-50" => Some(BackgroundColor(Color::rgba_u8(255, 247, 237, alpha))),
                "bg-orange-100" => Some(BackgroundColor(Color::rgba_u8(255, 237, 213, alpha))),
                "bg-orange-200" => Some(BackgroundColor(Color::rgba_u8(254, 215, 170, alpha))),
                "bg-orange-300" => Some(BackgroundColor(Color::rgba_u8(253, 186, 116, alpha))),
                "bg-orange-400" => Some(BackgroundColor(Color::rgba_u8(251, 146, 60, alpha))),
                "bg-orange-500" => Some(BackgroundColor(Color::rgba_u8(249, 115, 22, alpha))),
                "bg-orange-600" => Some(BackgroundColor(Color::rgba_u8(234, 88, 12, alpha))),
                "bg-orange-700" => Some(BackgroundColor(Color::rgba_u8(194, 65, 12, alpha))),
                "bg-orange-800" => Some(BackgroundColor(Color::rgba_u8(154, 52, 18, alpha))),
                "bg-orange-900" => Some(BackgroundColor(Color::rgba_u8(124, 45, 18, alpha))),
                "bg-orange-950" => Some(BackgroundColor(Color::rgba_u8(67, 20, 7, alpha))),
                "bg-amber-50" => Some(BackgroundColor(Color::rgba_u8(255, 251, 235, alpha))),
                "bg-amber-100" => Some(BackgroundColor(Color::rgba_u8(254, 243, 199, alpha))),
                "bg-amber-200" => Some(BackgroundColor(Color::rgba_u8(253, 230, 138, alpha))),
                "bg-amber-300" => Some(BackgroundColor(Color::rgba_u8(252, 211, 77, alpha))),
                "bg-amber-400" => Some(BackgroundColor(Color::rgba_u8(251, 191, 36, alpha))),
                "bg-amber-500" => Some(BackgroundColor(Color::rgba_u8(245, 158, 11, alpha))),
                "bg-amber-600" => Some(BackgroundColor(Color::rgba_u8(217, 119, 6, alpha))),
                "bg-amber-700" => Some(BackgroundColor(Color::rgba_u8(180, 83, 9, alpha))),
                "bg-amber-800" => Some(BackgroundColor(Color::rgba_u8(146, 64, 14, alpha))),
                "bg-amber-900" => Some(BackgroundColor(Color::rgba_u8(120, 53, 15, alpha))),
                "bg-amber-950" => Some(BackgroundColor(Color::rgba_u8(69, 26, 3, alpha))),
                "bg-yellow-50" => Some(BackgroundColor(Color::rgba_u8(254, 252, 232, alpha))),
                "bg-yellow-100" => Some(BackgroundColor(Color::rgba_u8(254, 249, 195, alpha))),
                "bg-yellow-200" => Some(BackgroundColor(Color::rgba_u8(254, 240, 138, alpha))),
                "bg-yellow-300" => Some(BackgroundColor(Color::rgba_u8(253, 224, 71, alpha))),
                "bg-yellow-400" => Some(BackgroundColor(Color::rgba_u8(250, 204, 21, alpha))),
                "bg-yellow-500" => Some(BackgroundColor(Color::rgba_u8(234, 179, 8, alpha))),
                "bg-yellow-600" => Some(BackgroundColor(Color::rgba_u8(202, 138, 4, alpha))),
                "bg-yellow-700" => Some(BackgroundColor(Color::rgba_u8(161, 98, 7, alpha))),
                "bg-yellow-800" => Some(BackgroundColor(Color::rgba_u8(133, 77, 14, alpha))),
                "bg-yellow-900" => Some(BackgroundColor(Color::rgba_u8(113, 63, 18, alpha))),
                "bg-yellow-950" => Some(BackgroundColor(Color::rgba_u8(66, 32, 6, alpha))),
                "bg-lime-50" => Some(BackgroundColor(Color::rgba_u8(247, 254, 231, alpha))),
                "bg-lime-100" => Some(BackgroundColor(Color::rgba_u8(236, 252, 203, alpha))),
                "bg-lime-200" => Some(BackgroundColor(Color::rgba_u8(217, 249, 157, alpha))),
                "bg-lime-300" => Some(BackgroundColor(Color::rgba_u8(190, 242, 100, alpha))),
                "bg-lime-400" => Some(BackgroundColor(Color::rgba_u8(163, 230, 53, alpha))),
                "bg-lime-500" => Some(BackgroundColor(Color::rgba_u8(132, 204, 22, alpha))),
                "bg-lime-600" => Some(BackgroundColor(Color::rgba_u8(101, 163, 13, alpha))),
                "bg-lime-700" => Some(BackgroundColor(Color::rgba_u8(77, 124, 15, alpha))),
                "bg-lime-800" => Some(BackgroundColor(Color::rgba_u8(63, 98, 18, alpha))),
                "bg-lime-900" => Some(BackgroundColor(Color::rgba_u8(54, 83, 20, alpha))),
                "bg-lime-950" => Some(BackgroundColor(Color::rgba_u8(26, 46, 5, alpha))),
                "bg-green-50" => Some(BackgroundColor(Color::rgba_u8(240, 253, 244, alpha))),
                "bg-green-100" => Some(BackgroundColor(Color::rgba_u8(220, 252, 231, alpha))),
                "bg-green-200" => Some(BackgroundColor(Color::rgba_u8(187, 247, 208, alpha))),
                "bg-green-300" => Some(BackgroundColor(Color::rgba_u8(134, 239, 172, alpha))),
                "bg-green-400" => Some(BackgroundColor(Color::rgba_u8(74, 222, 128, alpha))),
                "bg-green-500" => Some(BackgroundColor(Color::rgba_u8(34, 197, 94, alpha))),
                "bg-green-600" => Some(BackgroundColor(Color::rgba_u8(22, 163, 74, alpha))),
                "bg-green-700" => Some(BackgroundColor(Color::rgba_u8(21, 128, 61, alpha))),
                "bg-green-800" => Some(BackgroundColor(Color::rgba_u8(22, 101, 52, alpha))),
                "bg-green-900" => Some(BackgroundColor(Color::rgba_u8(20, 83, 45, alpha))),
                "bg-green-950" => Some(BackgroundColor(Color::rgba_u8(5, 46, 22, alpha))),
                "bg-emerald-50" => Some(BackgroundColor(Color::rgba_u8(236, 253, 245, alpha))),
                "bg-emerald-100" => Some(BackgroundColor(Color::rgba_u8(209, 250, 229, alpha))),
                "bg-emerald-200" => Some(BackgroundColor(Color::rgba_u8(167, 243, 208, alpha))),
                "bg-emerald-300" => Some(BackgroundColor(Color::rgba_u8(110, 231, 183, alpha))),
                "bg-emerald-400" => Some(BackgroundColor(Color::rgba_u8(52, 211, 153, alpha))),
                "bg-emerald-500" => Some(BackgroundColor(Color::rgba_u8(16, 185, 129, alpha))),
                "bg-emerald-600" => Some(BackgroundColor(Color::rgba_u8(5, 150, 105, alpha))),
                "bg-emerald-700" => Some(BackgroundColor(Color::rgba_u8(4, 120, 87, alpha))),
                "bg-emerald-800" => Some(BackgroundColor(Color::rgba_u8(6, 95, 70, alpha))),
                "bg-emerald-900" => Some(BackgroundColor(Color::rgba_u8(6, 78, 59, alpha))),
                "bg-emerald-950" => Some(BackgroundColor(Color::rgba_u8(2, 44, 34, alpha))),
                "bg-teal-50" => Some(BackgroundColor(Color::rgba_u8(240, 253, 250, alpha))),
                "bg-teal-100" => Some(BackgroundColor(Color::rgba_u8(204, 251, 241, alpha))),
                "bg-teal-200" => Some(BackgroundColor(Color::rgba_u8(153, 246, 228, alpha))),
                "bg-teal-300" => Some(BackgroundColor(Color::rgba_u8(94, 234, 212, alpha))),
                "bg-teal-400" => Some(BackgroundColor(Color::rgba_u8(45, 212, 191, alpha))),
                "bg-teal-500" => Some(BackgroundColor(Color::rgba_u8(20, 184, 166, alpha))),
                "bg-teal-600" => Some(BackgroundColor(Color::rgba_u8(13, 148, 136, alpha))),
                "bg-teal-700" => Some(BackgroundColor(Color::rgba_u8(15, 118, 110, alpha))),
                "bg-teal-800" => Some(BackgroundColor(Color::rgba_u8(17, 94, 89, alpha))),
                "bg-teal-900" => Some(BackgroundColor(Color::rgba_u8(19, 78, 74, alpha))),
                "bg-teal-950" => Some(BackgroundColor(Color::rgba_u8(4, 47, 46, alpha))),
                "bg-cyan-50" => Some(BackgroundColor(Color::rgba_u8(236, 254, 255, alpha))),
                "bg-cyan-100" => Some(BackgroundColor(Color::rgba_u8(207, 250, 254, alpha))),
                "bg-cyan-200" => Some(BackgroundColor(Color::rgba_u8(165, 243, 252, alpha))),
                "bg-cyan-300" => Some(BackgroundColor(Color::rgba_u8(103, 232, 249, alpha))),
                "bg-cyan-400" => Some(BackgroundColor(Color::rgba_u8(34, 211, 238, alpha))),
                "bg-cyan-500" => Some(BackgroundColor(Color::rgba_u8(6, 182, 212, alpha))),
                "bg-cyan-600" => Some(BackgroundColor(Color::rgba_u8(8, 145, 178, alpha))),
                "bg-cyan-700" => Some(BackgroundColor(Color::rgba_u8(14, 116, 144, alpha))),
                "bg-cyan-800" => Some(BackgroundColor(Color::rgba_u8(21, 94, 117, alpha))),
                "bg-cyan-900" => Some(BackgroundColor(Color::rgba_u8(22, 78, 99, alpha))),
                "bg-cyan-950" => Some(BackgroundColor(Color::rgba_u8(8, 51, 68, alpha))),
                "bg-sky-50" => Some(BackgroundColor(Color::rgba_u8(240, 249, 255, alpha))),
                "bg-sky-100" => Some(BackgroundColor(Color::rgba_u8(224, 242, 254, alpha))),
                "bg-sky-200" => Some(BackgroundColor(Color::rgba_u8(186, 230, 253, alpha))),
                "bg-sky-300" => Some(BackgroundColor(Color::rgba_u8(125, 211, 252, alpha))),
                "bg-sky-400" => Some(BackgroundColor(Color::rgba_u8(56, 189, 248, alpha))),
                "bg-sky-500" => Some(BackgroundColor(Color::rgba_u8(14, 165, 233, alpha))),
                "bg-sky-600" => Some(BackgroundColor(Color::rgba_u8(2, 132, 199, alpha))),
                "bg-sky-700" => Some(BackgroundColor(Color::rgba_u8(3, 105, 161, alpha))),
                "bg-sky-800" => Some(BackgroundColor(Color::rgba_u8(7, 89, 133, alpha))),
                "bg-sky-900" => Some(BackgroundColor(Color::rgba_u8(12, 74, 110, alpha))),
                "bg-sky-950" => Some(BackgroundColor(Color::rgba_u8(8, 47, 73, alpha))),
                "bg-blue-50" => Some(BackgroundColor(Color::rgba_u8(239, 246, 255, alpha))),
                "bg-blue-100" => Some(BackgroundColor(Color::rgba_u8(219, 234, 254, alpha))),
                "bg-blue-200" => Some(BackgroundColor(Color::rgba_u8(191, 219, 254, alpha))),
                "bg-blue-300" => Some(BackgroundColor(Color::rgba_u8(147, 197, 253, alpha))),
                "bg-blue-400" => Some(BackgroundColor(Color::rgba_u8(96, 165, 250, alpha))),
                "bg-blue-500" => Some(BackgroundColor(Color::rgba_u8(59, 130, 246, alpha))),
                "bg-blue-600" => Some(BackgroundColor(Color::rgba_u8(37, 99, 235, alpha))),
                "bg-blue-700" => Some(BackgroundColor(Color::rgba_u8(29, 78, 216, alpha))),
                "bg-blue-800" => Some(BackgroundColor(Color::rgba_u8(30, 64, 175, alpha))),
                "bg-blue-900" => Some(BackgroundColor(Color::rgba_u8(30, 58, 138, alpha))),
                "bg-blue-950" => Some(BackgroundColor(Color::rgba_u8(23, 37, 84, alpha))),
                "bg-indigo-50" => Some(BackgroundColor(Color::rgba_u8(238, 242, 255, alpha))),
                "bg-indigo-100" => Some(BackgroundColor(Color::rgba_u8(224, 231, 255, alpha))),
                "bg-indigo-200" => Some(BackgroundColor(Color::rgba_u8(199, 210, 254, alpha))),
                "bg-indigo-300" => Some(BackgroundColor(Color::rgba_u8(165, 180, 252, alpha))),
                "bg-indigo-400" => Some(BackgroundColor(Color::rgba_u8(129, 140, 248, alpha))),
                "bg-indigo-500" => Some(BackgroundColor(Color::rgba_u8(99, 102, 241, alpha))),
                "bg-indigo-600" => Some(BackgroundColor(Color::rgba_u8(79, 70, 229, alpha))),
                "bg-indigo-700" => Some(BackgroundColor(Color::rgba_u8(67, 56, 202, alpha))),
                "bg-indigo-800" => Some(BackgroundColor(Color::rgba_u8(55, 48, 163, alpha))),
                "bg-indigo-900" => Some(BackgroundColor(Color::rgba_u8(49, 46, 129, alpha))),
                "bg-indigo-950" => Some(BackgroundColor(Color::rgba_u8(30, 27, 75, alpha))),
                "bg-violet-50" => Some(BackgroundColor(Color::rgba_u8(245, 243, 255, alpha))),
                "bg-violet-100" => Some(BackgroundColor(Color::rgba_u8(237, 233, 254, alpha))),
                "bg-violet-200" => Some(BackgroundColor(Color::rgba_u8(221, 214, 254, alpha))),
                "bg-violet-300" => Some(BackgroundColor(Color::rgba_u8(196, 181, 253, alpha))),
                "bg-violet-400" => Some(BackgroundColor(Color::rgba_u8(167, 139, 250, alpha))),
                "bg-violet-500" => Some(BackgroundColor(Color::rgba_u8(139, 92, 246, alpha))),
                "bg-violet-600" => Some(BackgroundColor(Color::rgba_u8(124, 58, 237, alpha))),
                "bg-violet-700" => Some(BackgroundColor(Color::rgba_u8(109, 40, 217, alpha))),
                "bg-violet-800" => Some(BackgroundColor(Color::rgba_u8(91, 33, 182, alpha))),
                "bg-violet-900" => Some(BackgroundColor(Color::rgba_u8(76, 29, 149, alpha))),
                "bg-violet-950" => Some(BackgroundColor(Color::rgba_u8(46, 16, 101, alpha))),
                "bg-purple-50" => Some(BackgroundColor(Color::rgba_u8(250, 245, 255, alpha))),
                "bg-purple-100" => Some(BackgroundColor(Color::rgba_u8(243, 232, 255, alpha))),
                "bg-purple-200" => Some(BackgroundColor(Color::rgba_u8(233, 213, 255, alpha))),
                "bg-purple-300" => Some(BackgroundColor(Color::rgba_u8(216, 180, 254, alpha))),
                "bg-purple-400" => Some(BackgroundColor(Color::rgba_u8(192, 132, 252, alpha))),
                "bg-purple-500" => Some(BackgroundColor(Color::rgba_u8(168, 85, 247, alpha))),
                "bg-purple-600" => Some(BackgroundColor(Color::rgba_u8(147, 51, 234, alpha))),
                "bg-purple-700" => Some(BackgroundColor(Color::rgba_u8(126, 34, 206, alpha))),
                "bg-purple-800" => Some(BackgroundColor(Color::rgba_u8(107, 33, 168, alpha))),
                "bg-purple-900" => Some(BackgroundColor(Color::rgba_u8(88, 28, 135, alpha))),
                "bg-purple-950" => Some(BackgroundColor(Color::rgba_u8(59, 7, 100, alpha))),
                "bg-fuchsia-50" => Some(BackgroundColor(Color::rgba_u8(253, 244, 255, alpha))),
                "bg-fuchsia-100" => Some(BackgroundColor(Color::rgba_u8(250, 232, 255, alpha))),
                "bg-fuchsia-200" => Some(BackgroundColor(Color::rgba_u8(245, 208, 254, alpha))),
                "bg-fuchsia-300" => Some(BackgroundColor(Color::rgba_u8(240, 171, 252, alpha))),
                "bg-fuchsia-400" => Some(BackgroundColor(Color::rgba_u8(232, 121, 249, alpha))),
                "bg-fuchsia-500" => Some(BackgroundColor(Color::rgba_u8(217, 70, 239, alpha))),
                "bg-fuchsia-600" => Some(BackgroundColor(Color::rgba_u8(192, 38, 211, alpha))),
                "bg-fuchsia-700" => Some(BackgroundColor(Color::rgba_u8(162, 28, 175, alpha))),
                "bg-fuchsia-800" => Some(BackgroundColor(Color::rgba_u8(134, 25, 143, alpha))),
                "bg-fuchsia-900" => Some(BackgroundColor(Color::rgba_u8(112, 26, 117, alpha))),
                "bg-fuchsia-950" => Some(BackgroundColor(Color::rgba_u8(74, 4, 78, alpha))),
                "bg-pink-50" => Some(BackgroundColor(Color::rgba_u8(253, 242, 248, alpha))),
                "bg-pink-100" => Some(BackgroundColor(Color::rgba_u8(252, 231, 243, alpha))),
                "bg-pink-200" => Some(BackgroundColor(Color::rgba_u8(251, 207, 232, alpha))),
                "bg-pink-300" => Some(BackgroundColor(Color::rgba_u8(249, 168, 212, alpha))),
                "bg-pink-400" => Some(BackgroundColor(Color::rgba_u8(244, 114, 182, alpha))),
                "bg-pink-500" => Some(BackgroundColor(Color::rgba_u8(236, 72, 153, alpha))),
                "bg-pink-600" => Some(BackgroundColor(Color::rgba_u8(219, 39, 119, alpha))),
                "bg-pink-700" => Some(BackgroundColor(Color::rgba_u8(190, 24, 93, alpha))),
                "bg-pink-800" => Some(BackgroundColor(Color::rgba_u8(157, 23, 77, alpha))),
                "bg-pink-900" => Some(BackgroundColor(Color::rgba_u8(131, 24, 67, alpha))),
                "bg-pink-950" => Some(BackgroundColor(Color::rgba_u8(80, 7, 36, alpha))),
                "bg-rose-50" => Some(BackgroundColor(Color::rgba_u8(255, 241, 242, alpha))),
                "bg-rose-100" => Some(BackgroundColor(Color::rgba_u8(255, 228, 230, alpha))),
                "bg-rose-200" => Some(BackgroundColor(Color::rgba_u8(254, 205, 211, alpha))),
                "bg-rose-300" => Some(BackgroundColor(Color::rgba_u8(253, 164, 175, alpha))),
                "bg-rose-400" => Some(BackgroundColor(Color::rgba_u8(251, 113, 133, alpha))),
                "bg-rose-500" => Some(BackgroundColor(Color::rgba_u8(244, 63, 94, alpha))),
                "bg-rose-600" => Some(BackgroundColor(Color::rgba_u8(225, 29, 72, alpha))),
                "bg-rose-700" => Some(BackgroundColor(Color::rgba_u8(190, 18, 60, alpha))),
                "bg-rose-800" => Some(BackgroundColor(Color::rgba_u8(159, 18, 57, alpha))),
                "bg-rose-900" => Some(BackgroundColor(Color::rgba_u8(136, 19, 55, alpha))),
                "bg-rose-950" => Some(BackgroundColor(Color::rgba_u8(76, 5, 25, alpha))),
                _ => None,
            };
            if color.is_some() {
                return color;
            }
        }
        None
    }
}

fn parse_size(size: &str) -> Option<Val> {
    if size.ends_with("%") {
        return size[0..size.len() - 1]
            .parse::<f32>()
            .map(Val::Percent)
            .ok();
    }

    if size.ends_with("px") {
        return size[0..size.len() - 2].parse::<f32>().map(Val::Px).ok();
    }

    if size.contains('/') {
        return parse_ratio(&size);
    }

    None
}

fn parse_ratio(s: &str) -> Option<Val> {
    let (fst, snd) = s.split_once('/').unwrap();
    let numerator = fst.parse::<u32>().ok()?;
    let denominator = snd.parse::<u32>().ok()?;
    Some(Val::Percent(numerator as f32 / denominator as f32 * 100.0))
}

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    let mut app = App::new();

    let window = WindowDescriptor {
        title: "Lobby Game".to_string(),
        resizable: false,
        canvas: Some(format!("#app")),
        height: 800.0,
        width: 800.0,
        decorations: false,
        fit_canvas_to_parent: true,
        mode: WindowMode::BorderlessFullscreen,
        ..default()
    };

    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window,
                    ..default()
                })
                .set(AssetPlugin {
                    asset_folder: "static/assets".to_string(),
                    ..default()
                }),
        )
        .add_startup_system(setup_system)
        .run();

    Ok(())
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // add rectangle
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
}

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
// ) {
//     // load the assets
//     let cornered_panel_texture_handle = asset_server.load("metalPanel_yellowCorner.png");

//     let panel_nine_patch_handle = nine_patches.add(NinePatchBuilder::from_patches(vec![
//         vec![
//             // top left corner patch
//             Patch {
//                 original_size: IVec2::new(30, 35),
//                 target_size: Size::new(Val::Undefined, Val::Undefined),
//                 content: None,
//             },
//             // top middle-left patch. This patch width can grow, and will contain the content for
//             // `PanelContent::Title`
//             Patch {
//                 original_size: IVec2::new(15, 35),
//                 target_size: Size::new(Val::Percent(30.), Val::Undefined),
//                 content: Some(Content::Title),
//             },
//             // top middle patch. In the original PNG, it's the yellow titled part
//             Patch {
//                 original_size: IVec2::new(25, 35),
//                 target_size: Size::new(Val::Undefined, Val::Undefined),
//                 content: None,
//             },
//             // top middle-right patch. This patch width can grow
//             Patch {
//                 original_size: IVec2::new(20, 35),
//                 target_size: Size::new(Val::Percent(70.), Val::Undefined),
//                 content: None,
//             },
//             // top right corner
//             Patch {
//                 original_size: IVec2::new(10, 35),
//                 target_size: Size::new(Val::Undefined, Val::Undefined),
//                 content: None,
//             },
//         ],
//         vec![
//             // left border. This patch height can grow
//             Patch {
//                 original_size: IVec2::new(10, -45),
//                 target_size: Size::new(Val::Undefined, Val::Percent(100.)),
//                 content: None,
//             },
//             // center. This patch can grow both in height and width, and will contain `PanelContent::Body`
//             Patch {
//                 original_size: IVec2::new(-20, -45),
//                 target_size: Size::new(Val::Percent(100.), Val::Percent(100.)),
//                 content: Some(Content::Content),
//             },
//             // right border. This patch height can grow
//             Patch {
//                 original_size: IVec2::new(10, -45),
//                 target_size: Size::new(Val::Undefined, Val::Percent(100.)),
//                 content: None,
//             },
//         ],
//         vec![
//             // bottom left corner
//             Patch {
//                 original_size: IVec2::new(10, 10),
//                 target_size: Size::new(Val::Undefined, Val::Undefined),
//                 content: None,
//             },
//             // bottom middle. This patch width can grow
//             Patch {
//                 original_size: IVec2::new(-20, 10),
//                 target_size: Size::new(Val::Percent(100.), Val::Undefined),
//                 content: None,
//             },
//             // bottom right corner
//             Patch {
//                 original_size: IVec2::new(10, 10),
//                 target_size: Size::new(Val::Undefined, Val::Undefined),
//                 content: None,
//             },
//         ],
//     ]));

//     commands.spawn((
//         // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
//         // of this entity
//         NinePatchBundle {
//             style: Style {
//                 margin: UiRect::all(Val::Auto),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 size: Size::new(Val::Px(900.), Val::Px(600.)),
//                 ..Default::default()
//             },
//             nine_patch_data: NinePatchData {
//                 nine_patch: panel_nine_patch_handle,
//                 texture: cornered_panel_texture_handle,
//                 ..Default::default()
//             },
//             ..Default::default()
//         },
//         UiElement::Panel,
//     ));

//     commands.spawn(Camera2dBundle::default());
// }

// fn set_content(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut nine_patches: ResMut<Assets<NinePatchBuilder<Content>>>,
//     mut patch_content: Query<(Entity, &mut NinePatchContent<Content>)>,
//     ui_element_query: Query<&UiElement>,
//     mut font: Local<Handle<Font>>,
// ) {
//     *font = asset_server.load("Kenney Future Narrow.ttf");

//     for (entity, mut nine_patch_content) in &mut patch_content.iter_mut() {
//         if !nine_patch_content.loaded {
//             match (
//                 *ui_element_query
//                     .get_component::<UiElement>(nine_patch_content.parent)
//                     .unwrap(),
//                 &nine_patch_content.content,
//             ) {
//                 (UiElement::Panel, Content::Content) => {
//                     let panel_texture_handle: Handle<Image> =
//                         asset_server.load("glassPanel_corners.png");

//                     // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
//                     let nine_patch_handle = nine_patches.add(
//                         NinePatchBuilder::by_margins_with_content(20, 20, 20, 20, Content::Content),
//                     );

//                     let content_entity = commands
//                         .spawn((
//                             // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
//                             // of this entity
//                             NinePatchBundle {
//                                 style: Style {
//                                     margin: UiRect::all(Val::Auto),
//                                     justify_content: JustifyContent::Center,
//                                     align_items: AlignItems::Center,
//                                     size: Size::new(Val::Px(850.), Val::Px(550.)),
//                                     ..Default::default()
//                                 },
//                                 nine_patch_data: NinePatchData {
//                                     nine_patch: nine_patch_handle,
//                                     texture: panel_texture_handle,
//                                     ..Default::default()
//                                 },
//                                 ..Default::default()
//                             },
//                             UiElement::InnerPanel,
//                         ))
//                         .id();
//                     commands.entity(entity).push_children(&[content_entity]);
//                     nine_patch_content.loaded = true;
//                 }
//                 (UiElement::Panel, Content::Title) => {
//                     let content_entity = commands
//                         .spawn(
//                             TextBundle::from_section(
//                                 "Example  Title",
//                                 TextStyle {
//                                     font: font.clone(),
//                                     font_size: 25.0,
//                                     color: Color::BLUE,
//                                 },
//                             )
//                             .with_style(Style {
//                                 margin: UiRect {
//                                     left: Val::Undefined,
//                                     right: Val::Auto,
//                                     top: Val::Px(8.),
//                                     bottom: Val::Auto,
//                                 },
//                                 ..Default::default()
//                             }),
//                         )
//                         .id();
//                     commands.entity(entity).push_children(&[content_entity]);
//                     nine_patch_content.loaded = true;
//                 }
//                 (UiElement::InnerPanel, _) => {
//                     // prepare the button
//                     let button_texture_handle = asset_server.load("blue_button02.png");
//                     let button_nine_patch_handle = nine_patches.add(
//                         NinePatchBuilder::by_margins_with_content(5, 10, 6, 6, Content::Content),
//                     );

//                     let button_cancel_entity = commands
//                         .spawn((
//                             // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
//                             // of this entity
//                             NinePatchBundle {
//                                 style: Style {
//                                     margin: UiRect {
//                                         left: Val::Px(0.),
//                                         right: Val::Auto,
//                                         top: Val::Auto,
//                                         bottom: Val::Px(0.),
//                                     },
//                                     size: Size::new(Val::Px(300.), Val::Px(80.)),

//                                     justify_content: JustifyContent::Center,
//                                     align_items: AlignItems::Center,
//                                     ..Default::default()
//                                 },
//                                 nine_patch_data: NinePatchData {
//                                     nine_patch: button_nine_patch_handle.clone(),
//                                     texture: button_texture_handle.clone(),
//                                     ..Default::default()
//                                 },
//                                 ..Default::default()
//                             },
//                             UiElement::ButtonCancel,
//                         ))
//                         .id();

//                     let button_ok_entity = commands
//                         .spawn((
//                             // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
//                             // of this entity
//                             NinePatchBundle {
//                                 style: Style {
//                                     margin: UiRect {
//                                         left: Val::Auto,
//                                         right: Val::Px(0.),
//                                         top: Val::Auto,
//                                         bottom: Val::Px(0.),
//                                     },
//                                     justify_content: JustifyContent::Center,
//                                     align_items: AlignItems::Center,
//                                     size: Size::new(Val::Px(300.), Val::Px(80.)),
//                                     ..Default::default()
//                                 },
//                                 nine_patch_data: NinePatchData {
//                                     nine_patch: button_nine_patch_handle,
//                                     texture: button_texture_handle,
//                                     ..Default::default()
//                                 },
//                                 ..Default::default()
//                             },
//                             UiElement::ButtonOK,
//                         ))
//                         .id();

//                     commands
//                         .entity(entity)
//                         .push_children(&[button_cancel_entity, button_ok_entity]);
//                     nine_patch_content.loaded = true;
//                 }
//                 (UiElement::ButtonOK, _) => {
//                     let content_entity = commands
//                         .spawn(
//                             TextBundle::from_section(
//                                 "OK",
//                                 TextStyle {
//                                     font: font.clone(),
//                                     font_size: 50.0,
//                                     color: Color::GREEN,
//                                 },
//                             )
//                             .with_style(Style {
//                                 margin: UiRect {
//                                     left: Val::Px(110.),
//                                     right: Val::Auto,
//                                     top: Val::Px(10.),
//                                     bottom: Val::Auto,
//                                 },
//                                 ..Default::default()
//                             }),
//                         )
//                         .id();
//                     commands.entity(entity).push_children(&[content_entity]);
//                     nine_patch_content.loaded = true;
//                 }
//                 (UiElement::ButtonCancel, _) => {
//                     let content_entity = commands
//                         .spawn(
//                             TextBundle::from_section(
//                                 "CANCEL",
//                                 TextStyle {
//                                     font: font.clone(),
//                                     font_size: 50.0,
//                                     color: Color::RED,
//                                 },
//                             )
//                             .with_style(Style {
//                                 margin: UiRect {
//                                     left: Val::Px(50.),
//                                     right: Val::Auto,
//                                     top: Val::Px(10.),
//                                     bottom: Val::Auto,
//                                 },
//                                 ..Default::default()
//                             }),
//                         )
//                         .id();
//                     commands.entity(entity).push_children(&[content_entity]);
//                     nine_patch_content.loaded = true;
//                 }
//             }
//         }
//     }
// }

// #[derive(Clone, PartialEq, Eq, std::hash::Hash)]
// enum Content {
//     Title,
//     Content,
// }

// #[derive(Clone, Copy, Component)]
// enum UiElement {
//     Panel,
//     InnerPanel,
//     ButtonOK,
//     ButtonCancel,
// }

// // by changing the component `Style.size`, the 9-Patch UI element will be resized
// fn update_size(time: Res<Time>, mut query: Query<(&mut Style, &UiElement)>) {
//     for (mut style, panel) in query.iter_mut() {
//         let (x, y) = time.elapsed_seconds().sin_cos();

//         match panel {
//             UiElement::Panel => {
//                 style.size.width = Val::Px((900. + 50. * x as f32).ceil());
//                 style.size.height = Val::Px((600. + 50. * y as f32).ceil());
//             }
//             UiElement::InnerPanel => {
//                 style.size.width = Val::Px((850. + 50. * x as f32).ceil());
//                 style.size.height = Val::Px((550. + 50. * y as f32).ceil());
//             }
//             UiElement::ButtonOK => style.size.width = Val::Px((300. + 50. * x as f32).ceil()),
//             UiElement::ButtonCancel => style.size.height = Val::Px((90. + 10. * y as f32).ceil()),
//         }
//     }
// }

// use bevy_prototype_lyon::entity::ShapeBundle;

// use wasm_bindgen::prelude::*;
// use cfg_if::cfg_if;
// use log;
// use std::rc::Rc;
// use std::ops::Deref;
// use std::collections::HashSet;
// use std::f32::consts::PI;
// use std::cell::RefCell;
// use web_sys::{Window, Document, History};
// use bevy::prelude::*;
// use bevy::ecs::prelude::*;
// use bevy::app::prelude::*;
// use bevy::asset::{AssetPlugin};
// use bevy::input::{InputPlugin};
// use bevy::scene::{ScenePlugin};
// use bevy::render::{RenderPlugin};
// use bevy::sprite::{SpritePlugin};
// use bevy::text::{TextPlugin};
// use bevy::gltf::{GltfPlugin};
// use bevy::render::camera::Camera;
// use bevy::render::color::Color;
// use bevy::input::keyboard::KeyboardInput;
// use bevy_prototype_lyon::prelude::*;
// use bevy_prototype_lyon::prelude::DrawMode::Fill;

// const LOBBY_WIDTH: f32 = 800.0;
// const LOBBY_HEIGHT: f32 = 800.0;

// const PLAYER_SPEED: f32 = 300.0;
// const PLAYER_RADIUS: f32 = 32.0;

// const WALL_THICKNESS: f32 = 32.0;
// const WALL_COLOR: Color = Color::NAVY;
// const WALLS_THICKNESS: f32 = 2.0;

// const GREEN_REGION: (f32, f32, f32, f32) = (256.0, 256.0, 192.0, 192.0);
// const GREEN_REGION_COLOR: Color = Color::GREEN;
// const RED_REGION: (f32, f32, f32, f32) = (352.0, 352.0, 96.0, 96.0);
// const RED_REGION_COLOR: Color = Color::RED;

// #[derive(Resource, Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// struct Player {
//     pub position: Vec2,
// }

// #[derive(Resource, Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// struct Walls {
//     pub top: f32,
//     pub left: f32,
//     pub bottom: f32,
//     pub right: f32,
// }

// #[derive(Default)]
// struct InputState {
//     left: bool,
//     right: bool,
//     up: bool,
//     down: bool,
// }

// #[derive(Resource)]
// struct CommandsQueue {
//     queue: HashSet<KeyCode>,
// }

// impl Default for CommandsQueue {
//     fn default() -> Self { CommandsQueue { queue: HashSet::new() } }
// }

// #[derive(Resource, Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// struct Game {
//     pub player: Player,
//     pub walls: Walls,
// }

// fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
//     let wall_thickness = WALL_THICKNESS;
//     let walls_thickness = WALLS_THICKNESS;
//     let wall_color = StrokeMode::new(WALL_COLOR, wall_thickness);
//     let lobby_width = LOBBY_WIDTH;
//     let lobby_height = LOBBY_HEIGHT;

//     commands.spawn(Camera2dBundle::default());

//     let walls = Walls {
//         top: lobby_height / 2.0 - wall_thickness / 2.0,
//         left: -lobby_width / 2.0 + wall_thickness / 2.0,
//         bottom: -lobby_height / 2.0 + wall_thickness / 2.0,
//         right: lobby_width / 2.0 - wall_thickness  / 2.0,
//     };

//     let wall_thickness_2 = wall_thickness / 2.0 + walls_thickness / 2.0;
//     let left_wall = shapes::Rectangle {
//         extents: Vec2::new(walls_thickness, lobby_height + wall_thickness_2 * 2.0),
//         origin: RectangleOrigin::CustomCenter(Vec2::new(-lobby_width / 2.0 - walls_thickness / 2.0, 0.0)),
//     };

//     let right_wall = shapes::Rectangle {
//         extents: Vec2::new(walls_thickness, lobby_height + wall_thickness_2 * 2.0),
//         origin: RectangleOrigin::CustomCenter(Vec2::new(lobby_width / 2.0 - walls_thickness / 2.0, 0.0)),
//     };

//     let top_wall = shapes::Rectangle {
//         extents: Vec2::new(lobby_width + wall_thickness_2 * 2.0, walls_thickness),
//         origin: RectangleOrigin::CustomCenter(Vec2::new(0.0, lobby_height / 2.0 - walls_thickness / 2.0)),
//     };

//     let bottom_wall = shapes::Rectangle {
//         extents: Vec2::new(lobby_width + wall_thickness_2 * 2.0, walls_thickness),
//         origin: RectangleOrigin::CustomCenter(Vec2::new(0.0, -lobby_height / 2.0 + walls_thickness / 2.0)),
//     };

//     let wall_fill_mode = FillMode::color(WALL_COLOR);
//     let wall_outline_mode = StrokeMode::new(WALL_COLOR, 2.0);
//     let wall_draw_mode = DrawMode::Outlined { fill_mode: wall_fill_mode, outline_mode: wall_outline_mode };

//     let mut wall_commands = commands.spawn(
//         GeometryBuilder::build_as(&left_wall, wall_draw_mode, Transform::default()));
//     wall_commands.insert(GeometryBuilder::build_as(&right_wall, wall_draw_mode, Transform::default()));
//     wall_commands.insert(GeometryBuilder::build_as(&top_wall, wall_draw_mode, Transform::default()));
//     wall_commands.insert(GeometryBuilder::build_as(&bottom_wall, wall_draw_mode, Transform::default()));

//     let green_region_size = Vec2::new(GREEN_REGION.2, GREEN_REGION.3);
//     let green_region_translation = Vec3::new(
//         GREEN_REGION.0 - lobby_width / 2.0 + GREEN_REGION.2 / 2.0,
//         GREEN_REGION.1 - lobby_height / 2.0 + GREEN_REGION.3 / 2.0,
//         0.0,
//     );

//     let green_region_entity = commands.spawn(GeometryBuilder::build_as(
//         &shapes::Rectangle {
//             extents: green_region_size,
//             origin: RectangleOrigin::TopLeft,
//         },
//         DrawMode::Fill(FillMode::color(GREEN_REGION_COLOR)),
//         Transform::from_translation(green_region_translation)
//     ))
//     .id();

//     let red_region_size = Vec2::new(RED_REGION.2, RED_REGION.3);
//     let red_region_translation = Vec3::new(
//         RED_REGION.0 - lobby_width / 2.0 + RED_REGION.2 / 2.0,
//         RED_REGION.1 - lobby_height / 2.0 + RED_REGION.3 / 2.0,
//         0.0,
//     );

//     let red_region_entity = commands.spawn(GeometryBuilder::build_as(
//         &shapes::Rectangle {
//             extents: red_region_size,
//             origin: RectangleOrigin::TopLeft,
//         },
//         DrawMode::Fill(FillMode::color(RED_REGION_COLOR)),
//         Transform::from_translation(red_region_translation)
//     ))
//     .id();
// }

// thread_local! {
//     static WINDOW: Rc<Window> = Rc::new(web_sys::window().unwrap_throw());
//     static DOCUMENT: Document = WINDOW.with(|w| w.document().unwrap_throw());
//     static HISTORY: History = WINDOW.with(|w| w.history().unwrap_throw());
// }

// #[wasm_bindgen(start)]
// pub async fn main_js() -> Result<(), JsValue> {
//   setup_logger();

//   log::info!("creating bevy world");
//   log::info!("starting run loop on window {:?}", WINDOW);

//   App::new()
//     .add_plugin(CorePlugin::default())
//     .add_plugin(TransformPlugin::default())
//     .add_plugin(InputPlugin::default())
//     .add_plugin(WindowPlugin::default())
//     .add_plugin(AssetPlugin::default())
//     .add_plugin(ScenePlugin::default())
//     .add_plugin(RenderPlugin::default())
//     .add_plugin(SpritePlugin::default())
//     .add_plugin(TextPlugin::default())
//     .add_plugin(GltfPlugin::default())
//     .add_startup_system(setup)
//     // .add_system(update_input.system())
//     // .add_system(update_player_position.system())
//     // .add_system(detect_region_entry_exit.system())
//     // .add_system(render.system())
//     .run();
//   //   let context = Context::new(WINDOW.clone());
//   //   let ui_manager = GameUiManager::new(&context);
//   //   let app_shared = Shared::new(App {
//   //     context: context,
//   //     ui_manager: ui_manager,
//   //     fps_tracker: FpsTracker::new(),
//   //   });

//   //   register_on_error_listener(&window);
//   //   register_on_visibility_change_listener(&window, app_shared.clone());
//   //   register_input_listeners(&window, app_shared.clone());

//   //   console::log_1(&JsValue::from_str("Starting run_loop(app_shared)!"));
//   //   run_loop(app_shared);
// //   log::info!("starting run loop");
//   Ok(())
// }

// cfg_if! {
//     if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
//         fn setup_logger() {
//             wasm_logger::init(wasm_logger::Config::default());
//             console_error_panic_hook::set_once();
//             std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//             log::info!("rust logging enabled!!!");
//             console_error_panic_hook::set_once();
//         }
//     } else {
//         fn setup_logger() {
//             log::info!("rust logging disabled!"); //<-- won't be seen
//         }
//     }
// }

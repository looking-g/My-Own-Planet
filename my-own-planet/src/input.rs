//! Systems for user input

use crate::{get_surface_point, DisplaceEdit, FormMode, MeshEdits, RandomRes, redo_planet_mesh, Planet, PlanetRes};

use bevy::{
    prelude::*,
    picking::hover::Hovered,
    ui_widgets::{
        observe, slider_self_update, Slider, SliderRange, SliderThumb,
        SliderValue, TrackClick, SliderPlugin,
    },
};


/// The plugin for all input related systems
pub fn input_plugin(app: &mut App) {
    app
        .add_plugins(SliderPlugin)
        .insert_resource(NumberOfCraters(0))
        .insert_resource(NumberOfMountains(0))
        .insert_resource(OldMousePos::default())
        .insert_resource(Buttons::default())
        .insert_resource(Sliders::default())
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (rotate_planet, react_to_buttons, update_text, update_slider_visuals))
        .add_observer(crater_generater)
        .add_observer(crater_remover)
        .add_observer(mountain_generater)
        .add_observer(mountain_remover)
        .add_observer(update_planet_visual)
    ;
}

#[derive(Event)]
struct AddCrater;
#[derive(Event)]
struct SubCrater;

#[derive(Resource)]
struct NumberOfCraters(u32);
#[derive(Component)]
struct NumCraterDisplay;

/// Adds a new crator to a random spot
fn crater_generater(
    _create: On<AddCrater>,
    mut planet: Single<&mut MeshEdits, With<Planet>>, 
    mut rand: ResMut<RandomRes>,
    planet_info: Res<PlanetRes>,
    mut count: ResMut<NumberOfCraters>,
    mut commands: Commands,
) {
    let crater_radius = 0.3;
    let ref mut edits = *planet;

    edits.0.push(
        DisplaceEdit::Circle{
            pos: get_surface_point(planet_info.size, &mut rand),
            r: crater_radius,
            mode: FormMode::Sub,
        }
    );
    count.0 += 1;

    commands.trigger(UpdateVisual)
}

fn crater_remover(
    _remover: On<SubCrater>,
    mut count: ResMut<NumberOfCraters>,
    mut planet: Single<&mut MeshEdits, With<Planet>>, 
    mut commands: Commands,
) {
    if count.0 > 0 {
        let ref mut edits = *planet;
        for (i, edit) in edits.0.iter().enumerate().rev(){
            if matches!(edit, DisplaceEdit::Circle{..}){
                (*edits).0.remove(i);
                count.0 -= 1;
                commands.trigger(UpdateVisual);
                return ();
            }
        }
    }
}

#[derive(Event)]
struct AddMountain;
#[derive(Event)]
struct SubMountain;

#[derive(Resource)]
struct NumberOfMountains(u32);
#[derive(Component)]
struct NumMountainDisplay;


/// Adds a new mountain to a random spot
fn mountain_generater(
    _create: On<AddMountain>,
    mut planet: Single<&mut MeshEdits, With<Planet>>, 
    mut rand: ResMut<RandomRes>,
    planet_info: Res<PlanetRes>,
    mut count: ResMut<NumberOfMountains>,
    mut commands: Commands,
) {
    let crater_radius = 0.6;
    let ref mut edits = *planet;

    edits.0.push(
        DisplaceEdit::HalfCircle{
            pos: get_surface_point(planet_info.size, &mut rand),
            r: crater_radius,
            mode: FormMode::Add,
        }
    );
    count.0 += 1;

    commands.trigger(UpdateVisual)
}

fn mountain_remover(
    _remover: On<SubMountain>,
    mut count: ResMut<NumberOfMountains>,
    mut planet: Single<&mut MeshEdits, With<Planet>>, 
    mut commands: Commands,
) {
    if count.0 > 0 {
        let ref mut edits = *planet;
        for (i, edit) in edits.0.iter().enumerate().rev(){
            if matches!(edit, DisplaceEdit::HalfCircle{..}){
                (*edits).0.remove(i);
                count.0 -= 1;
                commands.trigger(UpdateVisual);
                return ();
            }
        }
    }
}


#[derive(Event)]
struct UpdateVisual;

fn update_planet_visual(
    _activate: On<UpdateVisual>,
    mut planet: Single<(&mut MeshEdits, &mut Mesh3d), With<Planet>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    planet_info: Res<PlanetRes>,
) {
    let (ref mut edits, ref mut planet_mesh) = *planet;
    redo_planet_mesh(
        planet_mesh,
        edits,
        &mut meshes,
        planet_info,
    );
}
    
const PLANET_ROT_SPEED: f32 = 0.001;

use std::f32::consts::TAU; 
use bevy::window::PrimaryWindow;

const ALL_MOUSE_BUTTONS: [MouseButton; 3] = [MouseButton::Left, MouseButton::Middle, MouseButton::Right];

/// Allows the user to rotate the planet useing the mouse
fn rotate_planet(
    mut planet: Single<&mut Transform, With<Planet>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut old_pos: ResMut<OldMousePos>,
) {
    let ref mut transform = *planet;

    let current_pos = window.cursor_position();

    if mouse_buttons.any_pressed(ALL_MOUSE_BUTTONS) { 
        if let Some(some_pos) = current_pos{
            if let Some(some_old_pos) = old_pos.0 {
                let delta_pos = some_pos - some_old_pos;
     
                transform.rotate_y(delta_pos.x * TAU * PLANET_ROT_SPEED);
                transform.rotate_x(delta_pos.y * TAU * PLANET_ROT_SPEED);
            }
        }
    }

    old_pos.0 = current_pos;

}

#[derive(Resource, Default)]
struct OldMousePos(Option<Vec2>);


// UI

const BUTTON_NORMAL: Color = Color::srgb(0.3, 0.3, 0.3);
const BUTTON_HOVER: Color = Color::srgb(0.4, 0.4, 0.4);
const BUTTON_PRESS: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Resource)]
struct Buttons{
    add_crater: Entity,
    sub_crater: Entity,
    add_mountains: Entity,
    sub_mountains: Entity,
}

impl Default for Buttons {
    fn default() -> Self {
        Self{
            add_crater: Entity::PLACEHOLDER,
            sub_crater: Entity::PLACEHOLDER,
            add_mountains: Entity::PLACEHOLDER, 
            sub_mountains: Entity::PLACEHOLDER, 
        }
    }
}

#[derive(Resource)]
struct Sliders{
    test: Entity,
}

impl Default for Sliders {
    fn default() -> Self {
        Self{
            test: Entity::PLACEHOLDER,
        }
    }
}

// for all untracked text elements

#[derive(Component)]
struct NoTrack;

// based of the code from
// https://bevy.org/examples/ui-user-interface/anchor-layout/
fn make_text_ui(text: &str, font: Handle<Font>, node: Node, tracker: impl Component) -> impl Bundle{
    (
        tracker,
        node,
        Text::new(text),
        TextFont { 
            font,
            font_size: 20.0, 
            ..default() 
        },
        TextColor::from(Color::srgb(0.5, 0.5, 0.5)),
    )
}


fn make_button_ui(text: &str, font: Handle<Font>, mut node: Node) -> impl Bundle{

    node.justify_content = JustifyContent::Center;
    node.align_items = AlignItems::Center;
    (
        node,
        TextColor::BLACK,
        BackgroundColor(BUTTON_NORMAL.into()),
        Button,
        children![(
            Text::new(text),
            TextFont { 
                font,
                font_size: 20.0, 
                ..default() 
            },
        )]
    )
}

/// describes what should happen when a button is pressed
fn react_to_buttons(
    mut interactions: Query<(Entity, &Interaction, &mut BackgroundColor), With<Button>>,
    buttons: Res<Buttons>,
    mut commands: Commands,

    mouse_buttons: Res<ButtonInput<MouseButton>>,
) {
    // if the mouse was just pressed down
    let just_down = mouse_buttons.any_just_pressed(ALL_MOUSE_BUTTONS);
    for (entity, interaction, mut back_color) in interactions.iter_mut() {

        match *interaction{
            Interaction::Pressed => {
                *back_color = BUTTON_PRESS.into();
            }
            Interaction::Hovered => {
                *back_color = BUTTON_HOVER.into();
            }
            Interaction::None => {
                *back_color = BUTTON_NORMAL.into();
            }
        }
        
        match entity{
            // basic requierments for a button to do somthing
            entity if *interaction == Interaction::Pressed && just_down => {
                match entity{
                    x if buttons.add_crater == x => commands.trigger(AddCrater),
                    x if buttons.sub_crater == x => commands.trigger(SubCrater),
                    x if buttons.add_mountains == x => commands.trigger(AddMountain),
                    x if buttons.sub_mountains == x => commands.trigger(SubMountain),
                    _ => continue,
                }
            }
            _ => continue,
        };
    }
}


fn update_text(
    mut texts: Query<(&mut Text, Has<NumCraterDisplay>, Has<NumMountainDisplay>)>,
    num_crater: Res<NumberOfCraters>,
    num_mountain: Res<NumberOfMountains>,
) {
    for (mut text, crater, mountain) in texts.iter_mut() {
        if crater { text.0 = format!("{}", num_crater.0); }
        if mountain { text.0 = format!("{}", num_mountain.0); }
    }
}


fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    buttons: ResMut<Buttons>,
    sliders: ResMut<Sliders>,
) {


    // font from 
    // https://www.jetbrains.com/lp/mono/
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    let ui_aria = commands
        .spawn((Node {
            width: percent(46),
            height: percent(96),
            justify_self: JustifySelf::End,
            justify_content: JustifyContent::Start,
            margin: UiRect {
                right: percent(1),
                top: percent(1),
                ..default()
            },
            border_radius: BorderRadius::all(px(20)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.35).into()),
        // text
        children![
            // craters
            make_text_ui("Craters:", font.clone(),
                Node{
                    position_type: PositionType::Absolute,
                    left: percent(4),
                    top: percent(3),
                    ..default() 
                },
            NoTrack),
            make_text_ui("999", font.clone(),
                Node{
                    position_type: PositionType::Absolute,
                    left: percent(4),
                    top: percent(7),
                    ..default() 
                },
            NumCraterDisplay),
            // mountains
            make_text_ui("Mountains:", font.clone(),
                Node{
                    position_type: PositionType::Absolute,
                    left: percent(4),
                    top: percent(14),
                    ..default() 
                },
            NoTrack),
            make_text_ui("999", font.clone(),
                Node{
                    position_type: PositionType::Absolute,
                    left: percent(4),
                    top: percent(18),
                    ..default() 
                },
            NumMountainDisplay),

        ]
    // buttons
    )).id();

    let buttons = buttons.into_inner();

    spwn_button_system(&mut buttons.add_crater, &mut buttons.sub_crater, percent(3), font.clone(), ui_aria, &mut commands);
    spwn_button_system(&mut buttons.add_mountains, &mut buttons.sub_mountains, percent(14), font.clone(), ui_aria, &mut commands);

    let sliders = sliders.into_inner();

    make_slider_ui(
        Node{
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            position_type: PositionType::Absolute,
            width: percent(25),
            height: px(12),
            left: percent(4),
            top: percent(18),
            ..default() 
        },
        &mut sliders.test,
        &mut commands,
        (0.0, 100.0),
        25.0, 
    );
}


fn spwn_button_system(
    add_tracker: &mut Entity, 
    sub_tracker: &mut Entity, 
    val_from_top: Val, 
    font: Handle<Font>,
    parent: Entity,
    commands: &mut Commands,
) {
    *add_tracker = commands.spawn((
        make_button_ui("+1", font.clone(),
            Node{
                position_type: PositionType::Absolute,
                left: percent(90),
                top: val_from_top,
                width: percent(8),
                height: percent(5),
                border_radius: BorderRadius::all(px(10)),
                ..default() 
            }
        ),
        ChildOf(parent),
    )).id();


    *sub_tracker = commands.spawn((
        make_button_ui("-1", font.clone(),
            Node{
                position_type: PositionType::Absolute,
                left: percent(50),
                top: val_from_top,
                width: percent(8),
                height: percent(5),
                border_radius: BorderRadius::all(px(10)),
                ..default() 
            }
        ),
        ChildOf(parent),
    )).id();

}

#[derive(Component)]
struct ThumbParent;

// based of the code from
// https://bevy.org/examples/ui-user-interface/vertical-slider/
fn make_slider_ui(
    node: Node, // used for defining pos and size
    tracker: &mut Entity, // this value is set to the slider's id
    commands: &mut Commands,
    range: (f32, f32), // start, end
    init_val: f32, // starting value
) {
    // parent node
    *tracker = commands.spawn((
        node,
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
        },
        SliderValue(init_val),
        SliderRange::new(range.0, range.1),
        observe(slider_self_update),
    )).id();
    // black bar
    commands.spawn((
        ChildOf(*tracker),
        Node {
            height: px(6),
            border_radius: BorderRadius::all(px(3)),
            ..default()
        },
        BackgroundColor(BUTTON_HOVER),
    ));
    // sliding thumb aka. nub
    commands.spawn((
        ChildOf(*tracker),
        ThumbParent,
        Node { // pos node
            display: Display::Flex,
            position_type: PositionType::Absolute,
            left: px(0),
            right: px(0),
            top: px(0),
            bottom: px(0),
            ..default()
        },
        children![(
            SliderThumb,
            Node { // looks node
                display: Display::Flex,
                width: px(12),
                height: px(12),
                right: px(6),
                position_type: PositionType::Relative,
                border_radius: BorderRadius::MAX,
                ..default()
            },
            BackgroundColor(BUTTON_PRESS),
        )],
    ));
}

fn update_slider_visuals(
    sliders: Query<(&Children, &SliderValue, &SliderRange), Changed<SliderValue>>,
    mut slider_thumbs: Query<&mut Node, With<ThumbParent>>,
) {
    for (children, slider_value, slider_range) in sliders.iter() {
        for child in children.iter() {
            if let Ok(mut child_node) = slider_thumbs.get_mut(child) {
                child_node.left = percent((slider_value.0 / slider_range.end()) * 100.0)
            }
        }
    }
}






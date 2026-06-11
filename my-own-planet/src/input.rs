//! Systems for user input

use crate::{get_surface_point, DisplaceEdit, FormMode, MeshEdits, RandomRes, redo_planet_mesh, Planet, PlanetRes};

use bevy::prelude::*;


/// The plugin for all input related systems
pub fn input_plugin(app: &mut App) {
    app
        .insert_resource(NumberOfCraters(0))
        .insert_resource(OldMousePos::default())
        .insert_resource(Buttons::default())
        .add_systems(Startup, setup_ui)
        .add_systems(Update, (rotate_planet, react_to_buttons, update_text))
        .add_observer(crater_generater)
        .add_observer(crater_remover)
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
    mut crater_count: ResMut<NumberOfCraters>,
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
    crater_count.0 += 1;

    commands.trigger(UpdateVisual)
}

fn crater_remover(
    _remover: On<SubCrater>,
    mut num_crater: ResMut<NumberOfCraters>,
    mut planet: Single<&mut MeshEdits, With<Planet>>, 
    mut commands: Commands,
) {
    if num_crater.0 > 0 {
        let ref mut edits = *planet;
        for (i, edit) in edits.0.iter().enumerate().rev(){
            if matches!(edit, DisplaceEdit::Circle{..}){
                (*edits).0.remove(i);
                num_crater.0 -= 1;
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


// UI stuff

const BUTTON_NORMAL: Color = Color::srgb(0.3, 0.3, 0.3);
const BUTTON_HOVER: Color = Color::srgb(0.4, 0.4, 0.4);
const BUTTON_PRESS: Color = Color::srgb(0.6, 0.6, 0.6);

#[derive(Resource)]
struct Buttons{
    add_crater: Entity,
    sub_crater: Entity,
}

impl Default for Buttons {
    fn default() -> Self {
        Self{
            add_crater: Entity::PLACEHOLDER,
            sub_crater: Entity::PLACEHOLDER,
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut buttons: ResMut<Buttons>,
) {


    // font from 
    // https://www.jetbrains.com/lp/mono/
    let font = asset_server.load("fonts/JetBrainsMono-Regular.ttf");

    commands
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
        // static ui parts
        children![
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
                    top: percent(6),
                    ..default() 
                },
            NumCraterDisplay),
        ]
    // buttons
    )).with_children(|parent| {
    {
        buttons.add_crater = parent.spawn(
            make_button_ui("+1", font.clone(),
                Node{
                    position_type: PositionType::Absolute,
                    left: percent(66),
                    top: percent(3),
                    width: percent(8),
                    height: percent(5),
                    border_radius: BorderRadius::all(px(10)),
                    ..default() 
                }
            )

        ).id();
        buttons.sub_crater = parent.spawn(
            make_button_ui("-1", font.clone(),
                Node{
                    position_type: PositionType::Absolute,
                    left: percent(33),
                    top: percent(3),
                    width: percent(8),
                    height: percent(5),
                    border_radius: BorderRadius::all(px(10)),
                    ..default() 
                }
            )

        ).id();
    }
    });

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
                    _ => continue,
                }
            }
            _ => continue,
        };
    }
}


fn update_text(
    mut crater_text: Single<&mut Text, With<NumCraterDisplay>>,
    num_crater: Res<NumberOfCraters>,
) {
    crater_text.0 = format!("{}", num_crater.0);
}








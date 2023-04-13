use std::fmt::Debug;
use std::time::Duration;

use bevy::app::{App, ScheduleRunnerSettings};
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::new(5, 0)));
    app.add_plugins(MinimalPlugins);
    app.init_resource::<Events<Action>>();
    app.add_event::<Action>();
    app.add_startup_system(startup_system);
    app.add_systems((handle_parser_events, greet_players).chain());
    app.add_systems((
        inspect_changes_system::<SeatNum>.in_base_set(CoreSet::PostUpdate),
        inspect_changes_system::<Name>.in_base_set(CoreSet::PostUpdate),
        greet_players.in_base_set(CoreSet::PostUpdate),
    ));
    // )
    // .run();

    let action = Action::SeatUpdated(SeatUpdatedParams {
        name: "adevlupec".into(),
        seat_num: 1,
        npc: false,
    });
    let mut event_source = app.world.get_resource_mut::<Events<Action>>().unwrap();
    event_source.send(action);

    app.update();
    println!("Between update");

    let actions_vec = vec![
        Action::SeatUpdated(SeatUpdatedParams {
            name: "Dette32".into(),
            seat_num: 2,
            npc: false,
        }),
        Action::SeatUpdated(SeatUpdatedParams {
            name: "Drug08".into(),
            seat_num: 3,
            npc: false,
        }),
        Action::SeatUpdated(SeatUpdatedParams {
            name: "FluffyStutt".into(),
            seat_num: 4,
            npc: false,
        }),
    ];
    let mut event_source = app.world.get_resource_mut::<Events<Action>>().unwrap();
    actions_vec.into_iter().for_each(|action| event_source.send(action));

    app.update();
}

// -- Action --
#[derive(Debug)]
enum Action {
    SeatUpdated(SeatUpdatedParams),
}

#[derive(Default, Debug)]
struct SeatUpdatedParams {
    name: String,
    seat_num: u8,
    npc: bool,
}
// -- Action end --

// -- Player --
#[derive(Component, Default, Debug)]
struct Name(String);

#[derive(Component, Default, Debug)]
struct SeatNum(u8);

#[derive(Component)]
struct Npc;

#[derive(Component, Default)]
struct Stack(u64);
// -- Player end --

fn startup_system() {
    println!("Startup system, init some config");
}

fn handle_parser_events(
    mut commands: Commands,
    event_source: Res<Events<Action>>,
    mut event_reader: Local<Option<ManualEventReader<Action>>>,
) {
    event_reader
        .get_or_insert(event_source.get_reader())
        .iter(&event_source)
        .for_each(|event| {
            match event {
                Action::SeatUpdated(seat_params) => {
                    println!("Action::SeatUpdated from event source");

                    if seat_params.npc {
                        commands.spawn((
                            Name(seat_params.name.clone()),
                            SeatNum(seat_params.seat_num),
                            Npc,
                        ))
                    } else {
                        commands.spawn((
                            Name(seat_params.name.clone()),
                            SeatNum(seat_params.seat_num),
                        ))
                    }
                }
            };
        });

    println!("Events were handled");
}

fn greet_players(query: Query<&Name, With<SeatNum>>) {
    println!("greet");
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn inspect_changes_system<T: Component + Debug>(q: Query<Ref<T>>) {
    // Iterate over each component of type `T` and log its changed status.
    for val in &q {
        if val.is_changed() {
            println!(
                "Value `{val:?}` was last changed at tick {}.",
                val.last_changed()
            );
        } else {
            println!("Value `{val:?}` is unchanged.");
        }
    }
}

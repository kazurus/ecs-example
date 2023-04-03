use std::time::Duration;

use bevy::app::{App, ScheduleRunnerSettings};
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::new(5, 0)))
        .add_plugins(MinimalPlugins)
        .init_resource::<ParserEventSource>()
        .add_event::<Action>()
        .add_startup_system(startup_system)
        .add_systems(
            (
                handle_parser_events,
                greet_players,
            )
                .chain(),
        )
        .run();
}

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

#[derive(Component, Default)]
struct Name(String);

#[derive(Component, Default)]
struct SeatNum(u8);

#[derive(Component)]
struct Npc;

#[derive(Component, Default)]
struct Stack(u64);

fn startup_system(mut event_source: ResMut<ParserEventSource>) {
    println!("Startup system");
    let action = Action::SeatUpdated(SeatUpdatedParams {
        name: "Name 1".into(),
        seat_num: 1,
        npc: true,
    });
    event_source.events.push(action);
}

#[derive(Resource, Default)]
struct ParserEventSource {
    events: Vec<Action>,
}

fn handle_parser_events(mut commands: Commands, mut event_source: ResMut<ParserEventSource>) {
    event_source.events.iter().for_each(|event| {
        match event {
            Action::SeatUpdated(seat_params) => {
                println!("Action::SeatUpdated");

                if seat_params.npc == true {
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
    event_source.events.clear();
}

fn greet_players(query: Query<&Name, With<SeatNum>>) {
    println!("greet");
    for name in &query {
        println!("hello {}!", name.0);
    }
}

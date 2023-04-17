use std::collections::HashMap;
use std::fmt::Debug;

use bevy::app::App;
use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::{ComponentId, Components};
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Events<Action>>()
        // .add_event::<Action>()
        .add_startup_system(startup_system)
        .add_system(handle_parser_events)
        .add_systems(
            (
                show_all_players_system.in_base_set(CoreSet::PostUpdateFlush),
                show_game_system.in_base_set(CoreSet::PostUpdateFlush),
                show_board_system.in_base_set(CoreSet::PostUpdateFlush),
            )
                .chain(),
        )
        .add_systems((
            // inspect_changes_system::<PlayerSeatNum>.in_base_set(CoreSet::PostUpdate),
        ));

    let actions_vec = vec![
        Action::SeatUpdated(SeatUpdatedParams {
            name: "adevlupec".into(),
            seat_num: 1,
            npc: false,
        }),
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
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![
        Action::GameHandIdSet("174088855475".to_string()),
        Action::GameTypeSet(GameType::NL),
        Action::GameLimitSet(GameLimit::L100),
    ];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::GameMaxSeatsSet(6), Action::GameDealerSeatNumSet(3)];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::StackUpdated(StackUpdatedParams {
        name: "adevlupec".into(),
        stack: 53368,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::StackUpdated(StackUpdatedParams {
        name: "Dette32".into(),
        stack: 10845,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::StackUpdated(StackUpdatedParams {
        name: "Drug08".into(),
        stack: 9686,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::StackUpdated(StackUpdatedParams {
        name: "FluffyStutt".into(),
        stack: 11326,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::NpcCardsDealt(NpcCardsDealtParams {
        name: "FluffyStutt".into(),
        cards: vec![Card::H2, Card::SK],
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::CommunityCardsDealt(CommunityCardsDealtParams {
        prev_cards: vec![],
        // new_cards: vec![Card::H8, Card::S7, Card::D8],
        new_cards: vec![Card::H2, Card::S7, Card::D8],
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);
}

fn apply_batch_actions_to_app(app: &mut App, actions: Vec<Action>) {
    let mut event_source = app.world.get_resource_mut::<Events<Action>>().unwrap();
    actions
        .into_iter()
        .for_each(|action| event_source.send(action));
    app.update();
}

// -- Action --
#[derive(Debug, Clone)]
enum Action {
    SeatUpdated(SeatUpdatedParams),
    StackUpdated(StackUpdatedParams),
    // NewGameLaunched,
    GameHandIdSet(String),
    GameTypeSet(GameType),
    GameLimitSet(GameLimit),
    GameMaxSeatsSet(u8),
    GameDealerSeatNumSet(u8),
    CommunityCardsDealt(CommunityCardsDealtParams),
    NpcCardsDealt(NpcCardsDealtParams),
}

#[derive(Default, Debug, Clone)]
struct SeatUpdatedParams {
    name: String,
    seat_num: u8,
    npc: bool,
}

#[derive(Default, Debug, Clone)]
struct StackUpdatedParams {
    name: String,
    stack: u64,
}

#[derive(Default, Debug, Clone)]
struct CommunityCardsDealtParams {
    prev_cards: Vec<Card>,
    new_cards: Vec<Card>,
}

#[derive(Default, Debug, Clone)]
struct NpcCardsDealtParams {
    name: String,
    cards: Vec<Card>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Card {
    H2,
    H8,
    HT,
    S7,
    SQ,
    SK,
    ST,
    D8,
    C2,
}
// -- Action end --

// -- Game --
#[derive(Component, Default, Debug)]
struct Game;

#[derive(Component, Default, Debug)]
struct GameHandId(String);

#[derive(Component, Debug, Clone, Copy)]
enum GameType {
    NL,
}

#[derive(Component, Debug, Clone, Copy)]
enum GameLimit {
    L100,
}

#[derive(Component, Debug)]
struct GameMaxSeats(u8);

#[derive(Component, Default, Debug, Clone, Copy)]
struct DealerSeatNum(u8);
// -- Game end --

// -- Board --
#[derive(Component, Default, Debug)]
struct Board;

#[derive(Component, Default, Debug)]
struct BoardCards(Vec<Card>);
// -- Board end --

// -- Player --
#[derive(Component, Default, Debug)]
struct Player;

#[derive(Component, Default, Debug)]
struct PlayerName(String);

#[derive(Component, Default, Debug)]
struct PlayerSeatNum(u8);

#[derive(Component, Debug)]
struct PlayerNpc;

#[derive(Component, Default, Debug)]
struct PlayerStack(u64);

#[derive(Component, Default, Debug)]
struct PlayerCards(Vec<Card>);

#[derive(Component, Default, Debug)]
struct Dealer;
// -- Player end --

fn startup_system() {
    println!("Startup system, init some config");
}

fn handle_parser_events(
    mut commands: Commands,
    event_source: Res<Events<Action>>,
    players_entities: Query<(Entity, &PlayerSeatNum)>,
    mut event_reader: Local<Option<ManualEventReader<Action>>>,
    mut game_entity: Local<Option<Entity>>,
    mut board_entity: Local<Option<Entity>>,
    mut players_hmap: Local<Option<HashMap<String, Entity>>>,
) {
    let game_entity = game_entity
        .get_or_insert_with(|| commands.spawn(Game).id())
        .to_owned();
    let board_entity = board_entity
        .get_or_insert_with(|| commands.spawn(Board).id())
        .to_owned();
    let players_hmap = players_hmap.get_or_insert_with(Default::default);

    event_reader
        .get_or_insert_with(|| event_source.get_reader())
        .iter(&event_source)
        .for_each(|event| {
            match event {
                // Action::NewGameLaunched => unimplemented!(),
                Action::GameHandIdSet(hand_id) => {
                    println!("Action::GameHandIdSet from event source");
                    commands
                        .entity(game_entity)
                        .insert(GameHandId(hand_id.clone()));
                }
                Action::GameMaxSeatsSet(max_seats) => {
                    println!("Action::GameMaxSeatsSet from event source");
                    commands
                        .entity(game_entity)
                        .insert(GameMaxSeats(*max_seats));
                }
                Action::GameDealerSeatNumSet(dealer_seat_num) => {
                    println!("Action::GameDealerSeatNumSet from event source");

                    commands
                        .entity(game_entity)
                        .insert(DealerSeatNum(*dealer_seat_num));

                    let player_entity = *players_entities
                        .into_iter()
                        .filter(|(_, &PlayerSeatNum(seat_num))| seat_num == *dealer_seat_num)
                        .map(|(e, _)| e)
                        .collect::<Vec<_>>()
                        .first()
                        .expect("Can't find Player with appropriate seat number");
                    commands.entity(player_entity).insert(Dealer);
                }
                Action::GameTypeSet(game_type) => {
                    println!("Action::GameTypeSet from event source");
                    commands.entity(game_entity).insert(*game_type);
                }
                Action::GameLimitSet(game_limit) => {
                    println!("Action::GameLimitSet from event source");
                    commands.entity(game_entity).insert(*game_limit);
                }
                Action::SeatUpdated(seat_params) => {
                    println!("Action::SeatUpdated from event source");

                    let mut player_entity_command = commands.spawn((
                        Player,
                        PlayerName(seat_params.name.clone()),
                        PlayerSeatNum(seat_params.seat_num),
                    ));

                    if seat_params.npc {
                        player_entity_command.insert(PlayerNpc);
                    }

                    players_hmap.insert(seat_params.name.clone(), player_entity_command.id());
                }
                Action::StackUpdated(StackUpdatedParams { name, stack }) => {
                    println!("Action::StackUpdated from event source");

                    let player_entity = players_hmap
                        .get(name)
                        .expect("Can't find Player for updating stack");
                    commands.entity(*player_entity).insert(PlayerStack(*stack));
                }
                Action::NpcCardsDealt(NpcCardsDealtParams { name, cards }) => {
                    let player_entity = players_hmap
                        .get(name)
                        .expect("Can't find Player for updating stack");
                    commands
                        .entity(*player_entity)
                        .insert((PlayerNpc, PlayerCards(cards.clone())));
                }
                Action::CommunityCardsDealt(CommunityCardsDealtParams {
                    prev_cards,
                    new_cards,
                }) => {
                    let mut board_cards = prev_cards.clone();
                    board_cards.extend(new_cards.clone().into_iter());
                    commands
                        .entity(board_entity)
                        .insert(BoardCards(board_cards));
                }
            };
        });

    println!("Events were handled");
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

// -- For console history --
fn show_all_players_system(
    query: Query<
        (
            Entity,
            Option<&PlayerName>,
            Option<&PlayerSeatNum>,
            Option<&PlayerStack>,
            Option<&PlayerNpc>,
            Option<&Dealer>,
            Option<&PlayerCards>,
        ),
        With<Player>,
    >,
) {
    println!("----- Players ----------");
    query.for_each(|val| println!("{val:?}"));
    println!("========================");
}

fn show_game_system(
    query: Query<
        (
            Entity,
            Option<&GameType>,
            Option<&GameLimit>,
            Option<&GameHandId>,
            Option<&GameMaxSeats>,
            Option<&DealerSeatNum>,
        ),
        With<Game>,
    >,
) {
    println!("----- Game ----------");
    query.for_each(|val| println!("{val:?}"));
    println!("========================");
}

fn show_board_system(query: Query<(Entity, Option<&BoardCards>), With<Board>>) {
    println!("----- Board ----------");
    query.for_each(|val| println!("{val:?}"));
    println!("========================");
}

fn show_entity_state_system(
    archetypes: &Archetypes,
    components: &Components,
    entities: Query<Entity>,
) {
    for entity in &entities {
        println!("----- start ----------");
        println!("Entity: {:?}", entity);
        for comp_id in get_components_for_entity(&entity, archetypes).unwrap() {
            if let Some(comp_info) = components.get_info(comp_id) {
                println!("Component: {:?}", comp_info);
            }
        }
        println!("----- end ----------");
    }
}

fn get_components_for_entity<'a>(
    entity: &Entity,
    archetypes: &'a Archetypes,
) -> Option<impl Iterator<Item = ComponentId> + 'a> {
    for archetype in archetypes.iter() {
        if archetype.entities().iter().any(|e| e.entity() == *entity) {
            return Some(archetype.components());
        }
    }
    None
}

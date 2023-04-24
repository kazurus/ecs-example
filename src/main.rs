use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

use bevy::app::App;
use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::{ComponentId, Components};
use bevy::ecs::event::{Events, ManualEventReader};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Events<Action>>()
        // .add_event::<Action>()
        .add_startup_system(startup_system)
        .add_system(
            read_parser_events_for_validation
                .pipe(validate_board_cards)
                .pipe(ignore)
                .in_base_set(CoreSet::PreUpdate),
        )
        .add_system(handle_parser_events)
        .add_systems(
            (
                show_all_players_system.in_base_set(CoreSet::PostUpdateFlush),
                show_game_system.in_base_set(CoreSet::PostUpdateFlush),
                show_board_system.in_base_set(CoreSet::PostUpdateFlush),
                make_decision_system.in_base_set(CoreSet::PostUpdateFlush),
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

    let actions_vec = vec![
        Action::BetMade(BetMadeParams {
            seat_index: 4,
            bet_size: 50,
        }),
        Action::BetMade(BetMadeParams {
            seat_index: 1,
            bet_size: 100,
        }),
    ];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::NpcCardsDealt(NpcCardsDealtParams {
        name: "FluffyStutt".into(),
        cards: vec![Card::H2, Card::SK],
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::BetMade(BetMadeParams {
        seat_index: 2,
        bet_size: 100,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::BetMade(BetMadeParams {
        seat_index: 3,
        bet_size: 100,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::BetMade(BetMadeParams {
        seat_index: 4,
        bet_size: 0,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::BetMade(BetMadeParams {
        seat_index: 1,
        bet_size: 0,
    })];
    apply_batch_actions_to_app(&mut app, actions_vec);

    let actions_vec = vec![Action::CommunityCardsDealt(CommunityCardsDealtParams {
        prev_cards: vec![],
        // Duplicate cards for error in validation
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
    BetMade(BetMadeParams),
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

#[allow(unused)]
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

#[derive(Default, Debug, Clone)]
struct BetMadeParams {
    // TODO: should use name?
    seat_index: u8,
    bet_size: u64,
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

// -- Round --
#[derive(Component, Default, Debug)]
struct RoundMaxBet(u64);
// -- Round end --

// -- Board --
#[derive(Component, Default, Debug)]
struct Board;

#[derive(Component, Default, Debug)]
struct BoardCards(Vec<Card>);
// -- Board end --

// -- Player --
#[derive(Component, Default, Debug)]
struct Player;

#[derive(Component, Default, Debug,  Clone)]
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
struct PlayerRoundBetting(bool);

#[derive(Component, Default, Debug, Clone)]
struct PlayerRoundBets(Vec<u64>);

impl PlayerRoundBets {
    fn bets_sum(&self) -> u64 {
        self.0.iter().sum()
    }
}

#[derive(Component, Default, Debug)]
struct Dealer;

#[derive(Component, Default, Debug)]
struct NeedDecision(bool);
// -- Player end --

fn startup_system() {
    println!("Startup system, init some config");
}

#[derive(Default, Debug)]
struct RoundBetsCounter {
    without_raise: u8,
    allin_or_fold: u8,
    blind_bets: u8,
}

impl RoundBetsCounter {
    fn total(&self) -> u8 {
        self.without_raise + self.allin_or_fold
    }

    fn up_without_raise(&mut self) {
        self.without_raise += 1;
    }

    fn up_allin_or_fold(&mut self) {
        if self.blind_bets < 2 {
            self.blind_bets += 1;
        };

        self.allin_or_fold += 1;
    }

    fn accept_raise(&mut self) {
        if self.blind_bets < 2 {
            self.blind_bets += 1;
        } else {
            self.without_raise = 1;
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn handle_parser_events(
    mut commands: Commands,
    event_source: Res<Events<Action>>,
    mut players_entities: Query<(
        Entity,
        &PlayerSeatNum,
        Option<&mut PlayerStack>,
        Option<&mut PlayerRoundBets>,
        Option<&PlayerNpc>,
        Option<&mut PlayerRoundBetting>,
        Option<&mut NeedDecision>,
    )>,
    mut round_max_bet: Query<(Entity, &mut RoundMaxBet)>,
    mut event_reader: Local<Option<ManualEventReader<Action>>>,
    mut game_entity: Local<Option<Entity>>,
    mut board_entity: Local<Option<Entity>>,
    mut players_hmap: Local<Option<HashMap<String, Entity>>>,
    mut round_bets_counter: Local<Option<RoundBetsCounter>>,
) {
    let game_entity = game_entity
        .get_or_insert_with(|| commands.spawn(Game).id())
        .to_owned();
    let board_entity = board_entity
        .get_or_insert_with(|| commands.spawn(Board).id())
        .to_owned();
    let players_hmap = players_hmap.get_or_insert_with(HashMap::default);
    let round_bets_counter = round_bets_counter.get_or_insert_with(RoundBetsCounter::default);

    event_reader
        .get_or_insert_with(|| event_source.get_reader())
        .iter(&event_source)
        .for_each(|event| {
            match event {
                // Action::NewGameLaunched => unimplemented!(),
                Action::GameHandIdSet(hand_id) => {
                    println!("Action::GameHandIdSet from event source");
                    commands.spawn(RoundMaxBet(default()));
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

                    let (player_entity, ..) = players_entities
                        .iter()
                        .find(|(_, &PlayerSeatNum(seat_num), ..)| seat_num == *dealer_seat_num)
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
                        PlayerRoundBets(default()),
                        PlayerRoundBetting(true),
                    ));

                    if seat_params.npc {
                        player_entity_command
                            .insert(PlayerNpc)
                            .insert(NeedDecision(default()));
                    }

                    players_hmap.insert(seat_params.name.clone(), player_entity_command.id());
                    players_hmap
                        .insert(seat_params.seat_num.to_string(), player_entity_command.id());
                }
                Action::StackUpdated(StackUpdatedParams { name, stack }) => {
                    println!("Action::StackUpdated from event source");

                    let player_entity = players_hmap
                        .get(name)
                        .expect("Can't find Player for updating stack");
                    commands
                        .entity(*player_entity)
                        .insert(PlayerStack(*stack))
                        .insert(PlayerRoundBetting(true));
                }
                Action::NpcCardsDealt(NpcCardsDealtParams { name, cards }) => {
                    println!("Action::NpcCardsDealt from event source");

                    let player_entity = players_hmap
                        .get(name)
                        .expect("Can't find Player for updating stack");
                    commands.entity(*player_entity).insert((
                        PlayerNpc,
                        PlayerCards(cards.clone()),
                        NeedDecision(default()),
                    ));
                }
                Action::CommunityCardsDealt(CommunityCardsDealtParams {
                    prev_cards,
                    new_cards,
                }) => {
                    println!("Action::CommunityCardsDealt from event source");

                    let mut board_cards = prev_cards.clone();
                    board_cards.extend(new_cards.clone().into_iter());
                    commands
                        .entity(board_entity)
                        .insert(BoardCards(board_cards));
                }
                Action::BetMade(BetMadeParams {
                    seat_index: bet_seat_num,
                    bet_size,
                }) => {
                    println!("Action::BetMade from event source");

                    let (_, mut round_max_bet) = round_max_bet
                        .get_single_mut()
                        .expect("RoundMaxBet should be single and exist");

                    let mut players_entities_sorted_by_seat =
                        players_entities.iter_mut().collect::<Vec<_>>();
                    players_entities_sorted_by_seat.sort_by(|pe, pe1| pe.1 .0.cmp(&pe1.1 .0));

                    players_entities_sorted_by_seat.into_iter().for_each(
                        |(
                            _,
                            PlayerSeatNum(seat_num),
                            player_stack,
                            player_round_bets,
                            _,
                            _maybe_in_round_betting,
                            mayby_need_decision,
                        )| {
                            match mayby_need_decision {
                                Some(mut need_decision) if need_decision.0 => {
                                    need_decision.0 = false;
                                }
                                _ => (),
                            }

                            if *seat_num != *bet_seat_num {
                                return;
                            };

                            let mut player_round_bets = player_round_bets
                                .expect("PlayerRoundBets should exist for BetMade action");
                            player_round_bets.0.push(*bet_size);

                            let mut player_stack =
                                player_stack.expect("PlayerStack should exist for BetMade action");
                            player_stack.0 -= bet_size;
                            let player_round_bets_sum = player_round_bets.bets_sum();

                            if player_round_bets_sum > round_max_bet.0 && player_stack.0 == 0 {
                                round_bets_counter.up_allin_or_fold();
                                round_max_bet.0 = player_round_bets_sum;
                            } else if player_round_bets_sum > round_max_bet.0 {
                                round_bets_counter.accept_raise();
                                round_max_bet.0 = player_round_bets_sum;
                            } else if *bet_size > 0 && player_stack.0 == 0 {
                                round_bets_counter.up_allin_or_fold();
                            } else if *bet_size > 0
                                || (*bet_size == 0 && round_max_bet.0 == player_round_bets_sum)
                            {
                                round_bets_counter.up_without_raise();
                            } else if *bet_size == 0 {
                                round_bets_counter.up_allin_or_fold();
                            }

                            if player_stack.0 == 0
                                || (*bet_size == 0 && round_max_bet.0 > player_round_bets_sum)
                            {
                                _maybe_in_round_betting
                                    .expect("Player should be in betting")
                                    .0 = false;
                            }
                        },
                    );

                    let players_entities_vec = players_entities.iter_mut().collect::<Vec<_>>();
                    let mut players_entities_vec_deque = VecDeque::from(players_entities_vec);
                    let total_players = players_entities_vec_deque.len();
                    players_entities_vec_deque.rotate_left(usize::from(*bet_seat_num - 1));
                    players_entities_vec_deque.pop_front();

                    let mayby_next_player =
                        players_entities_vec_deque
                            .into_iter()
                            .find(|player_entity| {
                                player_entity
                                    .5
                                    .as_ref()
                                    .map_or(false, |is_round_betting| is_round_betting.0)
                            });

                    #[allow(unused_variables)]
                    if let Some((next_player, _, _, _, Some(is_npc), _, Some(mut need_decision))) =
                        mayby_next_player
                    {
                        let total_bets = usize::from(round_bets_counter.total());

                        if total_bets != total_players {
                            need_decision.0 = true;
                        }
                    };
                }
            };
        });

    println!("Events were handled");
}

fn read_parser_events_for_validation(
    event_source: Res<Events<Action>>,
    mut event_reader: Local<Option<ManualEventReader<Action>>>,
) -> Vec<Action> {
    event_reader
        .get_or_insert_with(|| event_source.get_reader())
        .iter(&event_source)
        .map(|event| match event {
            Action::GameHandIdSet(_) => {
                println!("Validation Action::GameHandIdSet from event source");
                Action::GameHandIdSet("111111".into())
            }
            Action::GameDealerSeatNumSet(_) => {
                println!("Validation Action::GameDealerSeatNumSet from event source");
                Action::GameDealerSeatNumSet(1)
            }
            _ => event.clone(),
        })
        .collect::<Vec<_>>()
}

fn validate_board_cards(
    In(actions): In<Vec<Action>>,
    players_cards_entities: Query<&PlayerCards>,
    board_cards_entities: Query<&BoardCards>,
) -> Vec<Action> {
    actions.iter().for_each(|action| {
        match action {
            Action::CommunityCardsDealt(CommunityCardsDealtParams {
                new_cards: cards, ..
            })
            | Action::NpcCardsDealt(NpcCardsDealtParams { cards, .. }) => {
                println!("Validator validate_board_cards - {actions:?}");

                let board_cards = match board_cards_entities.get_single() {
                    Ok(board_cards_entity) => board_cards_entity.0.clone(),
                    Err(QuerySingleError::MultipleEntities(_)) => {
                        panic!("BoardCards should be single entity")
                    }
                    _ => vec![],
                };

                let all_known_cards = players_cards_entities.iter().fold(
                    board_cards,
                    |mut all_cards, player_cards| {
                        all_cards.extend(player_cards.0.clone().into_iter());
                        all_cards
                    },
                );

                println!("Validator validate_board_cards - {all_known_cards:?} - all_known_cards");
                if all_known_cards.iter().any(|card| cards.contains(card)) {
                    println!("Validator validate_board_cards - {action:?} - has bad cards");
                }
            }
            _ => (),
        };
    });

    actions
}

fn make_decision_system(
    query: Query<(&NeedDecision, &PlayerName, &PlayerRoundBets), Changed<NeedDecision>>,
) {
    println!("Try make_decision_system");
    query.for_each(|(need_decision, player_name, player_round_bets)| {
        println!("Try make_decision_system {need_decision:?} - {player_name:?}");
        if !need_decision.0 {
            return;
        };
        println!("Will generate decision for {player_name:?} - {player_round_bets:?}");
    });
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
            // Option<&PlayerName>,
            Option<&PlayerSeatNum>,
            // Option<&PlayerStack>,
            Option<&PlayerNpc>,
            // Option<&Dealer>,
            Option<&PlayerRoundBetting>,
            Option<&NeedDecision>,
            Option<&PlayerCards>,
            Option<&PlayerRoundBets>,
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

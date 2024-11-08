use bevy::{animation::animate_targets, prelude::*};
use std::time::Duration;

use crate::{
    interaction::{Interactable, PendingInteractionExecuted},
    utils::entity::{get_n_parent, get_top_parent},
};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerAnimation>()
            .add_systems(Startup, setup_animations)
            .add_systems(
                Update,
                (
                    play_continuous_animations.before(animate_targets),
                    handle_cyclic_interaction_animations,
                    change_player_animation,
                ),
            );
    }
}

#[derive(Resource)]
pub struct AnimationLib {
    nodes: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum PlayerAnimation {
    #[default]
    Idle,
    Jogging,
}

impl PlayerAnimation {
    pub fn index(&self) -> usize {
        match self {
            Self::Idle => 0,
            Self::Jogging => 1,
        }
    }
}

#[derive(Component)]
pub struct ContinuousAnimation;

#[derive(Component)]
pub struct CyclicAnimation {
    curr: u32,
    max: u32,
}

impl CyclicAnimation {
    pub fn new(curr: u32, max: u32) -> Self {
        Self { curr, max }
    }

    pub fn cycle(&mut self) -> u32 {
        let c = self.curr;
        if self.curr == self.max {
            self.curr = 0;
        } else {
            self.curr += 1;
        }
        c
    }
}

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Build the animation graph
    let mut graph = AnimationGraph::new();
    let nodes = graph
        .add_clips(
            [
                GltfAssetLabel::Animation(PlayerAnimation::Idle.index())
                    .from_asset("models/Man.glb"),
                GltfAssetLabel::Animation(PlayerAnimation::Jogging.index())
                    .from_asset("models/Man.glb"),
                GltfAssetLabel::Animation(0).from_asset("models/Treasure_Chest.glb"),
            ]
            .into_iter()
            .map(|path| asset_server.load(path)),
            1.0,
            graph.root,
        )
        .collect();

    // Insert a resource with the current scene information
    commands.insert_resource(AnimationLib {
        nodes,
        graph: graphs.add(graph),
    });
}

fn play_continuous_animations(
    mut commands: Commands,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    continuous_animation_query: Query<&ContinuousAnimation>,
    parent_query: Query<&Parent>,
    animation_lib: Res<AnimationLib>,
) {
    for (entity, mut animation_player) in &mut animation_player_query {
        if continuous_animation_query
            .get(get_n_parent(entity, &parent_query, 2))
            .is_err()
        {
            continue;
        }

        let mut transitions = AnimationTransitions::new();

        transitions
            .play(
                &mut animation_player,
                animation_lib.nodes[PlayerAnimation::Idle.index()],
                Duration::ZERO,
            )
            .repeat();

        commands
            .entity(entity)
            .insert(animation_lib.graph.clone())
            .insert(transitions);
    }
}

fn handle_cyclic_interaction_animations(
    mut commands: Commands,
    mut event_reader: EventReader<PendingInteractionExecuted>,
    // mut cyclic_animation_query: Query<(Entity, &mut CyclicAnimation), With<Interactable>>,
    mut animation_player_query: Query<(Entity, &mut AnimationPlayer)>,
    mut cyclic_animation_query: Query<&mut CyclicAnimation, With<Interactable>>,
    parent_query: Query<&Parent>,
    animation_lib: Res<AnimationLib>,
) {
    for event in event_reader.read() {
        for (entity, mut animation_player) in &mut animation_player_query {
            let parent = get_n_parent(entity, &parent_query, 3);
            if parent.index() != event.0 {
                continue;
            }

            if let Ok(mut cyclic_animation) = cyclic_animation_query.get_mut(parent) {
                // TODO: animate entity based on what "c" value it is on

                let c = cyclic_animation.cycle();
                let i = c as usize;

                println!("animating entity {} at cycle: {}", entity.index(), c);

                // if c == 1 {
                //     animation_player.adjust_speeds(-1.0);
                // }
                animation_player.stop_all();
                animation_player.play(animation_lib.nodes[i]).replay();

                commands.entity(entity).insert(animation_lib.graph.clone());

                // let mut transitions = AnimationTransitions::new();

                // transitions.play(
                //     &mut animation_player,
                //     animation_lib.nodes[PlayerAnimation::Jogging.index()],
                //     Duration::ZERO,
                // );

                // commands
                //     .entity(entity)
                //     .insert(animation_lib.graph.clone())
                //     .insert(transitions);
            }
        }
    }
}

fn change_player_animation(
    mut animation_player_query: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    player_animation: Res<State<PlayerAnimation>>,
    mut next_player_animation: ResMut<NextState<PlayerAnimation>>,
    animation_lib: Res<AnimationLib>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (mut animation_player, mut transitions) in animation_player_query.iter_mut() {
        let is_moving =
            keys.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]);
        let animation = *player_animation.get();

        let i = if is_moving && animation != PlayerAnimation::Jogging {
            next_player_animation.set(PlayerAnimation::Jogging);
            PlayerAnimation::Jogging.index()
        } else if !is_moving && animation != PlayerAnimation::Idle {
            next_player_animation.set(PlayerAnimation::Idle);
            PlayerAnimation::Idle.index()
        } else {
            continue;
        };

        transitions
            .play(
                &mut animation_player,
                animation_lib.nodes[i],
                Duration::from_millis(250),
            )
            .repeat();
    }
}
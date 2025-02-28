//! [Component]s and [Bundle]s used by the plugin.

pub use crate::ldtk::{EntityInstance, LayerInstance};
use bevy::prelude::*;

use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

#[allow(unused_imports)]
use crate::{
    assets::LdtkLevel,
    prelude::{LdtkEntity, LdtkIntCell},
    resources::{LdtkSettings, LevelSelection},
    utils::ldtk_grid_coords_to_grid_coords,
};

use bevy_ecs_tilemap::{TileBundle, TileBundleTrait, TileParent, TilePos};

#[allow(unused_imports)]
use bevy_ecs_tilemap::Map;

/// [Component] added to any `IntGrid` tile by default.
///
/// When loading levels, you can flesh out `IntGrid` entities in your own system by querying for
/// `Added<IntGridCell>`.
/// Or, you can hook into the entity's spawning process using [LdtkIntCell].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default, Hash, Component)]
pub struct IntGridCell {
    pub value: i32,
}

/// [Component] that determines the desired levels to be loaded for an [LdtkWorldBundle].
///
/// There is an abstraction for this in the form of the [LevelSelection] resource.
/// This component does not respond to the [LdtkSettings] resource at all, while the
/// [LevelSelection] does.
/// If a [LevelSelection] is inserted, the plugin will update this component based off its value.
/// If not, [LevelSet] allows you to have more direct control over the levels you spawn.
///
/// Changes to this component are idempotent, so levels won't be respawned greedily.
#[derive(Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct LevelSet {
    pub uids: HashSet<i32>,
}

/// [Component] that indicates that an ldtk entity should be a child of the world, not the level.
///
/// By default, [LdtkEntity]s are children of the level they spawn in.
/// This can be a problem if that entity is supposed to travel across multiple levels, since they
/// will despawn the moment the level they were born in despawns.
///
/// This component makes them children of the [LdtkWorldBundle] (after one update),
/// so they can traverse levels without despawning.
/// Furthermore, this component prevents respawns of the same entity if the level they were born in
/// despawns/respawns.
/// For this purpose, it uses the values stored in this component to uniquely identify ldtk
/// entities.
///
/// Implements [LdtkEntity], and can be added to an [LdtkEntity] bundle with the `#[worldly]` field
/// attribute. See [LdtkEntity#worldly] for more details.
#[derive(Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub struct Worldly {
    pub spawn_level: i32,
    pub spawn_layer: i32,
    pub entity_def_uid: i32,
    pub spawn_px: IVec2,
}

impl Worldly {
    /// Creates a [Worldly] from the entity information available to the
    /// [LdtkEntity::bundle_entity] method.
    ///
    /// Used for the `#[worldly]` attribute macro for `#[derive(LdtkEntity)]`.
    /// See [LdtkEntity#worldly] for more info.
    pub fn from_entity_info(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
    ) -> Worldly {
        Worldly {
            spawn_level: layer_instance.level_id,
            spawn_layer: layer_instance.layer_def_uid,
            entity_def_uid: entity_instance.def_uid,
            spawn_px: entity_instance.px,
        }
    }
}

/// [Component] that stores grid-based coordinate information.
///
/// For Tile, AutoTile, and IntGrid layers, all tiles have this component by default.
///
/// Can be added to an [LdtkEntity] bundle with the `#[grid_coords]` attribute.
/// Then, it will be spawned with the initial grid-based position of the entity in LDtk.
/// See [LdtkEntity#grid_coords] for attribute macro usage.
///
/// Note that the plugin will not automatically update the entity's [Transform] when this component
/// is updated, nor visa versa.
/// This is left up to the user since there are plenty of scenarios where this behavior needs to be
/// custom.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Component)]
pub struct GridCoords {
    pub x: i32,
    pub y: i32,
}

impl From<IVec2> for GridCoords {
    fn from(i_vec_2: IVec2) -> Self {
        GridCoords {
            x: i_vec_2.x,
            y: i_vec_2.y,
        }
    }
}

impl From<GridCoords> for IVec2 {
    fn from(grid_coords: GridCoords) -> Self {
        IVec2::new(grid_coords.x, grid_coords.y)
    }
}

impl From<TilePos> for GridCoords {
    fn from(tile_pos: TilePos) -> Self {
        GridCoords {
            x: tile_pos.0 as i32,
            y: tile_pos.1 as i32,
        }
    }
}

impl Add<GridCoords> for GridCoords {
    type Output = GridCoords;
    fn add(self, rhs: GridCoords) -> Self::Output {
        GridCoords {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<GridCoords> for GridCoords {
    fn add_assign(&mut self, rhs: GridCoords) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub<GridCoords> for GridCoords {
    type Output = GridCoords;
    fn sub(self, rhs: GridCoords) -> Self::Output {
        GridCoords {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<GridCoords> for GridCoords {
    fn sub_assign(&mut self, rhs: GridCoords) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<GridCoords> for GridCoords {
    type Output = GridCoords;
    fn mul(self, rhs: GridCoords) -> Self::Output {
        GridCoords {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign<GridCoords> for GridCoords {
    fn mul_assign(&mut self, rhs: GridCoords) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl GridCoords {
    /// Creates a [GridCoords] from the entity information available to the
    /// [LdtkEntity::bundle_entity] method.
    ///
    /// Used for the `#[grid_coords]` attribute macro for `#[derive(LdtkEntity)]`.
    /// See [LdtkEntity#grid_coords] for more info.
    pub fn from_entity_info(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
    ) -> GridCoords {
        ldtk_grid_coords_to_grid_coords(entity_instance.grid, layer_instance.c_hei)
    }
}

#[derive(Clone, Default, Bundle)]
pub(crate) struct TileGridBundle {
    #[bundle]
    pub tile_bundle: TileBundle,
    pub grid_coords: GridCoords,
}

impl TileBundleTrait for TileGridBundle {
    fn get_tile_pos_mut(&mut self) -> &mut TilePos {
        self.tile_bundle.get_tile_pos_mut()
    }

    fn get_tile_parent(&mut self) -> &mut TileParent {
        self.tile_bundle.get_tile_parent()
    }
}

#[derive(Clone, Default, Bundle)]
pub(crate) struct IntGridCellBundle {
    pub int_grid_cell: IntGridCell,
}

#[derive(Clone, Bundle, Default)]
pub(crate) struct EntityInstanceBundle {
    pub entity_instance: EntityInstance,
}

/// [Bundle] for spawning LDtk worlds and their levels. The main bundle for using this plugin.
///
/// After the ldtk file is done loading, the levels you've chosen with [LevelSelection] or
/// [LevelSet] will begin to spawn.
/// Each level is its own entity, with the [LdtkWorldBundle] as its parent.
/// Each level has `Handle<LdtkLevel>`, [Map], [Transform], and [GlobalTransform] components.
/// Finally, all tiles and entities in the level are spawned as children to the level unless marked
/// by a [Worldly] component.
#[derive(Clone, Default, Bundle)]
pub struct LdtkWorldBundle {
    pub ldtk_handle: Handle<crate::assets::LdtkAsset>,
    pub level_set: LevelSet,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

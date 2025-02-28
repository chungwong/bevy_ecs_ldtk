use crate::{app::ldtk_entity::*, app::ldtk_int_cell::*};
use bevy::prelude::*;

/// Provides functions to register [Bundle]s to bevy's [App] for particular LDtk layer identifiers,
/// entity identifiers, and IntGrid values.
///
/// After being registered, [Entity]s will be spawned with these bundles when some IntGrid tile or
/// entity meets the criteria you specify.
///
/// Not necessarily intended for custom implementations on your own types.
pub trait RegisterLdtkObjects {
    /// Used internally by all the other LDtk entity registration functions.
    ///
    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it provides
    /// defaulting functionality:
    /// - Setting `layer_identifier` to [None] will make the registration apply to any Entity layer.
    /// - Setting `entity_identifier` to [None] will make the registration apply to any LDtk entity.
    ///
    /// This defaulting functionality means that a particular instance of an LDtk entity may match
    /// multiple registrations.
    /// In these cases, registrations are prioritized in order of most to least specific:
    /// 1. `layer_identifier` and `entity_identifier` are specified
    /// 2. Just `entity_identifier` is specified
    /// 3. Just `layer_identifier` is specified
    /// 4. Neither `entity_identifier` nor `layer_identifier` are specified
    fn register_ldtk_entity_for_layer_optional<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
    ) -> &mut Self;

    /// Registers [LdtkEntity] types to be spawned for a given Entity identifier and layer
    /// identifier in an LDtk file.
    ///
    /// This example lets the plugin know that it should spawn a MyBundle when it encounters a
    /// "my_entity_identifier" entity on a "MyLayerIdentifier" layer.
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// fn main() {
    ///     App::empty()
    ///         .add_plugin(LdtkPlugin)
    ///         .register_ldtk_entity_for_layer::<MyBundle>("MyLayerIdentifier", "my_entity_identifier")
    ///         // add other systems, plugins, resources...
    ///         .run();
    /// }
    ///
    /// # #[derive(Component, Default)]
    /// # struct ComponentA;
    /// # #[derive(Component, Default)]
    /// # struct ComponentB;
    /// # #[derive(Component, Default)]
    /// # struct ComponentC;
    /// #[derive(Bundle, LdtkEntity)]
    /// pub struct MyBundle {
    ///     a: ComponentA,
    ///     b: ComponentB,
    ///     c: ComponentC,
    /// }
    /// ```
    ///
    /// You can find more details on the `#[derive(LdtkEntity)]` macro at [LdtkEntity].
    fn register_ldtk_entity_for_layer<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: &str,
        entity_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            Some(entity_identifier.to_string()),
        )
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it applies the
    /// registration to all layers.
    fn register_ldtk_entity<B: LdtkEntity + Bundle>(
        &mut self,
        entity_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(None, Some(entity_identifier.to_string()))
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it applies the
    /// registration to all entities on the given layer.
    fn register_default_ldtk_entity_for_layer<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(Some(layer_identifier.to_string()), None)
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it applies the
    /// registration to any entity and any layer.
    fn register_default_ldtk_entity<B: LdtkEntity + Bundle>(&mut self) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(None, None)
    }

    /// Used internally by all the other LDtk int cell registration functions.
    ///
    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it provides
    /// defaulting functionality:
    /// - Setting `layer_identifier` to [None] will make the registration apply to any IntGrid layer.
    /// - Setting `value` to [None] will make the registration apply to any IntGrid tile.
    ///
    /// This defaulting functionality means that a particular LDtk IntGrid tile may match multiple
    /// registrations.
    /// In these cases, registrations are prioritized in order of most to least specific:
    /// 1. `layer_identifier` and `value` are specified
    /// 2. Just `value` is specified
    /// 3. Just `layer_identifier` is specified
    /// 4. Neither `value` nor `layer_identifier` are specified
    fn register_ldtk_int_cell_for_layer_optional<B: LdtkIntCell + Bundle>(
        &mut self,
        layer_identifier: Option<String>,
        value: Option<i32>,
    ) -> &mut Self;

    /// Registers [LdtkIntCell] types to be inserted for a given IntGrid value and layer identifier
    /// in an LDtk file.
    ///
    /// This example lets the plugin know that it should spawn a MyBundle when it encounters an
    /// IntGrid tile whose value is `1` on a "MyLayerIdentifier" layer.
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// fn main() {
    ///     App::empty()
    ///         .add_plugin(LdtkPlugin)
    ///         .register_ldtk_int_cell_for_layer::<MyBundle>("MyLayerIdentifier", 1)
    ///         // add other systems, plugins, resources...
    ///         .run();
    /// }
    ///
    /// # #[derive(Component, Default)]
    /// # struct ComponentA;
    /// # #[derive(Component, Default)]
    /// # struct ComponentB;
    /// # #[derive(Component, Default)]
    /// # struct ComponentC;
    /// #[derive(Bundle, LdtkIntCell)]
    /// pub struct MyBundle {
    ///     a: ComponentA,
    ///     b: ComponentB,
    ///     c: ComponentC,
    /// }
    /// ```
    ///
    /// You can find more details on the `#[derive(LdtkIntCell)]` macro at [LdtkIntCell].
    fn register_ldtk_int_cell_for_layer<B: LdtkIntCell + Bundle>(
        &mut self,
        layer_identifier: &str,
        value: i32,
    ) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            Some(value),
        )
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it applies the
    /// registration to all layers.
    fn register_ldtk_int_cell<B: LdtkIntCell + Bundle>(&mut self, value: i32) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(None, Some(value))
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it applies the
    /// registration to all tiles on the given layer.
    fn register_default_ldtk_int_cell_for_layer<B: LdtkIntCell + Bundle>(
        &mut self,
        layer_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            None,
        )
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it applies the
    /// registration to any tile and any layer.
    fn register_default_ldtk_int_cell<B: LdtkIntCell + Bundle>(&mut self) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(None, None)
    }
}

impl RegisterLdtkObjects for App {
    fn register_ldtk_entity_for_layer_optional<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
    ) -> &mut Self {
        let new_entry = Box::new(PhantomLdtkEntity::<B>::new());
        match self.world.get_non_send_resource_mut::<LdtkEntityMap>() {
            Some(mut entries) => {
                entries.insert((layer_identifier, entity_identifier), new_entry);
            }
            None => {
                let mut bundle_map = LdtkEntityMap::new();
                bundle_map.insert((layer_identifier, entity_identifier), new_entry);
                self.world.insert_non_send::<LdtkEntityMap>(bundle_map);
            }
        }
        self
    }

    fn register_ldtk_int_cell_for_layer_optional<B: LdtkIntCell + Bundle>(
        &mut self,
        layer_identifier: Option<String>,
        value: Option<i32>,
    ) -> &mut Self {
        let new_entry = Box::new(PhantomLdtkIntCell::<B>::new());
        match self.world.get_non_send_resource_mut::<LdtkIntCellMap>() {
            Some(mut entries) => {
                entries.insert((layer_identifier, value), new_entry);
            }
            None => {
                let mut bundle_map = LdtkIntCellMap::new();
                bundle_map.insert((layer_identifier, value), new_entry);
                self.world.insert_non_send::<LdtkIntCellMap>(bundle_map);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        components::{EntityInstance, IntGridCell},
        ldtk::{LayerInstance, TilesetDefinition},
    };

    #[derive(Default, Component, Debug)]
    struct ComponentA;

    #[derive(Default, Component, Debug)]
    struct ComponentB;

    #[derive(Default, Bundle, Debug)]
    struct LdtkEntityBundle {
        a: ComponentA,
        b: ComponentB,
    }

    impl LdtkEntity for LdtkEntityBundle {
        fn bundle_entity(
            _: &EntityInstance,
            _: &LayerInstance,
            _: Option<&Handle<Image>>,
            _: Option<&TilesetDefinition>,
            _: &AssetServer,
            _: &mut Assets<TextureAtlas>,
        ) -> LdtkEntityBundle {
            LdtkEntityBundle::default()
        }
    }

    #[derive(Default, Bundle)]
    struct LdtkIntCellBundle {
        a: ComponentA,
        b: ComponentB,
    }

    impl LdtkIntCell for LdtkIntCellBundle {
        fn bundle_int_cell(_: IntGridCell, _: &LayerInstance) -> LdtkIntCellBundle {
            LdtkIntCellBundle::default()
        }
    }

    #[test]
    fn test_ldtk_entity_registrations() {
        let mut app = App::new();
        app.register_ldtk_entity_for_layer::<LdtkEntityBundle>("layer", "entity_for_layer")
            .register_ldtk_entity::<LdtkEntityBundle>("entity")
            .register_default_ldtk_entity_for_layer::<LdtkEntityBundle>("default_entity_for_layer")
            .register_default_ldtk_entity::<LdtkEntityBundle>();

        let ldtk_entity_map = app.world.get_non_send_resource::<LdtkEntityMap>().unwrap();

        assert!(ldtk_entity_map.contains_key(&(
            Some("layer".to_string()),
            Some("entity_for_layer".to_string())
        )));

        assert!(ldtk_entity_map.contains_key(&(None, Some("entity".to_string()))));

        assert!(ldtk_entity_map.contains_key(&(Some("default_entity_for_layer".to_string()), None)));

        assert!(ldtk_entity_map.contains_key(&(None, None)));
    }

    #[test]
    fn test_ldtk_int_cell_registrations() {
        let mut app = App::new();
        app.register_ldtk_int_cell_for_layer::<LdtkIntCellBundle>("layer", 1)
            .register_ldtk_int_cell::<LdtkIntCellBundle>(2)
            .register_default_ldtk_int_cell_for_layer::<LdtkIntCellBundle>(
                "default_int_cell_for_layer",
            )
            .register_default_ldtk_int_cell::<LdtkIntCellBundle>();

        let ldtk_int_cell_map = app.world.get_non_send_resource::<LdtkIntCellMap>().unwrap();

        assert!(ldtk_int_cell_map.contains_key(&(Some("layer".to_string()), Some(1))));

        assert!(ldtk_int_cell_map.contains_key(&(None, Some(2))));

        assert!(
            ldtk_int_cell_map.contains_key(&(Some("default_int_cell_for_layer".to_string()), None))
        );

        assert!(ldtk_int_cell_map.contains_key(&(None, None)));
    }
}

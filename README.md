# `seldom_fn_plugin`

[![Crates.io](https://img.shields.io/crates/v/seldom_fn_plugin.svg)](https://crates.io/crates/seldom_fn_plugin)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_fn_plugin#license)
[![Crates.io](https://img.shields.io/crates/d/seldom_fn_plugin.svg)](https://crates.io/crates/seldom_fn_plugin)

`seldom_fn_plugin` allows using Rust functions in place of Bevy plugins
without sacrificing the builder pattern. This improves the ergonomics of plugin-heavy apps
and makes it possible to avoid certain `.clone()`s while maintaining modularity.

I would advise against exposing a `fn_plugin` in a public API. It is better to keep consistent
with the rest of the Bevy ecosystem in this case.

The code for this crate is only 10 lines, excluding docs and whitespace,
so you can avoid adding a dependency by just copying the code into your project.
I decided to publish it despite its length for a few reasons. First,
I want to see people use this pattern. Second, I work on many Bevy projects,
and would like to reduce the duplication of this code. Finally, I intend to publish more crates,
so it doesn't hurt to get familiar with the process.

## Examples

Here is an example usage:

```Rust
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

fn say_hi() {
    println!("hi");
}

fn my_plugin(app: &mut App) {
    app.add_system(say_hi);
}

fn main() {
    App::new().fn_plugin(my_plugin).run();
}
```

Here are some examples from a game and some other crates I'm developing:

Before:

```Rust
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>();
    }
}
```

After:

```Rust
pub fn controls_plugin(app: &mut App) {
    app.init_resource::<Controls>();
}
```

Before:

```Rust
pub(crate) struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PxAssetPlugin::<PxSpriteData>::default())
            .add_plugin(PxAssetPlugin::<PxTilesetData>::default())
            .add_plugin(PxAssetPlugin::<PxTypefaceData>::default())
            .add_plugin(PxAssetPlugin::<PxFilterData>::default());
    }
}

struct PxAssetPlugin<D: PxAssetData>(PhantomData<D>);

impl<D: PxAssetData> Plugin for PxAssetPlugin<D> {
    fn build(&self, app: &mut App) {
        app.add_asset::<PxAsset<D>>()
            .init_resource::<LoadingAssets<D>>()
            .add_system_set(SystemSet::on_update(PaletteState::Loaded).with_system(D::load));
    }
}

impl<D: PxAssetData> Default for PxAssetPlugin<D> {
    fn default() -> Self {
        Self(default())
    }
}
```

After (avoids an annoying `PhantomData`):

```Rust
pub(crate) fn asset_plugin(app: &mut App) {
    app.fn_plugin(px_asset_plugin::<PxSpriteData>)
        .fn_plugin(px_asset_plugin::<PxTilesetData>)
        .fn_plugin(px_asset_plugin::<PxTypefaceData>)
        .fn_plugin(px_asset_plugin::<PxFilterData>);
}

fn px_asset_plugin<D: PxAssetData>(app: &mut App) {
    app.add_asset::<PxAsset<D>>()
        .init_resource::<LoadingAssets<D>>()
        .add_system_set(SystemSet::on_update(PaletteState::Loaded).with_system(D::load));
}
```

Before:

```Rust
pub(crate) struct CollisionPlugin<G: PxCollisionGroup> {
    listeners: HashMap<G, HashSet<G>>,
    resolvers: HashMap<G, HashSet<G>>,
}

impl<G: PxCollisionGroup> Plugin for CollisionPlugin<G> {
    fn build(&self, app: &mut App) {
        app.add_event::<PxCollision>().add_system(detect_collisions(
            self.listeners.clone(),
            self.resolvers.clone(),
        ));
    }
}

impl<G: PxCollisionGroup> CollisionPlugin<G> {
    pub(crate) fn new(
        listeners: HashMap<G, HashSet<G>>,
        resolvers: HashMap<G, HashSet<G>>,
    ) -> Self {
        Self {
            listeners,
            resolvers,
        }
    }
}
```

After (avoids a couple `.clone()`s):

```Rust
pub(crate) fn collision_plugin<G: PxCollisionGroup>(
    listeners: HashMap<G, HashSet<G>>,
    resolvers: HashMap<G, HashSet<G>>,
) -> impl FnOnce(&mut App) {
    |app| {
        app.add_event::<PxCollision>()
            .add_system(detect_collisions(listeners, resolvers));
    }
}
```

## License

`seldom_fn_plugin` is dual-licensed under MIT and Apache 2.0 at your option.
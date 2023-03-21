//! The raw manifest data before it has been processed.
//!
//! The processing will primarily remove the string IDs and replace them by numbers.

use std::time::Duration;

use bevy::{reflect::TypeUuid, utils::HashMap};
use serde::Deserialize;

use crate::{
    items::{recipe::RecipeData, ItemCount, ItemData},
    organisms::energy::Energy,
};

use super::{Id, Item, Manifest, Recipe};

/// A utility trait to ensure that all trait bounds are satisfied.
pub(crate) trait RawManifest:
    std::fmt::Debug + TypeUuid + Send + Sync + for<'de> Deserialize<'de> + 'static
{
    /// The marker type for the manifest ID.
    type Marker: 'static + Send + Sync;

    /// The type of the processed manifest data.
    type Data: std::fmt::Debug + Send + Sync;

    /// The path of the asset.
    fn path() -> &'static str;

    /// Process the raw manifest from the asset file to the manifest data used in-game.
    fn process(&self) -> Manifest<Self::Marker, Self::Data>;
}

/// The item data as seen in the original manifest file.
///
/// This will be converted to [`crate::items::ItemData`].
#[derive(Debug, Clone, Deserialize)]
pub struct RawItemData {
    /// The maximum number of items that can fit in a stack.
    stack_size: usize,
}

impl From<&RawItemData> for ItemData {
    fn from(value: &RawItemData) -> Self {
        Self::new(value.stack_size)
    }
}

/// The item manifest as seen in the manifest file.
#[derive(Debug, Clone, Deserialize, TypeUuid)]
#[uuid = "cd9f4571-b0c4-4641-8d27-1c9c5ad4c812"]
pub(crate) struct RawItemManifest {
    /// The data for each item.
    items: HashMap<String, RawItemData>,
}

impl RawManifest for RawItemManifest {
    type Marker = Item;
    type Data = ItemData;

    fn path() -> &'static str {
        "manifests/items.manifest.json"
    }

    fn process(&self) -> Manifest<Self::Marker, Self::Data> {
        let mut manifest = Manifest::new();

        for (name, raw_data) in &self.items {
            let data = Self::Data::from(raw_data);

            manifest.insert(name, data)
        }

        manifest
    }
}

/// The recipe data as seen in the original manifest file.
///
/// This will be converted to [`crate::items::recipe::RecipeData`].
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RawRecipeData {
    /// The inputs needed to craft the recipe.
    ///
    /// Maps from the String identifier of the item to the count needed for the recipe.
    inputs: HashMap<String, usize>,

    /// The outputs generated by crafting.
    ///
    /// Maps from the String identifier of the item to the count produced by the recipe.
    outputs: HashMap<String, usize>,

    /// The time needed to craft the recipe in milliseconds.
    craft_time_ms: u64,

    /// Is work by units needed to advance this recipe?
    ///
    /// Defaults to `false`.
    #[serde(default)]
    work_required: Option<bool>,

    /// The amount of [`Energy`] produced by making this recipe, if any.
    ///
    /// This is only relevant to living structures.
    #[serde(default)]
    energy: Option<Energy>,
}

impl From<&RawRecipeData> for RecipeData {
    fn from(value: &RawRecipeData) -> Self {
        let inputs = value
            .inputs
            .iter()
            .map(|(name, count)| ItemCount::new(Id::<Item>::from_name(name), *count))
            .collect();

        let outputs = value
            .outputs
            .iter()
            .map(|(name, count)| ItemCount::new(Id::<Item>::from_name(name), *count))
            .collect();

        let craft_time = Duration::from_millis(value.craft_time_ms);

        let work_required = value.work_required.unwrap_or(false);

        RecipeData::new(inputs, outputs, craft_time, work_required, value.energy)
    }
}

/// The recipe manifest as seen in the manifest file.
#[derive(Debug, Clone, Deserialize, TypeUuid)]
#[uuid = "56d4f267-0a6e-43c2-b67f-ce4c9e962467"]
pub(crate) struct RawRecipeManifest {
    /// The data for each recipe.
    recipes: HashMap<String, RawRecipeData>,
}

impl RawManifest for RawRecipeManifest {
    type Marker = Recipe;
    type Data = RecipeData;

    fn path() -> &'static str {
        "manifests/recipes.manifest.json"
    }

    fn process(&self) -> Manifest<Self::Marker, Self::Data> {
        let mut manifest = Manifest::new();

        for (name, raw_data) in &self.recipes {
            let data = Self::Data::from(raw_data);

            manifest.insert(name, data)
        }

        manifest
    }
}

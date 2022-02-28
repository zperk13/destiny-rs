use {
    crate::{
        models::{
            types::*,
            color::Color,
            manifest_models::DestinyDisplayProperties,
        },
    },
    serde::Deserialize
};

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyInventoryItemDefinition.html#schema_Destiny-Definitions-DestinyInventoryItemDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    // TODO: a lot of properties
    pub display_properties: DestinyDisplayProperties,
    pub tooltip_notifications: Vec<TooltipNotification>,
    /// mapped to [`Collectible`](crate::models::manifest::ManifestKey::Collectible)
    pub collectible_hash: Option<Hash>,
    pub icon_watermark: Option<String>,
    pub icon_watermark_shelved: Option<String>,
    pub secondary_icon: String,
    pub secondary_overlay: String,
    pub secondary_special: String,
    pub background_color: Color,
    pub screenshot: Option<String>,
    pub item_type_display_name: String,
    pub flavor_text: String,
    pub ui_item_display_style: String,
    pub item_type_and_tier_display_name: String,
    pub display_source: String,
    pub tooltip_style: Option<String>,
    pub action: Option<ItemActionBlock>,
    pub crafting: Option<ItemCraftingBlock>,
    pub inventory: ItemInventoryBlock,
    pub set_data: Option<ItemSetBlock>,
    // TODO pub stats: Option<ItemStatBlock>,
    // mapped to ???
    pub emblem_objective_hash: Option<Hash>,
    pub equipping_block: Option<EquippingBlock>,
    pub translation_block: Option<ItemTranslationBlock>,
    pub preview: Option<ItemPreviewBlock>,
    pub quality: Option<ItemQualityBlock>,
    pub value: Option<ItemValueBlock>,
    /// mapped to [`Lore`](crate::models::manifest::ManifestKey::Lore)
    pub lore_hash: Option<Hash>,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub summary_item_hash: Option<Hash>,
    /// mapped to [`ItemCategory`](crate::models::manifest::ManifestKey::ItemCategory)
    pub item_category_hashes: Vec<Hash>,
    /// mapped to [`BreakerType`](crate::models::manifest::ManifestKey::BreakerType)
    pub breaker_type_hash: Option<Hash>,
    /// mapped to [`DamageType`](crate::models::manifest::ManifestKey::DamageType)
    pub damage_type_hashes: Option<Vec<Hash>>,
    /// mapped to [`DamageType`](crate::models::manifest::ManifestKey::DamageType)
    pub default_damage_type_hash: Option<Hash>,
    /// mapped to [`Season`](crate::models::manifest::ManifestKey::Season)
    pub season_hash: Option<Hash>,
    pub hash: Hash,
    pub redacted: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemTooltipNotification.html#schema_Destiny-Definitions-DestinyItemTooltipNotification)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TooltipNotification {
    pub display_string: String,
    pub display_style: String,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemActionBlockDefinition.html#schema_Destiny-Definitions-DestinyItemActionBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemActionBlock {
    pub verb_name: String,
    pub verb_description: String,
    pub is_positive: bool,
    pub overlay_screen_name: Option<String>,
    pub overlay_icon: Option<String>,
    pub required_cooldown_seconds: Int32,
    pub required_items: Vec<ItemActionRequiredItem>,
    pub progression_rewards: Vec<ProgressionReward>,
    pub action_type_label: Option<String>,
    pub required_location: Option<String>,
    pub required_cooldown_hash: Hash,
    pub delete_on_action: bool,
    pub consume_entire_stack: bool,
    pub use_on_acquire: bool,
}


/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemActionRequiredItemDefinition.html#schema_Destiny-Definitions-DestinyItemActionRequiredItemDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemActionRequiredItem {
    pub count: Int32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub item_hash: Hash,
    pub delete_on_action: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyProgressionRewardDefinition.html#schema_Destiny-Definitions-DestinyProgressionRewardDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProgressionReward {
    /// mapped to [`Progression`](crate::models::manifest::ManifestKey::Progression)
    pub progression_hash: Hash,
    pub amount: Int32,
    pub apply_throttles: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemInventoryBlockDefinition.html#schema_Destiny-Definitions-DestinyItemInventoryBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemInventoryBlock {
    pub stack_unique_label: String,
    pub max_stack_size: Int32,
    /// mapped to [`InventoryBucket`](crate::models::manifest::ManifestKey::InventoryBucket)
    pub bucket_type_hash: Hash,
    /// mapped to [`InventoryBucket`](crate::models::manifest::ManifestKey::InventoryBucket)
    pub recovery_bucket_type_hash: Hash,
    /// mapped to [`ItemTierType`](crate::models::manifest::ManifestKey::ItemTierType)
    pub tier_type_hash: Hash,
    pub is_instance_item: bool,
    pub tier_type_name: String,
    pub tier_type: Int32,
    pub expiration_tooltip: String,
    pub expired_in_activity_message: String,
    pub expired_in_orbit_message: String,
    pub suppress_expiration_when_objectives_complete: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemSetBlockDefinition.html#schema_Destiny-Definitions-DestinyItemSetBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSetBlock {
    pub item_list: Vec<ItemSetBlockEntry>,
    pub require_ordered_set_item_add: bool,
    pub set_is_featured: bool,
    pub set_type: String,
    pub quest_line_name: String,
    pub quest_line_description: String,
    pub quest_step_summary: String,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemSetBlockEntryDefinition.html#schema_Destiny-Definitions-DestinyItemSetBlockEntryDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemSetBlockEntry {
    pub tracking_value: Int32,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub item_hash: Hash,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemCraftingBlockDefinition.html#schema_Destiny-Definitions-DestinyItemCraftingBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemCraftingBlock {
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub output_item_hash: Hash,
    /// mapped to ???
    pub required_socket_type_hashes: Vec<Hash>,
    pub failed_requirement_strings: Vec<String>,
    /// mapped to ???
    pub base_material_requirements: Option<Hash>,
    pub bonus_plugs: Option<Vec<ItemCraftingBlockBonusPlug>>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemCraftingBlockBonusPlugDefinition.html#schema_Destiny-Definitions-DestinyItemCraftingBlockBonusPlugDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemCraftingBlockBonusPlug {
    /// mapped to ???
    pub socket_type_hash: Hash,
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub plug_item_hash: Hash,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyEquippingBlockDefinition.html#schema_Destiny-Definitions-DestinyEquippingBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EquippingBlock {
    /// mapped to ???
    pub gearset_item_hash: Option<Hash>,
    pub unique_label: String,
    pub unique_label_hash: Hash,
    /// mapped to ???
    pub equipment_slot_type_hash: Hash,
    pub attributes: Int32,
    pub ammo_type: Int32,
    pub display_strings: Vec<String>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemTranslationBlockDefinition.html#schema_Destiny-Definitions-DestinyItemTranslationBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemTranslationBlock {
    pub weapon_pattern_identifier: String,
    pub weapon_pattern_hash: Hash,
    pub default_dyes: Vec<DyeReference>,
    pub locked_dyes: Vec<DyeReference>,
    pub custom_dyes: Vec<DyeReference>,
    pub arrangements: Vec<GearArtArrangementReference>,
    pub has_geometry: bool,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-DyeReference.html#schema_Destiny-DyeReference)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DyeReference {
    pub channel_hash: Hash,
    pub dye_hash: Hash,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyGearArtArrangementReference.html#schema_Destiny-Definitions-DestinyGearArtArrangementReference)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GearArtArrangementReference {
    // mapped to ???
    pub class_hash: Hash,
    pub art_arrangement_hash: Hash,
}

/// [Bungie documentation](https://bungie-net.github.io/#/components/schemas/Destiny.Definitions.DestinyItemPreviewBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemPreviewBlock {
    pub screen_style: String,
    // mapped to ???
    pub preview_vendor_hash: Option<Hash>,
    // mapped to ???
    pub artifact_hash: Option<Hash>,
    pub preview_action_string: Option<String>,
    pub derived_item_categories: Vec<DerivedItemCategory>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Items-DestinyDerivedItemCategoryDefinition.html#schema_Destiny-Definitions-Items-DestinyDerivedItemCategoryDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DerivedItemCategory {
    pub category_description: String,
    pub items: Vec<DerivedItem>
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-Items-DestinyDerivedItemDefinition.html#schema_Destiny-Definitions-Items-DestinyDerivedItemDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DerivedItem {
    pub item_hash: Option<Hash>,
    pub item_name: String,
    pub item_detail: String,
    pub item_description: String,
    pub icon_path: String,
    pub vendor_item_index: Int32,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemQualityBlockDefinition.html#schema_Destiny-Definitions-DestinyItemQualityBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemQualityBlock {
    pub item_levels: Vec<Int32>,
    pub quality_level: Int32,
    pub infusion_category_hashes: Vec<Hash>,
    /// mapped to ???
    pub progression_level_requirement_hash: Hash,
    pub current_version: Uint32,
    pub versions: Vec<ItemVersion>,
    pub display_version_watermark_icons: Vec<String>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemVersionDefinition.html#schema_Destiny-Definitions-DestinyItemVersionDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemVersion {
    /// mapped to ???
    pub power_cap_hash: Hash
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-Definitions-DestinyItemValueBlockDefinition.html#schema_Destiny-Definitions-DestinyItemValueBlockDefinition)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemValueBlock {
    pub item_value: Vec<ItemQuantity>,
    pub value_description: Option<String>,
}

/// [Bungie documentation](https://bungie-net.github.io/multi/schema_Destiny-DestinyItemQuantity.html#schema_Destiny-DestinyItemQuantity)
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ItemQuantity {
    /// mapped to [`InventoryItem`](crate::models::manifest::ManifestKey::InventoryItem)
    pub item_hash: Hash,
    pub item_instance_id: Option<Int64>,
    pub quantity: Int32,
    pub has_conditional_visibility: bool,
}
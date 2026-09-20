---
type: "system"
package: "net.minecraft.core.component"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component

73 classes (32 top-level) across 2 packages in the processed jar; 5 changed by Loom processing; 15 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.core.component.BlockTransformer|BlockTransformer]] -- calls:6, injects_into:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.BlockTransformer_BlockTransformData|BlockTransformer$BlockTransformData]] -- calls:5 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.BlockTransformer_BlockTransformData_Builder|BlockTransformer$BlockTransformData$Builder]] -- calls:14 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.BlockTransformer_TransformParticle|BlockTransformer$TransformParticle]] -- reads:2 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers|DataComponentInitializers]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers_1|DataComponentInitializers$1]] -- calls:1, injects_into:2 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers_Initializer|DataComponentInitializers$Initializer]] -- calls:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentMap|DataComponentMap]] -- calls:9, reads:2 -- by fabric-item-api-v1, fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentMap_Builder|DataComponentMap$Builder]] -- calls:7, reads:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentPatch|DataComponentPatch]] -- calls:13, reads:13 -- by fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentPatch_Builder|DataComponentPatch$Builder]] -- calls:11 -- by fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponentPatch_SplitResult|DataComponentPatch$SplitResult]] -- calls:4 -- by fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.component.DataComponents|DataComponents]] -- reads:18 -- by fabric-item-api-v1, fabric-recipe-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.component.PatchedDataComponentMap|PatchedDataComponentMap]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.core.component.TypedDataComponent|TypedDataComponent]] -- calls:4 -- by fabric-item-api-v1, fabric-recipe-api-v1

## Declared inventory

### `net.minecraft.core.component` (14 top-level)

[[40-Interfaces/net.minecraft.core.component.BlockTransformer|BlockTransformer]], `DataComponentExactPredicate`, `DataComponentGetter`, `DataComponentHolder`, [[40-Interfaces/net.minecraft.core.component.DataComponentInitializers|DataComponentInitializers]], `DataComponentLookup`, [[40-Interfaces/net.minecraft.core.component.DataComponentMap|DataComponentMap]], [[40-Interfaces/net.minecraft.core.component.DataComponentPatch|DataComponentPatch]], `DataComponentType`, [[40-Interfaces/net.minecraft.core.component.DataComponents|DataComponents]], [[40-Interfaces/net.minecraft.core.component.PatchedDataComponentMap|PatchedDataComponentMap]], `Removed`, [[40-Interfaces/net.minecraft.core.component.TypedDataComponent|TypedDataComponent]], `package-info`

### `net.minecraft.core.component.predicates` (18 top-level)

`AnyValue`, `AttributeModifiersPredicate`, `BundlePredicate`, `ContainerPredicate`, `CustomDataPredicate`, `DamagePredicate`, `DataComponentPredicate`, `DataComponentPredicates`, `EnchantmentsPredicate`, `FireworkExplosionPredicate`, `FireworksPredicate`, `JukeboxPlayablePredicate`, `PotionsPredicate`, `TrimPredicate`, `VillagerTypePredicate`, `WritableBookPredicate`, `WrittenBookPredicate`, `package-info`


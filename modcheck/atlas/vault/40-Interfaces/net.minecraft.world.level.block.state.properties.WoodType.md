---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.properties.WoodType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.properties.WoodType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Lnet/minecraft/world/level/block/state/properties/B` | exact | invokespecial@25 in `WoodTypeBuilder.build` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `fenceGateClose` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@28 in `WoodTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `fenceGateOpen` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@37 in `WoodTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `hangingSignSoundType` | `()Lnet/minecraft/world/level/block/SoundType;` | exact | invokevirtual@19 in `WoodTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@4 in `HangingSignEditScreenMixin.init` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@20 in `HangingSignEditScreenMixin.init` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@4 in `SignEditScreenMixin.init` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@20 in `SignEditScreenMixin.init` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/world/level/block/state/properties/WoodType;)Lnet/mine` | exact | invokestatic@6 in `WoodTypeBuilder.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `soundType` | `()Lnet/minecraft/world/level/block/SoundType;` | exact | invokevirtual@10 in `WoodTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (21 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final name : Ljava/lang/String;
private final setType : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
private final soundType : Lnet/minecraft/world/level/block/SoundType;
private final hangingSignSoundType : Lnet/minecraft/world/level/block/SoundType;
private final fenceGateClose : Lnet/minecraft/sounds/SoundEvent;
private final fenceGateOpen : Lnet/minecraft/sounds/SoundEvent;
private static final TYPES : Ljava/util/Map;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final OAK : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final SPRUCE : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final BIRCH : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final ACACIA : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final CHERRY : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final JUNGLE : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final DARK_OAK : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final PALE_OAK : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final POPLAR : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final CRIMSON : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final WARPED : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final MANGROVE : Lnet/minecraft/world/level/block/state/properties/WoodType;
public static final BAMBOO : Lnet/minecraft/world/level/block/state/properties/WoodType;
public <init>(Ljava/lang/String;Lnet/minecraft/world/level/block/state/properties/BlockSetType;)V
public <init>(Ljava/lang/String;Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/SoundType;Lnet/minecraft/world/level/block/SoundType;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;)V
private static register(Lnet/minecraft/world/level/block/state/properties/WoodType;)Lnet/minecraft/world/level/block/state/properties/WoodType;
public static values()Ljava/util/stream/Stream;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public name()Ljava/lang/String;
public setType()Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public soundType()Lnet/minecraft/world/level/block/SoundType;
public hangingSignSoundType()Lnet/minecraft/world/level/block/SoundType;
public fenceGateClose()Lnet/minecraft/sounds/SoundEvent;
public fenceGateOpen()Lnet/minecraft/sounds/SoundEvent;
static <clinit>()V
```

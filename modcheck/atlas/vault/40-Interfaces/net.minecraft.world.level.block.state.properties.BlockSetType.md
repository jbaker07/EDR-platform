---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.properties.BlockSetType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.properties.BlockSetType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;ZZZLnet/minecraft/world/level/block/state/propertie` | exact | invokespecial@60 in `BlockSetTypeBuilder.build` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `buttonClickOff` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@109 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `buttonClickOn` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@118 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `canButtonBeActivatedByArrows` | `()Z` | exact | invokevirtual@28 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `canOpenByHand` | `()Z` | exact | invokevirtual@10 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `canOpenByWindCharge` | `()Z` | exact | invokevirtual@19 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `doorClose` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@55 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `doorOpen` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@64 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `pressurePlateClickOff` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@91 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `pressurePlateClickOn` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@100 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `pressurePlateSensitivity` | `()Lnet/minecraft/world/level/block/state/properties/BlockSetType$Press` | exact | invokevirtual@37 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/world/level/block/state/properties/BlockSetType;)Lnet/` | exact | invokestatic@5 in `BlockSetTypeBuilder.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `soundType` | `()Lnet/minecraft/world/level/block/SoundType;` | exact | invokevirtual@46 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `trapdoorClose` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@73 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `trapdoorOpen` | `()Lnet/minecraft/sounds/SoundEvent;` | exact | invokevirtual@82 in `BlockSetTypeBuilder.copyOf` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (34 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final name : Ljava/lang/String;
private final canOpenByHand : Z
private final canOpenByWindCharge : Z
private final canButtonBeActivatedByArrows : Z
private final pressurePlateSensitivity : Lnet/minecraft/world/level/block/state/properties/BlockSetType$PressurePlateSensitivity;
private final soundType : Lnet/minecraft/world/level/block/SoundType;
private final doorClose : Lnet/minecraft/sounds/SoundEvent;
private final doorOpen : Lnet/minecraft/sounds/SoundEvent;
private final trapdoorClose : Lnet/minecraft/sounds/SoundEvent;
private final trapdoorOpen : Lnet/minecraft/sounds/SoundEvent;
private final pressurePlateClickOff : Lnet/minecraft/sounds/SoundEvent;
private final pressurePlateClickOn : Lnet/minecraft/sounds/SoundEvent;
private final buttonClickOff : Lnet/minecraft/sounds/SoundEvent;
private final buttonClickOn : Lnet/minecraft/sounds/SoundEvent;
private static final TYPES : Ljava/util/Map;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final IRON : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final COPPER : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final GOLD : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final STONE : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final POLISHED_BLACKSTONE : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final OAK : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final SPRUCE : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final BIRCH : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final ACACIA : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final CHERRY : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final JUNGLE : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final DARK_OAK : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final PALE_OAK : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final POPLAR : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final CRIMSON : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final WARPED : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final MANGROVE : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static final BAMBOO : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public <init>(Ljava/lang/String;)V
public <init>(Ljava/lang/String;ZZZLnet/minecraft/world/level/block/state/properties/BlockSetType$PressurePlateSensitivity;Lnet/minecraft/world/level/block/SoundType;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;)V
private static register(Lnet/minecraft/world/level/block/state/properties/BlockSetType;)Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public static values()Ljava/util/stream/Stream;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public name()Ljava/lang/String;
public canOpenByHand()Z
public canOpenByWindCharge()Z
public canButtonBeActivatedByArrows()Z
public pressurePlateSensitivity()Lnet/minecraft/world/level/block/state/properties/BlockSetType$PressurePlateSensitivity;
public soundType()Lnet/minecraft/world/level/block/SoundType;
public doorClose()Lnet/minecraft/sounds/SoundEvent;
public doorOpen()Lnet/minecraft/sounds/SoundEvent;
public trapdoorClose()Lnet/minecraft/sounds/SoundEvent;
public trapdoorOpen()Lnet/minecraft/sounds/SoundEvent;
public pressurePlateClickOff()Lnet/minecraft/sounds/SoundEvent;
public pressurePlateClickOn()Lnet/minecraft/sounds/SoundEvent;
public buttonClickOff()Lnet/minecraft/sounds/SoundEvent;
public buttonClickOn()Lnet/minecraft/sounds/SoundEvent;
static <clinit>()V
```

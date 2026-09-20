---
type: "interface"
fqcn: "net.minecraft.sounds.SoundEvent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.sounds.SoundEvent

System: [[20-Systems/net.minecraft.sounds|net.minecraft.sounds]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@13 in `SoundTypeBuilder.of` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@2 in `FabricSoundsProvider$SoundExporter.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `location` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@11 in `SoundTypeBuilderImpl$RegistrationBuilderImpl.ofEvent` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (6 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final location : Lnet/minecraft/resources/Identifier;
private final fixedRange : Ljava/util/Optional;
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final DIRECT_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/resources/Identifier;Ljava/util/Optional;)V
private static create(Lnet/minecraft/resources/Identifier;Ljava/util/Optional;)Lnet/minecraft/sounds/SoundEvent;
public static createVariableRangeEvent(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/sounds/SoundEvent;
public static createFixedRangeEvent(Lnet/minecraft/resources/Identifier;F)Lnet/minecraft/sounds/SoundEvent;
public getRange(F)F
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public location()Lnet/minecraft/resources/Identifier;
public fixedRange()Ljava/util/Optional;
private static synthetic lambda$create$1(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/sounds/SoundEvent;
private static synthetic lambda$create$0(Lnet/minecraft/resources/Identifier;Ljava/lang/Float;)Lnet/minecraft/sounds/SoundEvent;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```

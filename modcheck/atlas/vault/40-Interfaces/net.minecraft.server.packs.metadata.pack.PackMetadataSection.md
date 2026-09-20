---
type: "interface"
fqcn: "net.minecraft.server.packs.metadata.pack.PackMetadataSection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.metadata.pack.PackMetadataSection

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/util/InclusiveRa` | exact | invokespecial@13 in `ModPackResourcesUtil.getMetadataPack` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `codecForPackType` | `(Lnet/minecraft/server/packs/PackType;)Lcom/mojang/serialization/Codec` | exact | invokestatic@1 in `ModPackResourcesUtil.getMetadataPackJson` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final description : Lnet/minecraft/network/chat/Component;
private final supportedFormats : Lnet/minecraft/util/InclusiveRange;
private static final FALLBACK_CODEC : Lcom/mojang/serialization/Codec;
public static final CLIENT_TYPE : Lnet/minecraft/server/packs/metadata/MetadataSectionType;
public static final SERVER_TYPE : Lnet/minecraft/server/packs/metadata/MetadataSectionType;
public static final FALLBACK_TYPE : Lnet/minecraft/server/packs/metadata/MetadataSectionType;
public <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/util/InclusiveRange;)V
private static codecForPackType(Lnet/minecraft/server/packs/PackType;)Lcom/mojang/serialization/Codec;
public static forPackType(Lnet/minecraft/server/packs/PackType;)Lnet/minecraft/server/packs/metadata/MetadataSectionType;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public description()Lnet/minecraft/network/chat/Component;
public supportedFormats()Lnet/minecraft/util/InclusiveRange;
private static synthetic lambda$codecForPackType$0(Lnet/minecraft/server/packs/PackType;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$1(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/server/packs/metadata/pack/PackMetadataSection;
static <clinit>()V
```

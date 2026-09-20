---
type: "interface"
fqcn: "net.minecraft.tags.TagFile"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagFile

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`record` public final; extends `java/lang/Record`; implements `net/fabricmc/fabric/api/tag/v1/FabricTagFile`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `entries` | `()Ljava/util/List;` | exact | invokevirtual@128 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `remove` | `()Ljava/util/List;` | inherited_exact | invokevirtual@138 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `remove` | `()Ljava/util/List;` | inherited_exact | invokevirtual@2 in `TagLoaderMixin.loadRemoveEntries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `replace` | `()Z` | exact | invokevirtual@111 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `<clinit>` | `()V` | exact | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@70 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entries : Ljava/util/List;
private final replace : Z
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Ljava/util/List;Z)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public entries()Ljava/util/List;
public replace()Z
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```

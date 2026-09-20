---
type: "interface"
fqcn: "net.minecraft.tags.TagEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.TagEntry

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;ZZ)V` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `build(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Con` | `` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `build(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Con` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `element(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/Ta` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `optionalElement(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/Ta` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `optionalTag(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/Ta` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `tag(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/tags/Ta` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (24, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.tags.TagEntry {
    private static final com.mojang.serialization.Codec<net.minecraft.tags.TagEntry> FULL_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.tags.TagEntry> CODEC;
    private final net.minecraft.resources.Identifier id;
    private final boolean tag;
    private final boolean required;
    private net.minecraft.tags.TagEntry(net.minecraft.resources.Identifier, boolean, boolean);
    private net.minecraft.tags.TagEntry(net.minecraft.util.ExtraCodecs$TagOrElementLocation, boolean);
    private net.minecraft.util.ExtraCodecs$TagOrElementLocation elementOrTag();
    public static net.minecraft.tags.TagEntry element(net.minecraft.resources.Identifier);
    public static net.minecraft.tags.TagEntry optionalElement(net.minecraft.resources.Identifier);
    public static net.minecraft.tags.TagEntry tag(net.minecraft.resources.Identifier);
    public static net.minecraft.tags.TagEntry optionalTag(net.minecraft.resources.Identifier);
    public <T> boolean build(net.minecraft.tags.TagEntry$Lookup<T>, java.util.function.Consumer<T>);
    public void visitRequiredDependencies(java.util.function.Consumer<net.minecraft.resources.Identifier>);
    public void visitOptionalDependencies(java.util.function.Consumer<net.minecraft.resources.Identifier>);
    public boolean verifyIfPresent(java.util.function.Predicate<net.minecraft.resources.Identifier>, java.util.function.Predicate<net.minecraft.resources.Identifier>);
    public java.lang.String toString();
    private static com.mojang.datafixers.util.Either lambda$static$5(net.minecraft.tags.TagEntry);
    private static net.minecraft.tags.TagEntry lambda$static$2(com.mojang.datafixers.util.Either);
    private static net.minecraft.tags.TagEntry lambda$static$4(net.minecraft.tags.TagEntry);
    private static net.minecraft.tags.TagEntry lambda$static$3(net.minecraft.util.ExtraCodecs$TagOrElementLocation);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.lang.Boolean lambda$static$1(net.minecraft.tags.TagEntry);
    static {};
}
```

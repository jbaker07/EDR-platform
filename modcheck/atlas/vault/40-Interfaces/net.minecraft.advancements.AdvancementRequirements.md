---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementRequirements"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementRequirements

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/List;)V` | `` | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `requirements()Ljava/util/List;` | `` | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `EMPTYLnet/minecraft/advancements/AdvancementRequirements;` | `` | both | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.advancements.AdvancementRequirements extends java.lang.Record {
    private final java.util.List<java.util.List<java.lang.String>> requirements;
    public static final com.mojang.serialization.Codec<net.minecraft.advancements.AdvancementRequirements> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.advancements.AdvancementRequirements> STREAM_CODEC;
    public static final net.minecraft.advancements.AdvancementRequirements EMPTY;
    public net.minecraft.advancements.AdvancementRequirements(java.util.List<java.util.List<java.lang.String>>);
    public static net.minecraft.advancements.AdvancementRequirements allOf(java.util.Collection<java.lang.String>);
    public static net.minecraft.advancements.AdvancementRequirements anyOf(java.util.Collection<java.lang.String>);
    public int size();
    public boolean test(java.util.function.Predicate<java.lang.String>);
    public int count(java.util.function.Predicate<java.lang.String>);
    private static boolean anyMatch(java.util.List<java.lang.String>, java.util.function.Predicate<java.lang.String>);
    public com.mojang.serialization.DataResult<net.minecraft.advancements.AdvancementRequirements> validate(java.util.Set<java.lang.String>);
    public boolean isEmpty();
    public java.lang.String toString();
    public java.util.Set<java.lang.String> names();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.List<java.util.List<java.lang.String>> requirements();
    private static java.lang.String lambda$validate$1(java.util.Set, java.util.Set);
    private static java.lang.String lambda$validate$0();
    static {};
}
```

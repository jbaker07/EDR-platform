---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementHolder

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `value()Lnet/minecraft/advancements/Advancement;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.advancements.AdvancementHolder extends java.lang.Record {
    private final net.minecraft.resources.Identifier id;
    private final net.minecraft.advancements.Advancement value;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.advancements.AdvancementHolder> STREAM_CODEC;
    public net.minecraft.advancements.AdvancementHolder(net.minecraft.resources.Identifier, net.minecraft.advancements.Advancement);
    public void register(net.minecraft.data.worldgen.BootstrapContext<net.minecraft.advancements.Advancement>);
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    public net.minecraft.resources.Identifier id();
    public net.minecraft.advancements.Advancement value();
    static {};
}
```

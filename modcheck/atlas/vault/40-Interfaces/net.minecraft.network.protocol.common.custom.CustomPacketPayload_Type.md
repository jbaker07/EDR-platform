---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> extends java.lang.Record {
    private final net.minecraft.resources.Identifier id;
    public net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type(net.minecraft.resources.Identifier);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.Identifier id();
}
```

---
type: "interface"
fqcn: "net.minecraft.client.KeyMapping$Category"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.KeyMapping$Category

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| injects_into | `register(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/KeyMapping$Category;` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.KeyMapping$Category extends java.lang.Record {
    private final net.minecraft.resources.Identifier id;
    private static final java.util.List<net.minecraft.client.KeyMapping$Category> SORT_ORDER;
    public static final net.minecraft.client.KeyMapping$Category MOVEMENT;
    public static final net.minecraft.client.KeyMapping$Category MISC;
    public static final net.minecraft.client.KeyMapping$Category MULTIPLAYER;
    public static final net.minecraft.client.KeyMapping$Category GAMEPLAY;
    public static final net.minecraft.client.KeyMapping$Category INVENTORY;
    public static final net.minecraft.client.KeyMapping$Category CREATIVE;
    public static final net.minecraft.client.KeyMapping$Category SPECTATOR;
    public static final net.minecraft.client.KeyMapping$Category DEBUG;
    public net.minecraft.client.KeyMapping$Category(net.minecraft.resources.Identifier);
    private static net.minecraft.client.KeyMapping$Category register(java.lang.String);
    public static net.minecraft.client.KeyMapping$Category register(net.minecraft.resources.Identifier);
    public net.minecraft.network.chat.Component label();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.Identifier id();
    static {};
}
```

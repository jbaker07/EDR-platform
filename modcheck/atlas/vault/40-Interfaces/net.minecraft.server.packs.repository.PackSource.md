---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.PackSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.PackSource

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `SERVERLnet/minecraft/server/packs/repository/PackSource;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `WORLDLnet/minecraft/server/packs/repository/PackSource;` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.packs.repository.PackSource {
    public static final java.util.function.UnaryOperator<net.minecraft.network.chat.Component> NO_DECORATION;
    public static final net.minecraft.server.packs.repository.PackSource DEFAULT;
    public static final net.minecraft.server.packs.repository.PackSource BUILT_IN;
    public static final net.minecraft.server.packs.repository.PackSource FEATURE;
    public static final net.minecraft.server.packs.repository.PackSource WORLD;
    public static final net.minecraft.server.packs.repository.PackSource SERVER;
    public abstract net.minecraft.network.chat.Component decorate(net.minecraft.network.chat.Component);
    public abstract boolean shouldAddAutomatically();
    public static net.minecraft.server.packs.repository.PackSource create(java.util.function.UnaryOperator<net.minecraft.network.chat.Component>, boolean);
    private static java.util.function.UnaryOperator<net.minecraft.network.chat.Component> decorateWithSource(java.lang.String);
    private static net.minecraft.network.chat.Component lambda$decorateWithSource$0(net.minecraft.network.chat.Component, net.minecraft.network.chat.Component);
    static {};
}
```

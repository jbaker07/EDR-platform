---
type: "interface"
fqcn: "net.minecraft.server.network.CommonListenerCookie"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.CommonListenerCookie

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createInitial(Lcom/mojang/authlib/GameProfile;Z)Lnet/minecraft/server/net` | `` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.network.CommonListenerCookie extends java.lang.Record {
    private final com.mojang.authlib.GameProfile gameProfile;
    private final int latency;
    private final net.minecraft.server.level.ClientInformation clientInformation;
    private final boolean transferred;
    public net.minecraft.server.network.CommonListenerCookie(com.mojang.authlib.GameProfile, int, net.minecraft.server.level.ClientInformation, boolean);
    public static net.minecraft.server.network.CommonListenerCookie createInitial(com.mojang.authlib.GameProfile, boolean);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public com.mojang.authlib.GameProfile gameProfile();
    public int latency();
    public net.minecraft.server.level.ClientInformation clientInformation();
    public boolean transferred();
}
```

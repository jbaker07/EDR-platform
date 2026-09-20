---
type: "interface"
fqcn: "net.minecraft.server.players.NameAndId"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.NameAndId

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lcom/mojang/authlib/GameProfile;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `"<init>"(Lcom/mojang/authlib/GameProfile;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.players.NameAndId extends java.lang.Record {
    private final java.util.UUID id;
    private final java.lang.String name;
    public static final com.mojang.serialization.Codec<net.minecraft.server.players.NameAndId> CODEC;
    public net.minecraft.server.players.NameAndId(com.mojang.authlib.GameProfile);
    public net.minecraft.server.players.NameAndId(com.mojang.authlib.services.response.NameAndId);
    public net.minecraft.server.players.NameAndId(java.util.UUID, java.lang.String);
    public static net.minecraft.server.players.NameAndId fromJson(com.google.gson.JsonObject);
    public void appendTo(com.google.gson.JsonObject);
    public static net.minecraft.server.players.NameAndId createOffline(java.lang.String);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.UUID id();
    public java.lang.String name();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```

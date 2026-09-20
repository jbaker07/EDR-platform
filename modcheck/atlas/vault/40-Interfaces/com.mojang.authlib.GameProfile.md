---
type: "interface"
fqcn: "com.mojang.authlib.GameProfile"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.authlib.GameProfile

Package `com.mojang.authlib`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/UUID;Ljava/lang/String;)V` | exact | invokespecial@17 in `FakePlayer.<clinit>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `id` | `()Ljava/util/UUID;` | exact | invokevirtual@18 in `TestServerConnectionImpl.getServerPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@22 in `ClientConfigurationNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@12 in `ClientPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@17 in `ServerConfigurationNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.

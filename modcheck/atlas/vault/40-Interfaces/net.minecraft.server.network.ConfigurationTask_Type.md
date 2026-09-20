---
type: "interface"
fqcn: "net.minecraft.server.network.ConfigurationTask$Type"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ConfigurationTask$Type

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `equals(Ljava/lang/Object;)Z` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Ljava/lang/String;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.network.ConfigurationTask$Type extends java.lang.Record {
    private final java.lang.String id;
    public net.minecraft.server.network.ConfigurationTask$Type(java.lang.String);
    public java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String id();
}
```

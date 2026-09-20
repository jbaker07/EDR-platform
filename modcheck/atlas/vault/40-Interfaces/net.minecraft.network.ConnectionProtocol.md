---
type: "interface"
fqcn: "net.minecraft.network.ConnectionProtocol"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.ConnectionProtocol

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `id()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.ConnectionProtocol extends java.lang.Enum<net.minecraft.network.ConnectionProtocol> {
    public static final net.minecraft.network.ConnectionProtocol HANDSHAKING;
    public static final net.minecraft.network.ConnectionProtocol PLAY;
    public static final net.minecraft.network.ConnectionProtocol STATUS;
    public static final net.minecraft.network.ConnectionProtocol LOGIN;
    public static final net.minecraft.network.ConnectionProtocol CONFIGURATION;
    private final java.lang.String id;
    private static final net.minecraft.network.ConnectionProtocol[] $VALUES;
    public static net.minecraft.network.ConnectionProtocol[] values();
    public static net.minecraft.network.ConnectionProtocol valueOf(java.lang.String);
    private net.minecraft.network.ConnectionProtocol(java.lang.String);
    public java.lang.String id();
    private static net.minecraft.network.ConnectionProtocol[] $values();
    static {};
}
```

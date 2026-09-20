---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.resolver.ServerAddress"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.resolver.ServerAddress

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `parseString(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/resolv` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.multiplayer.resolver.ServerAddress {
    private static final org.slf4j.Logger LOGGER;
    private final com.google.common.net.HostAndPort hostAndPort;
    private static final net.minecraft.client.multiplayer.resolver.ServerAddress INVALID;
    public net.minecraft.client.multiplayer.resolver.ServerAddress(java.lang.String, int);
    private net.minecraft.client.multiplayer.resolver.ServerAddress(com.google.common.net.HostAndPort);
    public java.lang.String getHost();
    public int getPort();
    public static net.minecraft.client.multiplayer.resolver.ServerAddress parseString(java.lang.String);
    public static boolean isValidAddress(java.lang.String);
    public static int parsePort(java.lang.String);
    public java.lang.String toString();
    public boolean equals(java.lang.Object);
    public int hashCode();
    static {};
}
```

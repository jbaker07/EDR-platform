---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ServerData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ServerData

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/client/m` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (35, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.ServerData {
    private static final org.slf4j.Logger LOGGER;
    private static final int MAX_ICON_SIZE;
    public java.lang.String name;
    public java.lang.String ip;
    public net.minecraft.network.chat.Component status;
    public net.minecraft.network.chat.Component motd;
    public net.minecraft.network.protocol.status.ServerStatus$Players players;
    public long ping;
    public int protocol;
    public net.minecraft.network.chat.Component version;
    public java.util.List<net.minecraft.network.chat.Component> playerList;
    private net.minecraft.client.multiplayer.ServerData$ServerPackStatus packStatus;
    private byte[] iconBytes;
    private net.minecraft.client.multiplayer.ServerData$Type type;
    private int acceptedCodeOfConduct;
    private net.minecraft.client.multiplayer.ServerData$State state;
    public net.minecraft.client.multiplayer.ServerData(java.lang.String, java.lang.String, net.minecraft.client.multiplayer.ServerData$Type);
    public net.minecraft.nbt.CompoundTag write();
    public net.minecraft.client.multiplayer.ServerData$ServerPackStatus getResourcePackStatus();
    public void setResourcePackStatus(net.minecraft.client.multiplayer.ServerData$ServerPackStatus);
    public static net.minecraft.client.multiplayer.ServerData read(net.minecraft.nbt.CompoundTag);
    public byte[] getIconBytes();
    public void setIconBytes(byte[]);
    public boolean isLan();
    public boolean isRealm();
    public net.minecraft.client.multiplayer.ServerData$Type type();
    public boolean hasAcceptedCodeOfConduct(java.lang.String);
    public void acceptCodeOfConduct(java.lang.String);
    public void clearCodeOfConduct();
    public void copyNameIconFrom(net.minecraft.client.multiplayer.ServerData);
    public void copyFrom(net.minecraft.client.multiplayer.ServerData);
    public net.minecraft.client.multiplayer.ServerData$State state();
    public void setState(net.minecraft.client.multiplayer.ServerData$State);
    public static byte[] validateIcon(byte[]);
    static {};
}
```

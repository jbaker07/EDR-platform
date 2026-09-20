---
type: "interface"
fqcn: "net.minecraft.client.GameNarrator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.GameNarrator

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `saySystemChatQueued(Lnet/minecraft/network/chat/Component;)V` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.GameNarrator {
    public static final net.minecraft.network.chat.Component NO_TITLE;
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.client.Minecraft minecraft;
    private final com.mojang.text2speech.Narrator narrator;
    public net.minecraft.client.GameNarrator(net.minecraft.client.Minecraft);
    public void sayChatQueued(net.minecraft.network.chat.Component);
    public void saySystemChatQueued(net.minecraft.network.chat.Component);
    public void saySystemQueued(net.minecraft.network.chat.Component);
    private void narrateNotInterruptingMessage(net.minecraft.network.chat.Component);
    public void saySystemNow(net.minecraft.network.chat.Component);
    public void saySystemNow(java.lang.String);
    private void narrateMessage(java.lang.String, boolean);
    private net.minecraft.client.NarratorStatus getStatus();
    private void logNarratedMessage(java.lang.String);
    public void updateNarratorStatus(net.minecraft.client.NarratorStatus);
    public boolean isActive();
    public void clear();
    public void destroy();
    public void checkStatus(boolean);
    static {};
}
```

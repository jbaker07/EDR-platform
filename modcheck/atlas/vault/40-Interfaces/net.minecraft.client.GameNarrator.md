---
type: "interface"
fqcn: "net.minecraft.client.GameNarrator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.GameNarrator

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `saySystemChatQueued` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@25 in `ClientSuggestionProviderMixin.sendFeedback` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (4 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NO_TITLE : Lnet/minecraft/network/chat/Component;
private static final LOGGER : Lorg/slf4j/Logger;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final narrator : Lcom/mojang/text2speech/Narrator;
public <init>(Lnet/minecraft/client/Minecraft;)V
public sayChatQueued(Lnet/minecraft/network/chat/Component;)V
public saySystemChatQueued(Lnet/minecraft/network/chat/Component;)V
public saySystemQueued(Lnet/minecraft/network/chat/Component;)V
private narrateNotInterruptingMessage(Lnet/minecraft/network/chat/Component;)V
public saySystemNow(Lnet/minecraft/network/chat/Component;)V
public saySystemNow(Ljava/lang/String;)V
private narrateMessage(Ljava/lang/String;Z)V
private getStatus()Lnet/minecraft/client/NarratorStatus;
private logNarratedMessage(Ljava/lang/String;)V
public updateNarratorStatus(Lnet/minecraft/client/NarratorStatus;)V
public isActive()Z
public clear()V
public destroy()V
public checkStatus(Z)V
static <clinit>()V
```

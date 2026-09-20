---
type: "interface"
fqcn: "net.minecraft.network.chat.MutableComponent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.MutableComponent

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `append(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/netwo` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `append(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/netwo` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `append(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `append(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/netwo` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `append(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/netwo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `append(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/netwo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `withStyle(Lnet/minecraft/ChatFormatting;)Lnet/minecraft/network/chat/` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (24, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.chat.MutableComponent implements net.minecraft.network.chat.Component {
    private final net.minecraft.network.chat.ComponentContents contents;
    private final java.util.List<net.minecraft.network.chat.Component> siblings;
    private net.minecraft.network.chat.Style style;
    private net.minecraft.util.FormattedCharSequence visualOrderText;
    private net.minecraft.locale.Language decomposedWith;
    net.minecraft.network.chat.MutableComponent(net.minecraft.network.chat.ComponentContents, java.util.List<net.minecraft.network.chat.Component>, net.minecraft.network.chat.Style);
    public static net.minecraft.network.chat.MutableComponent create(net.minecraft.network.chat.ComponentContents);
    public net.minecraft.network.chat.ComponentContents getContents();
    public java.util.List<net.minecraft.network.chat.Component> getSiblings();
    public net.minecraft.network.chat.MutableComponent setStyle(net.minecraft.network.chat.Style);
    public net.minecraft.network.chat.Style getStyle();
    public net.minecraft.network.chat.MutableComponent append(java.lang.String);
    public net.minecraft.network.chat.MutableComponent append(net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.MutableComponent withStyle(java.util.function.UnaryOperator<net.minecraft.network.chat.Style>);
    public net.minecraft.network.chat.MutableComponent withStyle(net.minecraft.network.chat.Style);
    public net.minecraft.network.chat.MutableComponent withStyle(net.minecraft.ChatFormatting...);
    public net.minecraft.network.chat.MutableComponent withStyle(net.minecraft.ChatFormatting);
    public net.minecraft.network.chat.MutableComponent withColor(int);
    public net.minecraft.network.chat.MutableComponent withColor(net.minecraft.network.chat.TextColor);
    public net.minecraft.network.chat.MutableComponent withoutShadow();
    public net.minecraft.util.FormattedCharSequence getVisualOrderText();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
}
```

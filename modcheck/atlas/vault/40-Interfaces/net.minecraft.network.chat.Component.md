---
type: "interface"
fqcn: "net.minecraft.network.chat.Component"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.Component

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `copy()Lnet/minecraft/network/chat/MutableComponent;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `empty()Lnet/minecraft/network/chat/MutableComponent;` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `empty()Lnet/minecraft/network/chat/MutableComponent;` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `empty()Lnet/minecraft/network/chat/MutableComponent;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getContents()Lnet/minecraft/network/chat/ComponentContents;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getString()Ljava/lang/String;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getVisualOrderText()Lnet/minecraft/util/FormattedCharSequence;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `literal(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `nullToEmpty(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;)Lnet/minecraft/network/chat/MutableCompo` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatable(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatableEscape(Ljava/lang/String;[Ljava/lang/Object;)Lnet/minecraft/networ` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `translatableWithFallback(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/network` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (37, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.chat.Component extends com.mojang.brigadier.Message,net.minecraft.network.chat.FormattedText {
    public abstract net.minecraft.network.chat.Style getStyle();
    public abstract net.minecraft.network.chat.ComponentContents getContents();
    public default java.lang.String getString();
    public default java.lang.String getString(int);
    public abstract java.util.List<net.minecraft.network.chat.Component> getSiblings();
    public default java.lang.String tryCollapseToString();
    public default net.minecraft.network.chat.MutableComponent plainCopy();
    public default net.minecraft.network.chat.MutableComponent copy();
    public abstract net.minecraft.util.FormattedCharSequence getVisualOrderText();
    public default <T> java.util.Optional<T> visit(net.minecraft.network.chat.FormattedText$StyledContentConsumer<T>, net.minecraft.network.chat.Style);
    public default <T> java.util.Optional<T> visit(net.minecraft.network.chat.FormattedText$ContentConsumer<T>);
    public default java.util.List<net.minecraft.network.chat.Component> toFlatList();
    public default java.util.List<net.minecraft.network.chat.Component> toFlatList(net.minecraft.network.chat.Style);
    public default boolean contains(net.minecraft.network.chat.Component);
    public static net.minecraft.network.chat.Component nullToEmpty(java.lang.String);
    public static net.minecraft.network.chat.MutableComponent literal(java.lang.String);
    public static net.minecraft.network.chat.MutableComponent translatable(java.lang.String);
    public static net.minecraft.network.chat.MutableComponent translatable(java.lang.String, java.lang.Object...);
    public static net.minecraft.network.chat.MutableComponent translatableEscape(java.lang.String, java.lang.Object...);
    public static net.minecraft.network.chat.MutableComponent translatableWithFallback(java.lang.String, java.lang.String);
    public static net.minecraft.network.chat.MutableComponent translatableWithFallback(java.lang.String, java.lang.String, java.lang.Object...);
    public static net.minecraft.network.chat.MutableComponent empty();
    public static net.minecraft.network.chat.MutableComponent keybind(java.lang.String);
    public static net.minecraft.network.chat.MutableComponent nbt(net.minecraft.util.CompilableString<net.minecraft.commands.arguments.NbtPathArgument$NbtPath>, boolean, boolean, java.util.Optional<net.minecraft.network.chat.Component>, net.minecraft.network.chat.contents.data.DataSource);
    public static net.minecraft.network.chat.MutableComponent score(net.minecraft.util.CompilableString<net.minecraft.commands.arguments.selector.EntitySelector>, java.lang.String);
    public static net.minecraft.network.chat.MutableComponent score(java.lang.String, java.lang.String);
    public static net.minecraft.network.chat.MutableComponent selector(net.minecraft.util.CompilableString<net.minecraft.commands.arguments.selector.EntitySelector>, java.util.Optional<net.minecraft.network.chat.Component>);
    public static net.minecraft.network.chat.MutableComponent object(net.minecraft.network.chat.contents.objects.ObjectInfo);
    public static net.minecraft.network.chat.MutableComponent object(net.minecraft.network.chat.contents.objects.ObjectInfo, net.minecraft.network.chat.Component);
    public static net.minecraft.network.chat.Component translationArg(java.util.Date);
    public static net.minecraft.network.chat.Component translationArg(com.mojang.brigadier.Message);
    public static net.minecraft.network.chat.Component translationArg(java.util.UUID);
    public static net.minecraft.network.chat.Component translationArg(net.minecraft.resources.Identifier);
    public static net.minecraft.network.chat.Component translationArg(net.minecraft.world.level.ChunkPos);
    public static net.minecraft.network.chat.Component translationArg(java.net.URI);
    private static java.util.Optional lambda$toFlatList$0(java.util.List, net.minecraft.network.chat.Style, java.lang.String);
    private static java.util.Optional lambda$getString$0(int, java.lang.StringBuilder, java.lang.String);
}
```

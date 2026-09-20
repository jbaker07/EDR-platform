---
type: "interface"
fqcn: "com.mojang.brigadier.CommandDispatcher"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.brigadier.CommandDispatcher

Package `com.mojang.brigadier`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@4 in `ClientPacketListenerMixin.onGameJoin` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `execute` | `(Ljava/lang/String;Ljava/lang/Object;)I` | exact | invokevirtual@52 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `findAmbiguities` | `(Lcom/mojang/brigadier/AmbiguityConsumer;)V` | exact | invokevirtual@102 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getPath` | `(Lcom/mojang/brigadier/tree/CommandNode;)Ljava/util/Collection;` | exact | invokevirtual@16 in `ClientCommandInternals.lambda$finalizeInit$0` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getPath` | `(Lcom/mojang/brigadier/tree/CommandNode;)Ljava/util/Collection;` | exact | invokevirtual@26 in `ClientCommandInternals.lambda$finalizeInit$0` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRoot` | `()Lcom/mojang/brigadier/tree/RootCommandNode;` | exact | invokevirtual@3 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRoot` | `()Lcom/mojang/brigadier/tree/RootCommandNode;` | exact | invokevirtual@3 in `ClientCommandInternals.executeRootHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRoot` | `()Lcom/mojang/brigadier/tree/RootCommandNode;` | exact | invokevirtual@12 in `ClientCommandInternals.addCommands` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRoot` | `()Lcom/mojang/brigadier/tree/RootCommandNode;` | exact | invokevirtual@16 in `ClientCommandInternals.addCommands` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRoot` | `()Lcom/mojang/brigadier/tree/RootCommandNode;` | exact | invokevirtual@28 in `ClientCommandInternals.addCommands` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getRoot` | `()Lcom/mojang/brigadier/tree/RootCommandNode;` | exact | invokevirtual@32 in `ClientCommandInternals.addCommands` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getSmartUsage` | `(Lcom/mojang/brigadier/tree/CommandNode;Ljava/lang/Object;)Ljava/util/` | exact | invokevirtual@11 in `ClientCommandInternals.executeHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `parse` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/brigadier/ParseResul` | exact | invokevirtual@5 in `ClientCommandInternals.requiresConfirmation` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `parse` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/brigadier/ParseResul` | exact | invokevirtual@23 in `ClientCommandInternals.requiresConfirmation` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `parse` | `(Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/brigadier/ParseResul` | exact | invokevirtual@16 in `ClientCommandInternals.executeArgumentHelp` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `register` | `(Lcom/mojang/brigadier/builder/LiteralArgumentBuilder;)Lcom/mojang/bri` | exact | invokevirtual@70 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `register` | `(Lcom/mojang/brigadier/builder/LiteralArgumentBuilder;)Lcom/mojang/bri` | exact | invokevirtual@90 in `ClientCommandInternals.finalizeInit` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.

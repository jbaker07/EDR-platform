---
type: "interface"
fqcn: "net.minecraft.server.commands.data.BlockDataAccessor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands.data.BlockDataAccessor

System: [[20-Systems/net.minecraft.server.commands|net.minecraft.server.commands]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/commands/data/DataAccessor`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `setData` | `(Lnet/minecraft/nbt/CompoundTag;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (5 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final ERROR_NOT_A_BLOCK_ENTITY : Lcom/mojang/brigadier/exceptions/SimpleCommandExceptionType;
public static final PROVIDER : Lnet/minecraft/server/commands/ArgProvider$Factory;
private final entity : Lnet/minecraft/world/level/block/entity/BlockEntity;
private final pos : Lnet/minecraft/core/BlockPos;
public <init>(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/core/BlockPos;)V
public setData(Lnet/minecraft/nbt/CompoundTag;)V
public getData()Lnet/minecraft/nbt/CompoundTag;
public getModifiedSuccess()Lnet/minecraft/network/chat/Component;
public getPrintSuccess(Lnet/minecraft/nbt/Tag;)Lnet/minecraft/network/chat/Component;
public getPrintSuccess(Lnet/minecraft/commands/arguments/NbtPathArgument$NbtPath;DI)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$static$0(Ljava/lang/String;)Lnet/minecraft/server/commands/ArgProvider;
private static synthetic lambda$static$2(Ljava/lang/String;Lcom/mojang/brigadier/context/CommandContext;)Lnet/minecraft/server/commands/data/DataAccessor;
private static synthetic lambda$static$1(Ljava/lang/String;)Lcom/mojang/brigadier/builder/ArgumentBuilder;
static <clinit>()V
```

---
type: "interface"
fqcn: "net.minecraft.commands.arguments.EntityAnchorArgument$Anchor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.arguments.EntityAnchorArgument$Anchor

System: [[20-Systems/net.minecraft.commands.arguments|net.minecraft.commands.arguments]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `EYES` | `Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;` | exact | getstatic@22 in `TestInputImpl.lambda$lookAt$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FEET : Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
public static final EYES : Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
private static final BY_NAME : Ljava/util/Map;
private final name : Ljava/lang/String;
private final transform : Ljava/util/function/BiFunction;
private static final synthetic $VALUES : [Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
public static values()[Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
private <init>(Ljava/lang/String;ILjava/lang/String;Ljava/util/function/BiFunction;)V
public static getByName(Ljava/lang/String;)Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
public apply(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/Vec3;
public apply(Lnet/minecraft/commands/CommandSourceStack;)Lnet/minecraft/world/phys/Vec3;
private static synthetic $values()[Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;
private static synthetic lambda$static$2(Ljava/util/HashMap;)V
private static synthetic lambda$static$1(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/Vec3;
private static synthetic lambda$static$0(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/Vec3;
static <clinit>()V
```

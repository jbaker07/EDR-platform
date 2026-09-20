---
type: "interface"
fqcn: "net.minecraft.client.tutorial.TutorialSteps"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.tutorial.TutorialSteps

System: [[20-Systems/net.minecraft.client.tutorial|net.minecraft.client.tutorial]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `NONE` | `Lnet/minecraft/client/tutorial/TutorialSteps;` | exact | getstatic@1 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (9 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MOVEMENT : Lnet/minecraft/client/tutorial/TutorialSteps;
public static final FIND_TREE : Lnet/minecraft/client/tutorial/TutorialSteps;
public static final PUNCH_TREE : Lnet/minecraft/client/tutorial/TutorialSteps;
public static final OPEN_INVENTORY : Lnet/minecraft/client/tutorial/TutorialSteps;
public static final CRAFT_PLANKS : Lnet/minecraft/client/tutorial/TutorialSteps;
public static final NONE : Lnet/minecraft/client/tutorial/TutorialSteps;
private final name : Ljava/lang/String;
private final constructor : Ljava/util/function/Function;
private static final synthetic $VALUES : [Lnet/minecraft/client/tutorial/TutorialSteps;
public static values()[Lnet/minecraft/client/tutorial/TutorialSteps;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/tutorial/TutorialSteps;
private <init>(Ljava/lang/String;ILjava/lang/String;Ljava/util/function/Function;)V
public create(Lnet/minecraft/client/tutorial/Tutorial;)Lnet/minecraft/client/tutorial/TutorialStepInstance;
public getName()Ljava/lang/String;
public static getByName(Ljava/lang/String;)Lnet/minecraft/client/tutorial/TutorialSteps;
private static synthetic $values()[Lnet/minecraft/client/tutorial/TutorialSteps;
static <clinit>()V
```

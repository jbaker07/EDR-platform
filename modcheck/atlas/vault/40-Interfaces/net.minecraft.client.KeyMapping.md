---
type: "interface"
fqcn: "net.minecraft.client.KeyMapping"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.KeyMapping

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `java/lang/Object`; implements `java/lang/Comparable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@35 in `TestInputImpl.getBoundKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@57 in `KeyMappingRegistryImpl.registerKeyMapping` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@70 in `KeyMappingRegistryImpl.registerKeyMapping` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@74 in `KeyMappingRegistryImpl.registerKeyMapping` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `getName` | `()Ljava/lang/String;` | exact | invokevirtual@88 in `KeyMappingRegistryImpl.registerKeyMapping` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `isDown` | `()Z` | exact | invokevirtual@23 in `MinecraftMixin.injectHandleInputEventsForPreAttackCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (9 fields, 38 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final ALL : Ljava/util/Map;
private static final MAP : Ljava/util/Map;
private final name : Ljava/lang/String;
private final defaultKey : Lcom/mojang/blaze3d/platform/InputConstants$Key;
private final category : Lnet/minecraft/client/KeyMapping$Category;
protected key : Lcom/mojang/blaze3d/platform/InputConstants$Key;
private isDown : Z
private clickCount : I
private final order : I
public static click(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V
public static set(Lcom/mojang/blaze3d/platform/InputConstants$Key;Z)V
private static forAllKeyMappings(Lcom/mojang/blaze3d/platform/InputConstants$Key;Ljava/util/function/Consumer;)V
public static setAll()V
public static releaseAll()V
public static restoreToggleStatesOnScreenClosed()V
public static resetToggleKeys()V
public static resetMapping()V
public <init>(Ljava/lang/String;ILnet/minecraft/client/KeyMapping$Category;)V
public <init>(Ljava/lang/String;Lcom/mojang/blaze3d/platform/InputConstants$Type;ILnet/minecraft/client/KeyMapping$Category;)V
public <init>(Ljava/lang/String;Lcom/mojang/blaze3d/platform/InputConstants$Type;ILnet/minecraft/client/KeyMapping$Category;I)V
public isDown()Z
public getCategory()Lnet/minecraft/client/KeyMapping$Category;
public consumeClick()Z
protected release()V
protected shouldSetOnIngameFocus()Z
public getName()Ljava/lang/String;
public getDefaultKey()Lcom/mojang/blaze3d/platform/InputConstants$Key;
public setKey(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V
public compareTo(Lnet/minecraft/client/KeyMapping;)I
public static createNameSupplier(Ljava/lang/String;)Ljava/util/function/Supplier;
public same(Lnet/minecraft/client/KeyMapping;)Z
public isUnbound()Z
public matches(Lnet/minecraft/client/input/KeyEvent;)Z
public matchesMouse(Lnet/minecraft/client/input/MouseButtonEvent;)Z
public matches(Lcom/mojang/blaze3d/platform/InputConstants$Key;)Z
public getTranslatedKeyMessage()Lnet/minecraft/network/chat/Component;
public isDefault()Z
public saveString()Ljava/lang/String;
public setDown(Z)V
private registerMapping(Lcom/mojang/blaze3d/platform/InputConstants$Key;)V
public static get(Ljava/lang/String;)Lnet/minecraft/client/KeyMapping;
public synthetic compareTo(Ljava/lang/Object;)I
private static synthetic lambda$registerMapping$0(Lcom/mojang/blaze3d/platform/InputConstants$Key;)Ljava/util/List;
private static synthetic lambda$createNameSupplier$0(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$set$0(ZLnet/minecraft/client/KeyMapping;)V
private static synthetic lambda$click$0(Lnet/minecraft/client/KeyMapping;)V
static <clinit>()V
```

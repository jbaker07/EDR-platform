---
type: "interface"
fqcn: "net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer"
module: "fabric-recipe-api-v1"
sha256: "7d63b44a449ddd84d4988ef17cec1894ef1890b2844079500ea47c3dc678d967"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer

Module: [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] -- kind: interface

```java
public static void register(net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer<?>)
public static net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer<?> get(net.minecraft.resources.Identifier)
public abstract net.minecraft.resources.Identifier getIdentifier()
public abstract com.mojang.serialization.MapCodec<T> getCodec()
public abstract net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, T> getStreamCodec()
```

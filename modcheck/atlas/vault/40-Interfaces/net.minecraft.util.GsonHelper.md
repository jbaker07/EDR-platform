---
type: "interface"
fqcn: "net.minecraft.util.GsonHelper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.GsonHelper

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fromJson` | `(Lcom/google/gson/Gson;Ljava/io/Reader;Ljava/lang/Class;)Ljava/lang/Ob` | exact | invokestatic@6 in `UnbakedModelDeserializerRegistry.deserialize` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getAsBoolean` | `(Lcom/google/gson/JsonObject;Ljava/lang/String;Z)Z` | exact | invokestatic@75 in `UnbakedModelJsonDeserializer.deserialize` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getAsString` | `(Lcom/google/gson/JsonObject;Ljava/lang/String;)Ljava/lang/String;` | exact | invokestatic@65 in `UnbakedModelJsonDeserializer.deserialize` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getType` | `(Lcom/google/gson/JsonElement;)Ljava/lang/String;` | exact | invokestatic@89 in `UnbakedModelJsonDeserializer.deserialize` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (1 fields, 73 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final GSON : Lcom/google/gson/Gson;
public <init>()V
public static isStringValue(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static isStringValue(Lcom/google/gson/JsonElement;)Z
public static isNumberValue(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static isNumberValue(Lcom/google/gson/JsonElement;)Z
public static isBooleanValue(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static isBooleanValue(Lcom/google/gson/JsonElement;)Z
public static isArrayNode(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static isObjectNode(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static isValidPrimitive(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static isValidNode(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static getNonNull(Lcom/google/gson/JsonObject;Ljava/lang/String;)Lcom/google/gson/JsonElement;
public static convertToString(Lcom/google/gson/JsonElement;Ljava/lang/String;)Ljava/lang/String;
public static getAsString(Lcom/google/gson/JsonObject;Ljava/lang/String;)Ljava/lang/String;
public static getAsString(Lcom/google/gson/JsonObject;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public static convertToItem(Lcom/google/gson/JsonElement;Ljava/lang/String;)Lnet/minecraft/core/Holder;
public static getAsItem(Lcom/google/gson/JsonObject;Ljava/lang/String;)Lnet/minecraft/core/Holder;
public static getAsItem(Lcom/google/gson/JsonObject;Ljava/lang/String;Lnet/minecraft/core/Holder;)Lnet/minecraft/core/Holder;
public static convertToBoolean(Lcom/google/gson/JsonElement;Ljava/lang/String;)Z
public static getAsBoolean(Lcom/google/gson/JsonObject;Ljava/lang/String;)Z
public static getAsBoolean(Lcom/google/gson/JsonObject;Ljava/lang/String;Z)Z
public static convertToDouble(Lcom/google/gson/JsonElement;Ljava/lang/String;)D
public static getAsDouble(Lcom/google/gson/JsonObject;Ljava/lang/String;)D
public static getAsDouble(Lcom/google/gson/JsonObject;Ljava/lang/String;D)D
public static convertToFloat(Lcom/google/gson/JsonElement;Ljava/lang/String;)F
public static getAsFloat(Lcom/google/gson/JsonObject;Ljava/lang/String;)F
public static getAsFloat(Lcom/google/gson/JsonObject;Ljava/lang/String;F)F
public static convertToLong(Lcom/google/gson/JsonElement;Ljava/lang/String;)J
public static getAsLong(Lcom/google/gson/JsonObject;Ljava/lang/String;)J
public static getAsLong(Lcom/google/gson/JsonObject;Ljava/lang/String;J)J
public static convertToInt(Lcom/google/gson/JsonElement;Ljava/lang/String;)I
public static getAsInt(Lcom/google/gson/JsonObject;Ljava/lang/String;)I
public static getAsInt(Lcom/google/gson/JsonObject;Ljava/lang/String;I)I
public static convertToByte(Lcom/google/gson/JsonElement;Ljava/lang/String;)B
public static getAsByte(Lcom/google/gson/JsonObject;Ljava/lang/String;)B
public static getAsByte(Lcom/google/gson/JsonObject;Ljava/lang/String;B)B
public static convertToCharacter(Lcom/google/gson/JsonElement;Ljava/lang/String;)C
public static getAsCharacter(Lcom/google/gson/JsonObject;Ljava/lang/String;)C
public static getAsCharacter(Lcom/google/gson/JsonObject;Ljava/lang/String;C)C
public static convertToBigDecimal(Lcom/google/gson/JsonElement;Ljava/lang/String;)Ljava/math/BigDecimal;
public static getAsBigDecimal(Lcom/google/gson/JsonObject;Ljava/lang/String;)Ljava/math/BigDecimal;
public static getAsBigDecimal(Lcom/google/gson/JsonObject;Ljava/lang/String;Ljava/math/BigDecimal;)Ljava/math/BigDecimal;
public static convertToBigInteger(Lcom/google/gson/JsonElement;Ljava/lang/String;)Ljava/math/BigInteger;
public static getAsBigInteger(Lcom/google/gson/JsonObject;Ljava/lang/String;)Ljava/math/BigInteger;
public static getAsBigInteger(Lcom/google/gson/JsonObject;Ljava/lang/String;Ljava/math/BigInteger;)Ljava/math/BigInteger;
public static convertToShort(Lcom/google/gson/JsonElement;Ljava/lang/String;)S
public static getAsShort(Lcom/google/gson/JsonObject;Ljava/lang/String;)S
public static getAsShort(Lcom/google/gson/JsonObject;Ljava/lang/String;S)S
public static convertToJsonObject(Lcom/google/gson/JsonElement;Ljava/lang/String;)Lcom/google/gson/JsonObject;
public static getAsJsonObject(Lcom/google/gson/JsonObject;Ljava/lang/String;)Lcom/google/gson/JsonObject;
public static getAsJsonObject(Lcom/google/gson/JsonObject;Ljava/lang/String;Lcom/google/gson/JsonObject;)Lcom/google/gson/JsonObject;
public static convertToJsonArray(Lcom/google/gson/JsonElement;Ljava/lang/String;)Lcom/google/gson/JsonArray;
public static getAsJsonArray(Lcom/google/gson/JsonObject;Ljava/lang/String;)Lcom/google/gson/JsonArray;
public static getAsJsonArray(Lcom/google/gson/JsonObject;Ljava/lang/String;Lcom/google/gson/JsonArray;)Lcom/google/gson/JsonArray;
public static convertToObject(Lcom/google/gson/JsonElement;Ljava/lang/String;Lcom/google/gson/JsonDeserializationContext;Ljava/lang/Class;)Ljava/lang/Object;
public static getAsObject(Lcom/google/gson/JsonObject;Ljava/lang/String;Lcom/google/gson/JsonDeserializationContext;Ljava/lang/Class;)Ljava/lang/Object;
public static getAsObject(Lcom/google/gson/JsonObject;Ljava/lang/String;Ljava/lang/Object;Lcom/google/gson/JsonDeserializationContext;Ljava/lang/Class;)Ljava/lang/Object;
public static getType(Lcom/google/gson/JsonElement;)Ljava/lang/String;
public static fromJson(Lcom/google/gson/Gson;Ljava/io/Reader;Ljava/lang/Class;)Ljava/lang/Object;
public static fromNullableJson(Lcom/google/gson/Gson;Ljava/io/Reader;Lcom/google/gson/reflect/TypeToken;)Ljava/lang/Object;
public static fromJson(Lcom/google/gson/Gson;Ljava/io/Reader;Lcom/google/gson/reflect/TypeToken;)Ljava/lang/Object;
public static fromNullableJson(Lcom/google/gson/Gson;Ljava/lang/String;Lcom/google/gson/reflect/TypeToken;)Ljava/lang/Object;
public static fromJson(Lcom/google/gson/Gson;Ljava/lang/String;Ljava/lang/Class;)Ljava/lang/Object;
public static parse(Ljava/lang/String;)Lcom/google/gson/JsonObject;
public static parse(Ljava/io/Reader;)Lcom/google/gson/JsonObject;
public static parseArray(Ljava/lang/String;)Lcom/google/gson/JsonArray;
public static parseArray(Ljava/io/Reader;)Lcom/google/gson/JsonArray;
public static toStableString(Lcom/google/gson/JsonElement;)Ljava/lang/String;
public static writeValue(Lcom/google/gson/stream/JsonWriter;Lcom/google/gson/JsonElement;Ljava/util/Comparator;)V
private static sortByKeyIfNeeded(Ljava/util/Collection;Ljava/util/Comparator;)Ljava/util/Collection;
public static encodesLongerThan(Lcom/google/gson/JsonElement;I)Z
private static synthetic lambda$convertToItem$0(Ljava/lang/String;Ljava/lang/String;)Lcom/google/gson/JsonSyntaxException;
static <clinit>()V
```

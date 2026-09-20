---
type: "interface"
fqcn: "net.minecraft.util.GsonHelper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.GsonHelper

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fromJson(Lcom/google/gson/Gson;Ljava/io/Reader;Ljava/lang/Class;)Lja` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getAsBoolean(Lcom/google/gson/JsonObject;Ljava/lang/String;Z)Z` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getAsString(Lcom/google/gson/JsonObject;Ljava/lang/String;)Ljava/lang/S` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getType(Lcom/google/gson/JsonElement;)Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (74, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.GsonHelper {
    private static final com.google.gson.Gson GSON;
    public net.minecraft.util.GsonHelper();
    public static boolean isStringValue(com.google.gson.JsonObject, java.lang.String);
    public static boolean isStringValue(com.google.gson.JsonElement);
    public static boolean isNumberValue(com.google.gson.JsonObject, java.lang.String);
    public static boolean isNumberValue(com.google.gson.JsonElement);
    public static boolean isBooleanValue(com.google.gson.JsonObject, java.lang.String);
    public static boolean isBooleanValue(com.google.gson.JsonElement);
    public static boolean isArrayNode(com.google.gson.JsonObject, java.lang.String);
    public static boolean isObjectNode(com.google.gson.JsonObject, java.lang.String);
    public static boolean isValidPrimitive(com.google.gson.JsonObject, java.lang.String);
    public static boolean isValidNode(com.google.gson.JsonObject, java.lang.String);
    public static com.google.gson.JsonElement getNonNull(com.google.gson.JsonObject, java.lang.String);
    public static java.lang.String convertToString(com.google.gson.JsonElement, java.lang.String);
    public static java.lang.String getAsString(com.google.gson.JsonObject, java.lang.String);
    public static java.lang.String getAsString(com.google.gson.JsonObject, java.lang.String, java.lang.String);
    public static net.minecraft.core.Holder<net.minecraft.world.item.Item> convertToItem(com.google.gson.JsonElement, java.lang.String);
    public static net.minecraft.core.Holder<net.minecraft.world.item.Item> getAsItem(com.google.gson.JsonObject, java.lang.String);
    public static net.minecraft.core.Holder<net.minecraft.world.item.Item> getAsItem(com.google.gson.JsonObject, java.lang.String, net.minecraft.core.Holder<net.minecraft.world.item.Item>);
    public static boolean convertToBoolean(com.google.gson.JsonElement, java.lang.String);
    public static boolean getAsBoolean(com.google.gson.JsonObject, java.lang.String);
    public static boolean getAsBoolean(com.google.gson.JsonObject, java.lang.String, boolean);
    public static double convertToDouble(com.google.gson.JsonElement, java.lang.String);
    public static double getAsDouble(com.google.gson.JsonObject, java.lang.String);
    public static double getAsDouble(com.google.gson.JsonObject, java.lang.String, double);
    public static float convertToFloat(com.google.gson.JsonElement, java.lang.String);
    public static float getAsFloat(com.google.gson.JsonObject, java.lang.String);
    public static float getAsFloat(com.google.gson.JsonObject, java.lang.String, float);
    public static long convertToLong(com.google.gson.JsonElement, java.lang.String);
    public static long getAsLong(com.google.gson.JsonObject, java.lang.String);
    public static long getAsLong(com.google.gson.JsonObject, java.lang.String, long);
    public static int convertToInt(com.google.gson.JsonElement, java.lang.String);
    public static int getAsInt(com.google.gson.JsonObject, java.lang.String);
    public static int getAsInt(com.google.gson.JsonObject, java.lang.String, int);
    public static byte convertToByte(com.google.gson.JsonElement, java.lang.String);
    public static byte getAsByte(com.google.gson.JsonObject, java.lang.String);
    public static byte getAsByte(com.google.gson.JsonObject, java.lang.String, byte);
    public static char convertToCharacter(com.google.gson.JsonElement, java.lang.String);
    public static char getAsCharacter(com.google.gson.JsonObject, java.lang.String);
    public static char getAsCharacter(com.google.gson.JsonObject, java.lang.String, char);
    public static java.math.BigDecimal convertToBigDecimal(com.google.gson.JsonElement, java.lang.String);
    public static java.math.BigDecimal getAsBigDecimal(com.google.gson.JsonObject, java.lang.String);
    public static java.math.BigDecimal getAsBigDecimal(com.google.gson.JsonObject, java.lang.String, java.math.BigDecimal);
    public static java.math.BigInteger convertToBigInteger(com.google.gson.JsonElement, java.lang.String);
    public static java.math.BigInteger getAsBigInteger(com.google.gson.JsonObject, java.lang.String);
    public static java.math.BigInteger getAsBigInteger(com.google.gson.JsonObject, java.lang.String, java.math.BigInteger);
    public static short convertToShort(com.google.gson.JsonElement, java.lang.String);
    public static short getAsShort(com.google.gson.JsonObject, java.lang.String);
    public static short getAsShort(com.google.gson.JsonObject, java.lang.String, short);
    public static com.google.gson.JsonObject convertToJsonObject(com.google.gson.JsonElement, java.lang.String);
    public static com.google.gson.JsonObject getAsJsonObject(com.google.gson.JsonObject, java.lang.String);
    public static com.google.gson.JsonObject getAsJsonObject(com.google.gson.JsonObject, java.lang.String, com.google.gson.JsonObject);
    public static com.google.gson.JsonArray convertToJsonArray(com.google.gson.JsonElement, java.lang.String);
    public static com.google.gson.JsonArray getAsJsonArray(com.google.gson.JsonObject, java.lang.String);
    public static com.google.gson.JsonArray getAsJsonArray(com.google.gson.JsonObject, java.lang.String, com.google.gson.JsonArray);
    public static <T> T convertToObject(com.google.gson.JsonElement, java.lang.String, com.google.gson.JsonDeserializationContext, java.lang.Class<? extends T>);
    public static <T> T getAsObject(com.google.gson.JsonObject, java.lang.String, com.google.gson.JsonDeserializationContext, java.lang.Class<? extends T>);
    public static <T> T getAsObject(com.google.gson.JsonObject, java.lang.String, T, com.google.gson.JsonDeserializationContext, java.lang.Class<? extends T>);
    public static java.lang.String getType(com.google.gson.JsonElement);
    public static <T> T fromJson(com.google.gson.Gson, java.io.Reader, java.lang.Class<T>);
    public static <T> T fromNullableJson(com.google.gson.Gson, java.io.Reader, com.google.gson.reflect.TypeToken<T>);
    public static <T> T fromJson(com.google.gson.Gson, java.io.Reader, com.google.gson.reflect.TypeToken<T>);
    public static <T> T fromNullableJson(com.google.gson.Gson, java.lang.String, com.google.gson.reflect.TypeToken<T>);
    public static <T> T fromJson(com.google.gson.Gson, java.lang.String, java.lang.Class<T>);
    public static com.google.gson.JsonObject parse(java.lang.String);
    public static com.google.gson.JsonObject parse(java.io.Reader);
    public static com.google.gson.JsonArray parseArray(java.lang.String);
    public static com.google.gson.JsonArray parseArray(java.io.Reader);
    public static java.lang.String toStableString(com.google.gson.JsonElement);
    public static void writeValue(com.google.gson.stream.JsonWriter, com.google.gson.JsonElement, java.util.Comparator<java.lang.String>) throws java.io.IOException;
    private static java.util.Collection<java.util.Map$Entry<java.lang.String, com.google.gson.JsonElement>> sortByKeyIfNeeded(java.util.Collection<java.util.Map$Entry<java.lang.String, com.google.gson.JsonElement>>, java.util.Comparator<java.lang.String>);
    public static boolean encodesLongerThan(com.google.gson.JsonElement, int);
    private static com.google.gson.JsonSyntaxException lambda$convertToItem$0(java.lang.String, java.lang.String);
    static {};
}
```

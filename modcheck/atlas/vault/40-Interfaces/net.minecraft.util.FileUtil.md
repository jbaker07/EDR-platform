---
type: "interface"
fqcn: "net.minecraft.util.FileUtil"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.FileUtil

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `validatePath([Ljava/lang/String;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.FileUtil {
    private static final java.util.regex.Pattern COPY_COUNTER_PATTERN;
    private static final int MAX_FILE_NAME;
    private static final java.util.regex.Pattern RESERVED_WINDOWS_FILENAMES;
    private static final java.util.regex.Pattern STRICT_PATH_SEGMENT_CHECK;
    public net.minecraft.util.FileUtil();
    public static java.lang.String sanitizeName(java.lang.String);
    public static java.lang.String findAvailableName(java.nio.file.Path, java.lang.String, java.lang.String) throws java.io.IOException;
    public static boolean isPathPortable(java.nio.file.Path);
    public static boolean isPathPartPortable(java.lang.String);
    public static java.lang.String getFullResourcePath(java.lang.String);
    public static java.lang.String normalizeResourcePath(java.lang.String);
    public static com.mojang.serialization.DataResult<java.util.List<java.lang.String>> decomposePath(java.lang.String);
    public static java.nio.file.Path resolvePath(java.nio.file.Path, java.util.List<java.lang.String>);
    private static boolean containsAllowedCharactersOnly(java.lang.String);
    public static boolean isValidPathSegment(java.lang.String);
    public static void validatePath(java.lang.String...);
    public static void createDirectoriesSafe(java.nio.file.Path) throws java.io.IOException;
    public static boolean isEmptyPath(java.nio.file.Path);
    private static java.lang.String lambda$decomposePath$3(java.lang.String, java.lang.String);
    private static java.lang.String lambda$decomposePath$2(java.lang.String, java.lang.String);
    private static java.lang.String lambda$decomposePath$1(java.lang.String);
    private static java.lang.String lambda$decomposePath$0(java.lang.String);
    static {};
}
```

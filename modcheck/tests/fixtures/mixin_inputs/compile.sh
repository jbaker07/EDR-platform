#!/bin/sh
# Compile the fixture inputs independently of any corpus jar's own build.
# Only annotation jars are on the classpath; nothing from Minecraft.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
JAVAC="$ROOT/toolchains/jdk-25.0.4.1+1/bin/javac"
C=/root/.gradle/caches/modules-2/files-2.1
CP="$(ls $C/net.fabricmc/sponge-mixin/0.17.4+mixin.0.8.7/*/sponge-mixin-0.17.4+mixin.0.8.7.jar)"
CP="$CP:$(ls $C/io.github.llamalad7/mixinextras-fabric/0.5.5/*/mixinextras-fabric-0.5.5.jar)"
CP="$CP:$(ls $C/net.fabricmc/fabric-loader/0.19.5/*/fabric-loader-0.19.5.jar)"
CP="$CP:$(ls $C/net.fabricmc.fabric-api/fabric-api-base/*/*/fabric-api-base-*.jar | grep -v sources | head -1)"
rm -rf "$HERE/classes"; mkdir -p "$HERE/classes"
JAVA_TOOL_OPTIONS= "$JAVAC" --release 21 -proc:none -Xlint:-options -cp "$CP" -d "$HERE/classes" $(find "$HERE/src" -name "*.java")
find "$HERE/classes" -name "*.class" | sort

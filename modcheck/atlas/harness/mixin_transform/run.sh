#!/bin/sh
# Compile the harness and the controlled scenarios, then run the pinned Mixin
# transformer over them. Prints the output JSON path. Mixin, MixinExtras and ASM
# come from the Gradle cache at the pinned versions; nothing from Minecraft.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
JDK="$ROOT/toolchains/jdk-25.0.4.1+1/bin"
C=/root/.gradle/caches/modules-2/files-2.1
MIXIN=$(ls $C/net.fabricmc/sponge-mixin/0.17.4+mixin.0.8.7/*/sponge-mixin-0.17.4+mixin.0.8.7.jar)
EXTRAS=$(ls $C/io.github.llamalad7/mixinextras-fabric/0.5.5/*/mixinextras-fabric-0.5.5.jar)
ASM=$(ls $C/org.ow2.asm/asm/9.10.1/*/asm-9.10.1.jar):$(ls $C/org.ow2.asm/asm-tree/9.10.1/*/asm-tree-9.10.1.jar):$(ls $C/org.ow2.asm/asm-commons/9.10.1/*/asm-commons-9.10.1.jar):$(ls $C/org.ow2.asm/asm-util/9.10.1/*/asm-util-9.10.1.jar):$(ls $C/org.ow2.asm/asm-analysis/9.10.1/*/asm-analysis-9.10.1.jar)
LIBS="$MIXIN:$EXTRAS:$ASM"
export JAVA_TOOL_OPTIONS=
rm -rf "$HERE/build"; mkdir -p "$HERE/build/harness" "$HERE/build/fixtures"
"$JDK/javac" --release 17 -proc:none -Xlint:-options -cp "$LIBS" -d "$HERE/build/harness" $(find "$HERE/src/harness" -name "*.java")
cp -r "$HERE/resources/META-INF" "$HERE/build/harness/"
"$JDK/javac" --release 17 -proc:none -Xlint:-options -cp "$LIBS:$HERE/build/harness" -d "$HERE/build/fixtures" $(find "$HERE/src/fixtures" -name "*.java")
cp "$HERE"/resources/*.mixins.json "$HERE/build/fixtures/"
OUT="${1:-$HERE/build/result.json}"
"$JDK/java" -Dmixin.service=harness.HarnessService -Dmixin.bootstrapService=harness.HarnessBootstrap \
  -Dharness.classdirs="$HERE/build/fixtures" -Dmixin.env.disableRefMap=true \
  -cp "$HERE/build/harness:$LIBS" harness.Main "$OUT" \
  fixtures.targets.A fixtures.targets.B fixtures.targets.B2 fixtures.targets.C fixtures.targets.D \
  fixtures.targets.E fixtures.targets.F fixtures.targets.G fixtures.targets.H fixtures.targets.J \
  fixtures.targets.K fixtures.targets.L fixtures.targets.M fixtures.targets.N
echo "sha256 $MIXIN $(sha256sum "$MIXIN" | cut -c1-64)"
echo "sha256 $EXTRAS $(sha256sum "$EXTRAS" | cut -c1-64)"

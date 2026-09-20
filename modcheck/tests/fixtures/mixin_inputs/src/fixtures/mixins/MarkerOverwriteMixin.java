package fixtures.mixins;

import fixtures.target.Target;
import org.spongepowered.asm.mixin.Final;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.Overwrite;
import org.spongepowered.asm.mixin.Shadow;
import org.spongepowered.asm.mixin.Unique;

@Mixin(Target.class)
public abstract class MarkerOverwriteMixin {
    @Shadow @Final public static int counter;
    @Shadow private int value;
    @Shadow protected abstract void helper();
    @Unique private int extra;

    /** Marker annotation: no arguments at all. */
    @Overwrite
    public int compute(int x) { return x + extra; }
}

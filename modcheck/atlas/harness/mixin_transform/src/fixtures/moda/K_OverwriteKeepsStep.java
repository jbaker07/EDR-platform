package fixtures.moda;
import fixtures.targets.K; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite; import org.spongepowered.asm.mixin.Shadow;
@Mixin(K.class) public abstract class K_OverwriteKeepsStep {
    @Shadow public abstract void step();
    @Overwrite public void run() { Trace.log("K1.overwrite.calls_step"); step(); }
}

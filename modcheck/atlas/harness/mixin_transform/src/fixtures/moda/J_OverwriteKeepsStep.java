package fixtures.moda;
import fixtures.targets.J; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite; import org.spongepowered.asm.mixin.Shadow;
@Mixin(J.class) public abstract class J_OverwriteKeepsStep {
    @Shadow public abstract void step();
    @Overwrite public void run() { Trace.log("J1.overwrite.calls_step"); step(); }
}

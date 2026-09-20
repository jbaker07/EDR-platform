package fixtures.moda;
import fixtures.targets.F; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(F.class) public class F_Overwrite { @Overwrite public void run() { Trace.log("F1.overwrite"); } }

package fixtures.moda;
import fixtures.targets.L; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(L.class) public class L_Overwrite { @Overwrite public void run() { Trace.log("L1.overwrite"); } }

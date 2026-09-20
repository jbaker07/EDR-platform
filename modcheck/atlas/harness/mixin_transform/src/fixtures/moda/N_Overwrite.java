package fixtures.moda;
import fixtures.targets.N; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(N.class) public class N_Overwrite { @Overwrite public void run() { Trace.log("N1.overwrite"); } }

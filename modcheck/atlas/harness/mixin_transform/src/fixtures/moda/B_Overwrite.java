package fixtures.moda;
import fixtures.targets.B; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(B.class) public class B_Overwrite { @Overwrite public void run() { Trace.log("B1.overwrite.no_step"); } }

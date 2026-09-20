package fixtures.moda;
import fixtures.targets.B2; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(B2.class) public class B2_Overwrite { @Overwrite public void run() { Trace.log("B2_1.overwrite.no_step"); } }

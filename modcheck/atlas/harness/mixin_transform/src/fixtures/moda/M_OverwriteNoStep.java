package fixtures.moda;
import fixtures.targets.M; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(M.class) public class M_OverwriteNoStep { @Overwrite public void run() { Trace.log("M1.overwrite.no_step"); } }

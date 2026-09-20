package fixtures.modb;
import fixtures.targets.F; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(F.class) public class F_Overwrite2 { @Overwrite public void run() { Trace.log("F2.overwrite"); } }

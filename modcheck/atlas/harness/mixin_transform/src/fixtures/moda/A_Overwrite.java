package fixtures.moda;
import fixtures.targets.A; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.Overwrite;
@Mixin(A.class) public class A_Overwrite { @Overwrite public void run() { Trace.log("A1.overwrite"); } }

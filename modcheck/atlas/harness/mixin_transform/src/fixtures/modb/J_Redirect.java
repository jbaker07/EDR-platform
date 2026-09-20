package fixtures.modb;
import fixtures.targets.J; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Redirect;
@Mixin(J.class) public class J_Redirect {
    @Redirect(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/J;step()V")) private void redirect(J self) { Trace.log("J2.redirect"); self.step(); }
}

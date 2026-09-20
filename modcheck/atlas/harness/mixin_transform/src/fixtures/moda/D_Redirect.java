package fixtures.moda;
import fixtures.targets.D; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Redirect;
@Mixin(D.class) public class D_Redirect {
    @Redirect(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/D;step()V")) private void redirect(D self) { Trace.log("D1.redirect"); self.step(); }
}

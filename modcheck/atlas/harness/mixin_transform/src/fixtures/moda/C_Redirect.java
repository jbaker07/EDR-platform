package fixtures.moda;
import fixtures.targets.C; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Redirect;
@Mixin(C.class) public class C_Redirect {
    @Redirect(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/C;step()V")) private void redirect(C self) { Trace.log("C1.redirect"); self.step(); }
}

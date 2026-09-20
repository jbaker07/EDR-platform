package fixtures.modb;
import fixtures.targets.C; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Redirect;
@Mixin(C.class) public class C_Redirect2 {
    @Redirect(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/C;step()V")) private void redirect(C self) { Trace.log("C2.redirect"); self.step(); }
}

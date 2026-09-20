package fixtures.modb;
import fixtures.targets.M; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(value = M.class, priority = 1100) public class M_InjectInvoke1100 {
    @Inject(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/M;step()V")) private void atStep(CallbackInfo ci) { Trace.log("M2.at_step.p1100"); }
}

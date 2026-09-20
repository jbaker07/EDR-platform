package fixtures.modb;
import fixtures.targets.K; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(value = K.class, priority = 1100) public class K_InjectInvoke1100 {
    @Inject(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/K;step()V")) private void atStep(CallbackInfo ci) { Trace.log("K2.at_step.p1100"); }
}

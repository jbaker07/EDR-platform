package fixtures.modb;
import fixtures.targets.B; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(B.class) public class B_InjectInvoke {
    @Inject(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/B;step()V"), require = 1) private void atStep(CallbackInfo ci) { Trace.log("B2.at_step"); }
}

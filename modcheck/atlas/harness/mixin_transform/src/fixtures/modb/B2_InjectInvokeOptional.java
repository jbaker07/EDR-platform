package fixtures.modb;
import fixtures.targets.B2; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(B2.class) public class B2_InjectInvokeOptional {
    @Inject(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/B2;step()V"), require = 0, expect = 0) private void atStep(CallbackInfo ci) { Trace.log("B2_2.at_step"); }
}

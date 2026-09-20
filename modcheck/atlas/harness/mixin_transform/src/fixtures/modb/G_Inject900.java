package fixtures.modb;
import fixtures.targets.G; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At; import org.spongepowered.asm.mixin.injection.Inject; import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
@Mixin(value = G.class, priority = 900) public class G_Inject900 {
    @Inject(method = "run", at = @At("HEAD")) private void head(CallbackInfo ci) { Trace.log("G2.head.p900"); }
}

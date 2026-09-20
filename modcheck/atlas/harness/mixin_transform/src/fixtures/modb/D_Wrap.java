package fixtures.modb;
import com.llamalad7.mixinextras.injector.wrapoperation.Operation; import com.llamalad7.mixinextras.injector.wrapoperation.WrapOperation;
import fixtures.targets.D; import harness.Trace; import org.spongepowered.asm.mixin.Mixin; import org.spongepowered.asm.mixin.injection.At;
@Mixin(D.class) public class D_Wrap {
    @WrapOperation(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/targets/D;step()V")) private void wrap(D self, Operation<Void> original) { Trace.log("D2.wrap.before"); original.call(self); Trace.log("D2.wrap.after"); }
}

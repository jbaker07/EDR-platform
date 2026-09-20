---
type: "evidence"
evidence_class: "executed_transformation"
mixin: "0.8.7"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Mixin transformation tests

**Evidence class `executed_transformation`.** The pinned Mixin transformer run in a plain JVM over independently compiled target and mixin classes (two mixin configs standing in for two mods). Not a Minecraft run: no game class, no fabric-loader, no transforming classloader.

- sponge-mixin `1f0ae44db7295f86`, MixinExtras `5da883dc4bfb16e4`, Java 25.0.4.1
- harness: `atlas/harness/mixin_transform`; configs `moda.mixins.json`, `modb.mixins.json` with `defaultRequire` 1

## Static checks on the pinned jar

- `InjectionPoint.checkPriority` default bytecode: `iload_1 iload_2 if_icmpge iconst_1 goto iconst_0 ireturn` -- returns true iff targetPriority < mixinPriority (iload_1, iload_2, if_icmpge)
- overriders returning true unconditionally: MethodHead, BeforeReturn, BeforeFinalReturn; not overriding: BeforeInvoke, AfterInvoke, BeforeFieldAccess, BeforeNew, BeforeConstant, JumpInsnPoint
- applicator pass order: MAIN -> INJECT_PREPARE -> INITIALISER_APPLY_LEGACY -> INJECT_PREPARE_LEGACY -> INITIALISER_APPLY -> ACCESSOR -> INJECT_PREINJECT -> INJECT_APPLY
- `@Mixin` defaults: `{"value": [], "targets": [], "priority": 1000, "remap": true}`

## Scenarios

## A

**Question.** Does an @Overwrite from one mod remove a HEAD/TAIL @Inject on the same method from another mod (equal priority 1000)?

**Setup.** moda A_Overwrite: @Overwrite run(); modb A_Inject: @Inject HEAD and TAIL into run()

- transformed: True; bytes changed: True
- applied (audit): A_Overwrite, A_Inject
- trace: `A2.head -> A1.overwrite -> A2.tail`

## B

**Question.** Does an INVOKE-point @Inject (require=1) into a method overwritten by an equal-priority mod apply?

**Setup.** moda B_Overwrite (body no longer calls step); modb B_InjectInvoke at INVOKE step(), require=1

- transformed: False; bytes changed: False
- applied (audit): B_Overwrite, B_InjectInvoke
- transformation error: `org.spongepowered.asm.mixin.injection.throwables.InvalidInjectionException: @At("INVOKE") on fixtures/targets/B::atStep with priority 1000 cannot inject into fixtures/targets/B::run()V merged by fixtures.moda.B_Overwrite with priority 1000 [INJECT_PREPARE Applicator Phase -> modb.mixins.json:B_InjectInvoke from mod (unknown) -> Prepare Injections -> handler$zzg000$atStep(Lorg/spongepowered/asm/mix`
- trace: `(not run)`

## B2

**Question.** Same as B with require=0, expect=0: is the failure a target-count failure or a validation refusal?

**Setup.** moda B2_Overwrite; modb B2_InjectInvokeOptional at INVOKE step(), require=0, expect=0

- transformed: False; bytes changed: False
- applied (audit): B2_Overwrite, B2_InjectInvokeOptional
- transformation error: `org.spongepowered.asm.mixin.injection.throwables.InvalidInjectionException: @At("INVOKE") on fixtures/targets/B2::atStep with priority 1000 cannot inject into fixtures/targets/B2::run()V merged by fixtures.moda.B2_Overwrite with priority 1000 [INJECT_PREPARE Applicator Phase -> modb.mixins.json:B2_InjectInvokeOptional from mod (unknown) -> Prepare Injections -> handler$zzh001$atStep(Lorg/spongepow`
- trace: `(not run)`

## C

**Question.** Do two @Redirects of the same call site from two mods both apply?

**Setup.** moda C_Redirect and modb C_Redirect2 both redirect INVOKE step() in run()

- transformed: False; bytes changed: False
- applied (audit): C_Redirect, C_Redirect2
- transformation error: `org.spongepowered.asm.mixin.injection.throwables.InjectionError: Critical injection failure: Redirector redirect(Lfixtures/targets/C;)V in modb.mixins.json:C_Redirect2 from mod (unknown) failed injection check, (0/1) succeeded. Scanned 0 target(s). No refMap loaded.`
- trace: `(not run)`

## D

**Question.** Does a MixinExtras @WrapOperation compose with another mod's @Redirect of the same call?

**Setup.** moda D_Redirect; modb D_Wrap (@WrapOperation) on INVOKE step()

- transformed: True; bytes changed: True
- applied (audit): D_Redirect, D_Wrap
- trace: `D.run -> D2.wrap.before -> D1.redirect -> D.step -> D2.wrap.after`

## E

**Question.** In what order do equal-priority HEAD and TAIL injections from two mods run?

**Setup.** moda E_Inject and modb E_Inject2, both HEAD+TAIL, priority 1000, configs added moda then modb

- transformed: True; bytes changed: True
- applied (audit): E_Inject, E_Inject2
- trace: `E1.head -> E2.head -> E.run -> E.step -> E1.tail -> E2.tail`

## F

**Question.** What happens when two mods @Overwrite the same method at equal priority?

**Setup.** moda F_Overwrite and modb F_Overwrite2

- transformed: True; bytes changed: True
- applied (audit): F_Overwrite, F_Overwrite2
- trace: `F1.overwrite`

## G

**Question.** Does mixin priority order HEAD injections from two mods?

**Setup.** moda G_Inject1100 (priority 1100) and modb G_Inject900 (priority 900), both HEAD

- transformed: True; bytes changed: True
- applied (audit): G_Inject900, G_Inject1100
- trace: `G2.head.p900 -> G1.head.p1100 -> G.run -> G.step`

## H

**Question.** Does a cancelling HEAD injection from one mod suppress another mod's TAIL and RETURN injections?

**Setup.** moda H_Cancel: HEAD cancellable, ci.cancel(); modb H_TailReturn: TAIL and RETURN

- transformed: True; bytes changed: True
- applied (audit): H_Cancel, H_TailReturn
- trace: `H1.head.cancel`

## J

**Question.** Does a @Redirect into a method overwritten by an equal-priority mod apply when the overwritten body still contains the call?

**Setup.** moda J_OverwriteKeepsStep: @Overwrite run() calling step(); modb J_Redirect on INVOKE step()

- transformed: False; bytes changed: False
- applied (audit): J_OverwriteKeepsStep, J_Redirect
- transformation error: `org.spongepowered.asm.mixin.injection.throwables.InvalidInjectionException: @At("INVOKE") on fixtures/targets/J::redirect with priority 1000 cannot inject into fixtures/targets/J::run()V merged by fixtures.moda.J_OverwriteKeepsStep with priority 1000 [INJECT_PREPARE Applicator Phase -> modb.mixins.json:J_Redirect from mod (unknown) -> Prepare Injections -> redirect$zzn000$redirect(Lfixtures/target`
- trace: `(not run)`

## K

**Question.** Does an INVOKE-point @Inject with HIGHER priority than the overwriting mixin apply to the overwritten body?

**Setup.** moda K_OverwriteKeepsStep (1000); modb K_InjectInvoke1100 (1100) at INVOKE step()

- transformed: True; bytes changed: True
- applied (audit): K_OverwriteKeepsStep, K_InjectInvoke1100
- trace: `K1.overwrite.calls_step -> K2.at_step.p1100 -> K.step`

## L

**Question.** Do HEAD/TAIL injections with LOWER priority than the overwriting mixin apply?

**Setup.** moda L_Overwrite (1000); modb L_InjectHead900 (900) HEAD and TAIL

- transformed: True; bytes changed: True
- applied (audit): L_InjectHead900, L_Overwrite
- trace: `L2.head.p900 -> L1.overwrite -> L2.tail.p900`

## M

**Question.** With higher priority, what happens when the overwritten body no longer contains the INVOKE target?

**Setup.** moda M_OverwriteNoStep (1000); modb M_InjectInvoke1100 (1100) at INVOKE step(), require=1

- transformed: False; bytes changed: False
- applied (audit): M_OverwriteNoStep, M_InjectInvoke1100
- transformation error: `org.spongepowered.asm.mixin.injection.throwables.InjectionError: Critical injection failure: Callback method atStep(Lorg/spongepowered/asm/mixin/injection/callback/CallbackInfo;)V in modb.mixins.json:M_InjectInvoke1100 from mod (unknown) failed injection check, (0/1) succeeded. Scanned 0 target(s). No refMap loaded.`
- trace: `(not run)`

## N

**Question.** Does a HEAD injection with higher priority than the overwrite apply?

**Setup.** moda N_Overwrite (1000); modb N_InjectHead1100 (1100) HEAD

- transformed: True; bytes changed: True
- applied (audit): N_Overwrite, N_InjectHead1100
- trace: `N2.head.p1100 -> N1.overwrite`

## Not established here

- Anything about a running Minecraft: no game class, no loader, no Knot classloader was involved.
- Rules for injector pairs not in the scenarios.

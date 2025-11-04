; ModuleID = 'main.aa1001188fddf649-cgu.0'
source_filename = "main.aa1001188fddf649-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"core::fmt::rt::Argument<'_>" = type { %"core::fmt::rt::ArgumentType<'_>" }
%"core::fmt::rt::ArgumentType<'_>" = type { ptr, [1 x i64] }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h53e7a5429aeb5b8dE", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E", ptr @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E" }>, align 8
@anon.1a8ca7ad941f0113fec942780bd214ec.0 = private unnamed_addr constant <{ [8 x i8], [8 x i8] }> <{ [8 x i8] zeroinitializer, [8 x i8] undef }>, align 8
@alloc_fad0cd83b7d1858a846a172eb260e593 = private unnamed_addr constant [42 x i8] c"is_aligned_to: align is not a power-of-two", align 1
@alloc_e92e94d0ff530782b571cfd99ec66aef = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_fad0cd83b7d1858a846a172eb260e593, [8 x i8] c"*\00\00\00\00\00\00\00" }>, align 8
@alloc_d223581b72e206fca9e9c24d510f856e = private unnamed_addr constant [115 x i8] c"/Users/jiten/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/const_ptr.rs\00", align 1
@alloc_eac3d824e42a01065ce06cb143766110 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_d223581b72e206fca9e9c24d510f856e, [16 x i8] c"r\00\00\00\00\00\00\00^\05\00\00\0D\00\00\00" }>, align 8
@alloc_bd3468a7b96187f70c1ce98a3e7a63bf = private unnamed_addr constant [283 x i8] c"unsafe precondition(s) violated: ptr::copy_nonoverlapping requires that both pointer arguments are aligned and non-null and the specified memory ranges do not overlap\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_64e308ef4babfeb8b6220184de794a17 = private unnamed_addr constant [221 x i8] c"unsafe precondition(s) violated: hint::assert_unchecked must never be called when the condition is false\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_529ab01e55306001f97506c4312a6bcd = private unnamed_addr constant [111 x i8] c"/Users/jiten/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ub_checks.rs\00", align 1
@alloc_635fc0aab9b543c20346dd146d54c413 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_529ab01e55306001f97506c4312a6bcd, [16 x i8] c"n\00\00\00\00\00\00\00\94\00\00\006\00\00\00" }>, align 8
@alloc_a28e8c8fd5088943a8b5d44af697ff83 = private unnamed_addr constant [279 x i8] c"unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to be aligned and non-null, and the total size of the slice not to exceed `isize::MAX`\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_763310d78c99c2c1ad3f8a9821e942f3 = private unnamed_addr constant [61 x i8] c"is_nonoverlapping: `size_of::<T>() * count` overflows a usize", align 1
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant [1 x i8] c"\0A", align 1
@alloc_9771be2481f51be410bd2ac520d18601 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer, ptr @alloc_49a1e817e911805af64bbc7efb390101, [8 x i8] c"\01\00\00\00\00\00\00\00" }>, align 8
@alloc_3edef0b68cfa9c8c95e6d4fe1a68842b = private unnamed_addr constant [5 x i8] c"Hello", align 1
@alloc_57d70e9d94c65ecfc15225d29a5ed72b = private unnamed_addr constant [198 x i8] c"unsafe precondition(s) violated: Vec::set_len requires that new_len <= capacity()\0A\0AThis indicates a bug in the program. This Undefined Behavior check is optional, and cannot be relied on for safety.", align 1
@alloc_69ffe2091b46fff79749f36e5872c1c3 = private unnamed_addr constant [110 x i8] c"/Users/jiten/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs\00", align 1
@alloc_45d9bde824eaf8c7c137c6cac06cfaa6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69ffe2091b46fff79749f36e5872c1c3, [16 x i8] c"m\00\00\00\00\00\00\00\87\06\00\00\12\00\00\00" }>, align 8
@alloc_d3a417227aabfa5716961bc00ee236df = private unnamed_addr constant [108 x i8] c"/Users/jiten/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs\00", align 1
@alloc_05441d7184f3a3d1288dcc4d94055447 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_d3a417227aabfa5716961bc00ee236df, [16 x i8] c"k\00\00\00\00\00\00\00\BD\01\00\00\1D\00\00\00" }>, align 8
@alloc_0bb5b8f08fd5c02a63b2190e1c166c91 = private unnamed_addr constant [109 x i8] c"/Users/jiten/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/ptr/mod.rs\00", align 1
@alloc_e4dc20d2cae5f82cf9816b90557d9a81 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_0bb5b8f08fd5c02a63b2190e1c166c91, [16 x i8] c"l\00\00\00\00\00\00\00\0F\02\00\00\05\00\00\00" }>, align 8
@alloc_ffedbed2222a78852ecc406c34126416 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69ffe2091b46fff79749f36e5872c1c3, [16 x i8] c"m\00\00\00\00\00\00\00\DD\07\00\00\09\00\00\00" }>, align 8

; std::rt::lang_start
; Function Attrs: uwtable
define hidden i64 @_ZN3std2rt10lang_start17h8f3f1d2dc6734b7fE(ptr %main, i64 %argc, ptr %argv, i8 %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call i64 @_ZN3std2rt19lang_start_internal17h7e788da8c79e20dcE(ptr align 1 %_7, ptr align 8 @vtable.0, i64 %argc, ptr %argv, i8 %sigpipe)
  ret i64 %_0
}

; std::rt::lang_start::{{closure}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E"(ptr align 8 %_1) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %_1, align 8
; call std::sys::backtrace::__rust_begin_short_backtrace
  call void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h42045ea58f9d9c4dE(ptr %_4)
; call <() as std::process::Termination>::report
  %self = call i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6bef7039522cf155E"()
  %_0 = zext i8 %self to i32
  ret i32 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace
; Function Attrs: noinline uwtable
define internal void @_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h42045ea58f9d9c4dE(ptr %f) unnamed_addr #2 {
start:
; call core::ops::function::FnOnce::call_once
  call void @_ZN4core3ops8function6FnOnce9call_once17h3f1179ab9007e468E(ptr %f)
  call void asm sideeffect "", "~{memory}"(), !srcloc !3
  ret void
}

; <T as alloc::string::ToString>::to_string
; Function Attrs: inlinehint uwtable
define internal void @"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h72a87fe360ab2743E"(ptr sret([24 x i8]) align 8 %_0, ptr align 1 %self.0, i64 %self.1) unnamed_addr #1 {
start:
; call <str as alloc::string::SpecToString>::spec_to_string
  call void @"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h40fc4fdca5c69852E"(ptr sret([24 x i8]) align 8 %_0, ptr align 1 %self.0, i64 %self.1)
  ret void
}

; core::intrinsics::cold_path
; Function Attrs: cold nounwind uwtable
define internal void @_ZN4core10intrinsics9cold_path17hb14d4c3de9c956c2E() unnamed_addr #3 {
start:
  ret void
}

; core::fmt::rt::<impl core::fmt::Arguments>::new_v1
; Function Attrs: inlinehint uwtable
define internal void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117h52740d1096461af2E"(ptr sret([48 x i8]) align 8 %_0, ptr align 8 %pieces, ptr align 8 %args) unnamed_addr #1 {
start:
  store ptr %pieces, ptr %_0, align 8
  %0 = getelementptr inbounds i8, ptr %_0, i64 8
  store i64 2, ptr %0, align 8
  %1 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %2 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %3 = getelementptr inbounds i8, ptr %_0, i64 32
  store ptr %1, ptr %3, align 8
  %4 = getelementptr inbounds i8, ptr %3, i64 8
  store i64 %2, ptr %4, align 8
  %5 = getelementptr inbounds i8, ptr %_0, i64 16
  store ptr %args, ptr %5, align 8
  %6 = getelementptr inbounds i8, ptr %5, i64 8
  store i64 1, ptr %6, align 8
  ret void
}

; core::fmt::rt::Argument::new_display
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt2rt8Argument11new_display17h724f44171c7c84f5E(ptr sret([16 x i8]) align 8 %_0, ptr align 4 %x) unnamed_addr #1 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %x, ptr %_2, align 8
  %0 = getelementptr inbounds i8, ptr %_2, i64 8
  store ptr @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf71ca2f276637ae9E", ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_2, i64 16, i1 false)
  ret void
}

; core::fmt::rt::Argument::new_display
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt2rt8Argument11new_display17hdff29c69bcb68acfE(ptr sret([16 x i8]) align 8 %_0, ptr align 8 %x) unnamed_addr #1 {
start:
  %_2 = alloca [16 x i8], align 8
  store ptr %x, ptr %_2, align 8
  %0 = getelementptr inbounds i8, ptr %_2, i64 8
  store ptr @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h69b885877aab5ca5E", ptr %0, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %_2, i64 16, i1 false)
  ret void
}

; core::ops::function::FnOnce::call_once{{vtable.shim}}
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17h53e7a5429aeb5b8dE"(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  %0 = load ptr, ptr %_1, align 8
; call core::ops::function::FnOnce::call_once
  %_0 = call i32 @_ZN4core3ops8function6FnOnce9call_once17h09dee0142ad55168E(ptr %0)
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal i32 @_ZN4core3ops8function6FnOnce9call_once17h09dee0142ad55168E(ptr %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %1 = alloca [16 x i8], align 8
  %_2 = alloca [0 x i8], align 1
  %_1 = alloca [8 x i8], align 8
  store ptr %0, ptr %_1, align 8
; invoke std::rt::lang_start::{{closure}}
  %_0 = invoke i32 @"_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17hc0676eddb8145381E"(ptr align 8 %_1)
          to label %bb1 unwind label %cleanup

bb3:                                              ; preds = %cleanup
  %2 = load ptr, ptr %1, align 8
  %3 = getelementptr inbounds i8, ptr %1, i64 8
  %4 = load i32, ptr %3, align 8
  %5 = insertvalue { ptr, i32 } poison, ptr %2, 0
  %6 = insertvalue { ptr, i32 } %5, i32 %4, 1
  resume { ptr, i32 } %6

cleanup:                                          ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
  %8 = extractvalue { ptr, i32 } %7, 0
  %9 = extractvalue { ptr, i32 } %7, 1
  store ptr %8, ptr %1, align 8
  %10 = getelementptr inbounds i8, ptr %1, i64 8
  store i32 %9, ptr %10, align 8
  br label %bb3

bb1:                                              ; preds = %start
  ret i32 %_0
}

; core::ops::function::FnOnce::call_once
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3ops8function6FnOnce9call_once17h3f1179ab9007e468E(ptr %_1) unnamed_addr #1 {
start:
  %_2 = alloca [0 x i8], align 1
  call void %_1()
  ret void
}

; core::ptr::copy_nonoverlapping::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @_ZN4core3ptr19copy_nonoverlapping18precondition_check17h39204a95ee8aee99E(ptr %src, ptr %dst, i64 %size, i64 %align, i64 %count, ptr align 8 %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %1 = alloca [4 x i8], align 4
  %_29 = alloca [48 x i8], align 8
  %_24 = alloca [4 x i8], align 4
  %_23 = alloca [8 x i8], align 8
  %_22 = alloca [8 x i8], align 8
  %_21 = alloca [8 x i8], align 8
  %_20 = alloca [48 x i8], align 8
  %_17 = alloca [16 x i8], align 8
  %_15 = alloca [48 x i8], align 8
  %is_zst = alloca [1 x i8], align 1
  %align1 = alloca [8 x i8], align 8
  %zero_size = alloca [1 x i8], align 1
  %2 = icmp eq i64 %count, 0
  br i1 %2, label %bb1, label %bb2

bb1:                                              ; preds = %start
  store i8 1, ptr %zero_size, align 1
  store i64 %align, ptr %align1, align 8
  %3 = load i8, ptr %zero_size, align 1
  %4 = trunc nuw i8 %3 to i1
  %5 = zext i1 %4 to i8
  store i8 %5, ptr %is_zst, align 1
  %6 = call i64 @llvm.ctpop.i64(i64 %align)
  %7 = trunc i64 %6 to i32
  store i32 %7, ptr %_24, align 4
  %8 = load i32, ptr %_24, align 4
  %9 = icmp eq i32 %8, 1
  br i1 %9, label %bb26, label %bb15

bb2:                                              ; preds = %start
  %10 = icmp eq i64 %size, 0
  %11 = zext i1 %10 to i8
  store i8 %11, ptr %zero_size, align 1
  store i64 %align, ptr %align1, align 8
  %12 = load i8, ptr %zero_size, align 1
  %13 = trunc nuw i8 %12 to i1
  %14 = zext i1 %13 to i8
  store i8 %14, ptr %is_zst, align 1
  %15 = call i64 @llvm.ctpop.i64(i64 %align)
  %16 = trunc i64 %15 to i32
  store i32 %16, ptr %_24, align 4
  %17 = load i32, ptr %_24, align 4
  %18 = icmp eq i32 %17, 1
  br i1 %18, label %bb14, label %bb15

bb26:                                             ; preds = %bb1
  store ptr %src, ptr %_22, align 8
  %19 = sub i64 %align, 1
  store i64 %19, ptr %_23, align 8
  %20 = load i64, ptr %_22, align 8
  %21 = load i64, ptr %_23, align 8
  %22 = and i64 %20, %21
  store i64 %22, ptr %_21, align 8
  %23 = load i64, ptr %_21, align 8
  %24 = icmp eq i64 %23, 0
  br i1 %24, label %bb27, label %bb11

bb15:                                             ; preds = %bb2, %bb1
  store ptr @alloc_e92e94d0ff530782b571cfd99ec66aef, ptr %_20, align 8
  %25 = getelementptr inbounds i8, ptr %_20, i64 8
  store i64 1, ptr %25, align 8
  %26 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %27 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %28 = getelementptr inbounds i8, ptr %_20, i64 32
  store ptr %26, ptr %28, align 8
  %29 = getelementptr inbounds i8, ptr %28, i64 8
  store i64 %27, ptr %29, align 8
  %30 = getelementptr inbounds i8, ptr %_20, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %30, align 8
  %31 = getelementptr inbounds i8, ptr %30, i64 8
  store i64 0, ptr %31, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h9ec18d20141b7425E(ptr align 8 %_20, ptr align 8 @alloc_eac3d824e42a01065ce06cb143766110) #13
          to label %unreachable unwind label %terminate

bb27:                                             ; preds = %bb26
  br label %bb12

bb11:                                             ; preds = %bb14, %bb26
  br label %bb6

bb12:                                             ; preds = %bb10, %bb27
  br label %bb3

bb14:                                             ; preds = %bb2
  store ptr %src, ptr %_22, align 8
  %32 = sub i64 %align, 1
  store i64 %32, ptr %_23, align 8
  %33 = load i64, ptr %_22, align 8
  %34 = load i64, ptr %_23, align 8
  %35 = and i64 %33, %34
  store i64 %35, ptr %_21, align 8
  %36 = load i64, ptr %_21, align 8
  %37 = icmp eq i64 %36, 0
  br i1 %37, label %bb10, label %bb11

bb10:                                             ; preds = %bb14
  %38 = load i8, ptr %is_zst, align 1
  %39 = trunc nuw i8 %38 to i1
  br i1 %39, label %bb12, label %bb13

bb13:                                             ; preds = %bb10
  %40 = load i64, ptr %_22, align 8
  %_18 = icmp eq i64 %40, 0
  %_8 = xor i1 %_18, true
  br i1 %_8, label %bb3, label %bb6

bb6:                                              ; preds = %bb11, %bb13
  br label %bb7

bb3:                                              ; preds = %bb12, %bb13
  %41 = load i8, ptr %zero_size, align 1
  %is_zst2 = trunc nuw i8 %41 to i1
  %42 = call i64 @llvm.ctpop.i64(i64 %align)
  %43 = trunc i64 %42 to i32
  store i32 %43, ptr %1, align 4
  %_32 = load i32, ptr %1, align 4
  %44 = icmp eq i32 %_32, 1
  br i1 %44, label %bb21, label %bb22

bb21:                                             ; preds = %bb3
  %_31 = ptrtoint ptr %dst to i64
  %45 = load i64, ptr %_23, align 8
  %_30 = and i64 %_31, %45
  %46 = icmp eq i64 %_30, 0
  br i1 %46, label %bb17, label %bb18

bb22:                                             ; preds = %bb3
  store ptr @alloc_e92e94d0ff530782b571cfd99ec66aef, ptr %_29, align 8
  %47 = getelementptr inbounds i8, ptr %_29, i64 8
  store i64 1, ptr %47, align 8
  %48 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %49 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %50 = getelementptr inbounds i8, ptr %_29, i64 32
  store ptr %48, ptr %50, align 8
  %51 = getelementptr inbounds i8, ptr %50, i64 8
  store i64 %49, ptr %51, align 8
  %52 = getelementptr inbounds i8, ptr %_29, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %52, align 8
  %53 = getelementptr inbounds i8, ptr %52, i64 8
  store i64 0, ptr %53, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h9ec18d20141b7425E(ptr align 8 %_29, ptr align 8 @alloc_eac3d824e42a01065ce06cb143766110) #13
          to label %unreachable unwind label %terminate

bb17:                                             ; preds = %bb21
  br i1 %is_zst2, label %bb19, label %bb20

bb18:                                             ; preds = %bb21
  br label %bb5

bb20:                                             ; preds = %bb17
  %_27 = icmp eq i64 %_31, 0
  %_11 = xor i1 %_27, true
  br i1 %_11, label %bb4, label %bb5

bb19:                                             ; preds = %bb17
  br label %bb4

bb5:                                              ; preds = %bb18, %bb20
  br label %bb7

bb4:                                              ; preds = %bb19, %bb20
; invoke core::ub_checks::maybe_is_nonoverlapping::runtime
  %_6 = invoke zeroext i1 @_ZN4core9ub_checks23maybe_is_nonoverlapping7runtime17hbb8100947e5157aaE(ptr %src, ptr %dst, i64 %size, i64 %count)
          to label %bb24 unwind label %terminate

terminate:                                        ; preds = %bb15, %bb22, %bb4
  %54 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17h3811ef26462f5a57E() #14
  unreachable

bb24:                                             ; preds = %bb4
  br i1 %_6, label %bb9, label %bb8

bb8:                                              ; preds = %bb7, %bb24
  %55 = getelementptr inbounds nuw { ptr, i64 }, ptr %_17, i64 0
  store ptr @alloc_bd3468a7b96187f70c1ce98a3e7a63bf, ptr %55, align 8
  %56 = getelementptr inbounds i8, ptr %55, i64 8
  store i64 283, ptr %56, align 8
  store ptr %_17, ptr %_15, align 8
  %57 = getelementptr inbounds i8, ptr %_15, i64 8
  store i64 1, ptr %57, align 8
  %58 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %59 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %60 = getelementptr inbounds i8, ptr %_15, i64 32
  store ptr %58, ptr %60, align 8
  %61 = getelementptr inbounds i8, ptr %60, i64 8
  store i64 %59, ptr %61, align 8
  %62 = getelementptr inbounds i8, ptr %_15, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %62, align 8
  %63 = getelementptr inbounds i8, ptr %62, i64 8
  store i64 0, ptr %63, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17he7b3b461c44adf19E(ptr align 8 %_15, i1 zeroext false, ptr align 8 %0) #15
  unreachable

bb9:                                              ; preds = %bb24
  ret void

bb7:                                              ; preds = %bb6, %bb5
  br label %bb8

unreachable:                                      ; preds = %bb15, %bb22
  unreachable
}

; core::ptr::drop_in_place<alloc::string::String>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h699f9b831a8e228fE"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call core::ptr::drop_in_place<alloc::vec::Vec<u8>>
  call void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0a467c9f9aa1ee5fE"(ptr align 8 %_1)
  ret void
}

; core::ptr::drop_in_place<alloc::vec::Vec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17h0a467c9f9aa1ee5fE"(ptr align 8 %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
; invoke <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
  invoke void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h715dd9a513976372E"(ptr align 8 %_1)
          to label %bb4 unwind label %cleanup

bb3:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  invoke void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h0e6690d518606c4eE"(ptr align 8 %_1) #16
          to label %bb1 unwind label %terminate

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = extractvalue { ptr, i32 } %1, 0
  %3 = extractvalue { ptr, i32 } %1, 1
  store ptr %2, ptr %0, align 8
  %4 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %3, ptr %4, align 8
  br label %bb3

bb4:                                              ; preds = %start
; call core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
  call void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h0e6690d518606c4eE"(ptr align 8 %_1)
  ret void

terminate:                                        ; preds = %bb3
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hf9a3242f02e22468E() #14
  unreachable

bb1:                                              ; preds = %bb3
  %6 = load ptr, ptr %0, align 8
  %7 = getelementptr inbounds i8, ptr %0, i64 8
  %8 = load i32, ptr %7, align 8
  %9 = insertvalue { ptr, i32 } poison, ptr %6, 0
  %10 = insertvalue { ptr, i32 } %9, i32 %8, 1
  resume { ptr, i32 } %10
}

; core::ptr::drop_in_place<alloc::raw_vec::RawVec<u8>>
; Function Attrs: uwtable
define internal void @"_ZN4core3ptr53drop_in_place$LT$alloc..raw_vec..RawVec$LT$u8$GT$$GT$17h0e6690d518606c4eE"(ptr align 8 %_1) unnamed_addr #0 {
start:
; call <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
  call void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h83663997b631b7a1E"(ptr align 8 %_1)
  ret void
}

; core::hint::assert_unchecked::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @_ZN4core4hint16assert_unchecked18precondition_check17hf2423342d4ce0dc8E(i1 zeroext %cond, ptr align 8 %0) unnamed_addr #4 {
start:
  %_5 = alloca [16 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  br i1 %cond, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %1 = getelementptr inbounds nuw { ptr, i64 }, ptr %_5, i64 0
  store ptr @alloc_64e308ef4babfeb8b6220184de794a17, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  store i64 221, ptr %2, align 8
  store ptr %_5, ptr %_3, align 8
  %3 = getelementptr inbounds i8, ptr %_3, i64 8
  store i64 1, ptr %3, align 8
  %4 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %5 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %6 = getelementptr inbounds i8, ptr %_3, i64 32
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %_3, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 0, ptr %9, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17he7b3b461c44adf19E(ptr align 8 %_3, i1 zeroext false, ptr align 8 %0) #15
  unreachable

bb2:                                              ; preds = %start
  ret void
}

; core::slice::raw::from_raw_parts::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @_ZN4core5slice3raw14from_raw_parts18precondition_check17hf0736a9dec3dfd5fE(ptr %data, i64 %size, i64 %align, i64 %len, ptr align 8 %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %1 = alloca [4 x i8], align 4
  %max_len = alloca [8 x i8], align 8
  %_14 = alloca [48 x i8], align 8
  %_11 = alloca [16 x i8], align 8
  %_9 = alloca [48 x i8], align 8
  %2 = call i64 @llvm.ctpop.i64(i64 %align)
  %3 = trunc i64 %2 to i32
  store i32 %3, ptr %1, align 4
  %_18 = load i32, ptr %1, align 4
  %4 = icmp eq i32 %_18, 1
  br i1 %4, label %bb8, label %bb9

bb8:                                              ; preds = %start
  %_16 = ptrtoint ptr %data to i64
  %_17 = sub i64 %align, 1
  %_15 = and i64 %_16, %_17
  %5 = icmp eq i64 %_15, 0
  br i1 %5, label %bb6, label %bb7

bb9:                                              ; preds = %start
  store ptr @alloc_e92e94d0ff530782b571cfd99ec66aef, ptr %_14, align 8
  %6 = getelementptr inbounds i8, ptr %_14, i64 8
  store i64 1, ptr %6, align 8
  %7 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %8 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %9 = getelementptr inbounds i8, ptr %_14, i64 32
  store ptr %7, ptr %9, align 8
  %10 = getelementptr inbounds i8, ptr %9, i64 8
  store i64 %8, ptr %10, align 8
  %11 = getelementptr inbounds i8, ptr %_14, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %11, align 8
  %12 = getelementptr inbounds i8, ptr %11, i64 8
  store i64 0, ptr %12, align 8
; invoke core::panicking::panic_fmt
  invoke void @_ZN4core9panicking9panic_fmt17h9ec18d20141b7425E(ptr align 8 %_14, ptr align 8 @alloc_eac3d824e42a01065ce06cb143766110) #13
          to label %unreachable unwind label %terminate

bb6:                                              ; preds = %bb8
  %_12 = icmp eq i64 %_16, 0
  %_5 = xor i1 %_12, true
  br i1 %_5, label %bb1, label %bb4

bb7:                                              ; preds = %bb8
  br label %bb4

bb4:                                              ; preds = %bb7, %bb6
  br label %bb5

bb1:                                              ; preds = %bb6
  %_22 = icmp eq i64 %size, 0
  %13 = icmp eq i64 %size, 0
  br i1 %13, label %bb11, label %bb12

bb11:                                             ; preds = %bb1
  store i64 -1, ptr %max_len, align 8
  br label %bb14

bb12:                                             ; preds = %bb1
  br i1 %_22, label %panic, label %bb13

bb14:                                             ; preds = %bb13, %bb11
  %_23 = load i64, ptr %max_len, align 8
  %_7 = icmp ule i64 %len, %_23
  br i1 %_7, label %bb2, label %bb3

bb13:                                             ; preds = %bb12
  %14 = udiv i64 9223372036854775807, %size
  store i64 %14, ptr %max_len, align 8
  br label %bb14

panic:                                            ; preds = %bb12
; invoke core::panicking::panic_const::panic_const_div_by_zero
  invoke void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h1b0d02368eb84e20E(ptr align 8 @alloc_635fc0aab9b543c20346dd146d54c413) #13
          to label %unreachable unwind label %terminate

terminate:                                        ; preds = %bb9, %panic
  %15 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_cannot_unwind
  call void @_ZN4core9panicking19panic_cannot_unwind17h3811ef26462f5a57E() #14
  unreachable

unreachable:                                      ; preds = %bb9, %panic
  unreachable

bb3:                                              ; preds = %bb14
  br label %bb5

bb2:                                              ; preds = %bb14
  ret void

bb5:                                              ; preds = %bb4, %bb3
  %16 = getelementptr inbounds nuw { ptr, i64 }, ptr %_11, i64 0
  store ptr @alloc_a28e8c8fd5088943a8b5d44af697ff83, ptr %16, align 8
  %17 = getelementptr inbounds i8, ptr %16, i64 8
  store i64 279, ptr %17, align 8
  store ptr %_11, ptr %_9, align 8
  %18 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 1, ptr %18, align 8
  %19 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %20 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %21 = getelementptr inbounds i8, ptr %_9, i64 32
  store ptr %19, ptr %21, align 8
  %22 = getelementptr inbounds i8, ptr %21, i64 8
  store i64 %20, ptr %22, align 8
  %23 = getelementptr inbounds i8, ptr %_9, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %23, align 8
  %24 = getelementptr inbounds i8, ptr %23, i64 8
  store i64 0, ptr %24, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17he7b3b461c44adf19E(ptr align 8 %_9, i1 zeroext false, ptr align 8 %0) #15
  unreachable
}

; core::ub_checks::maybe_is_nonoverlapping::runtime
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @_ZN4core9ub_checks23maybe_is_nonoverlapping7runtime17hbb8100947e5157aaE(ptr %src, ptr %dst, i64 %size, i64 %count) unnamed_addr #1 {
start:
  %diff = alloca [8 x i8], align 8
  %_9 = alloca [16 x i8], align 8
  %src_usize = ptrtoint ptr %src to i64
  %dst_usize = ptrtoint ptr %dst to i64
  %0 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %size, i64 %count)
  %_14.0 = extractvalue { i64, i1 } %0, 0
  %_14.1 = extractvalue { i64, i1 } %0, 1
  br i1 %_14.1, label %bb1, label %bb3

bb3:                                              ; preds = %start
  %1 = getelementptr inbounds i8, ptr %_9, i64 8
  store i64 %_14.0, ptr %1, align 8
  store i64 1, ptr %_9, align 8
  %2 = getelementptr inbounds i8, ptr %_9, i64 8
  %size1 = load i64, ptr %2, align 8
  %_22 = icmp ult i64 %src_usize, %dst_usize
  br i1 %_22, label %bb4, label %bb5

bb1:                                              ; preds = %start
; call core::panicking::panic_nounwind
  call void @_ZN4core9panicking14panic_nounwind17h82f92e8e7c9f657cE(ptr align 1 @alloc_763310d78c99c2c1ad3f8a9821e942f3, i64 61) #15
  unreachable

bb5:                                              ; preds = %bb3
  %3 = sub i64 %src_usize, %dst_usize
  store i64 %3, ptr %diff, align 8
  br label %bb6

bb4:                                              ; preds = %bb3
  %4 = sub i64 %dst_usize, %src_usize
  store i64 %4, ptr %diff, align 8
  br label %bb6

bb6:                                              ; preds = %bb4, %bb5
  %_11 = load i64, ptr %diff, align 8
  %_0 = icmp uge i64 %_11, %size1
  ret i1 %_0
}

; main::main
; Function Attrs: uwtable
define hidden void @_ZN4main4main17hfa203070a8b0b79bE() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = alloca [16 x i8], align 8
  %_15 = alloca [16 x i8], align 8
  %args1 = alloca [16 x i8], align 8
  %_13 = alloca [48 x i8], align 8
  %str2 = alloca [24 x i8], align 8
  %str1 = alloca [24 x i8], align 8
  %_5 = alloca [16 x i8], align 8
  %args = alloca [16 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  %i = alloca [4 x i8], align 4
  store i32 100, ptr %i, align 4
; call core::fmt::rt::Argument::new_display
  call void @_ZN4core3fmt2rt8Argument11new_display17h724f44171c7c84f5E(ptr sret([16 x i8]) align 8 %_5, ptr align 4 %i)
  %1 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %1, ptr align 8 %_5, i64 16, i1 false)
; call core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  call void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117h52740d1096461af2E"(ptr sret([48 x i8]) align 8 %_3, ptr align 8 @alloc_9771be2481f51be410bd2ac520d18601, ptr align 8 %args)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h657677ef6fd356f5E(ptr align 8 %_3)
; call <T as alloc::string::ToString>::to_string
  call void @"_ZN45_$LT$T$u20$as$u20$alloc..string..ToString$GT$9to_string17h72a87fe360ab2743E"(ptr sret([24 x i8]) align 8 %str1, ptr align 1 @alloc_3edef0b68cfa9c8c95e6d4fe1a68842b, i64 5)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %str2, ptr align 8 %str1, i64 24, i1 false)
; invoke core::fmt::rt::Argument::new_display
  invoke void @_ZN4core3fmt2rt8Argument11new_display17hdff29c69bcb68acfE(ptr sret([16 x i8]) align 8 %_15, ptr align 8 %str2)
          to label %bb5 unwind label %cleanup

bb9:                                              ; preds = %cleanup
; invoke core::ptr::drop_in_place<alloc::string::String>
  invoke void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h699f9b831a8e228fE"(ptr align 8 %str2) #16
          to label %bb10 unwind label %terminate

cleanup:                                          ; preds = %bb6, %bb5, %start
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = extractvalue { ptr, i32 } %2, 0
  %4 = extractvalue { ptr, i32 } %2, 1
  store ptr %3, ptr %0, align 8
  %5 = getelementptr inbounds i8, ptr %0, i64 8
  store i32 %4, ptr %5, align 8
  br label %bb9

bb5:                                              ; preds = %start
  %6 = getelementptr inbounds nuw %"core::fmt::rt::Argument<'_>", ptr %args1, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %6, ptr align 8 %_15, i64 16, i1 false)
; invoke core::fmt::rt::<impl core::fmt::Arguments>::new_v1
  invoke void @"_ZN4core3fmt2rt38_$LT$impl$u20$core..fmt..Arguments$GT$6new_v117h52740d1096461af2E"(ptr sret([48 x i8]) align 8 %_13, ptr align 8 @alloc_9771be2481f51be410bd2ac520d18601, ptr align 8 %args1)
          to label %bb6 unwind label %cleanup

bb6:                                              ; preds = %bb5
; invoke std::io::stdio::_print
  invoke void @_ZN3std2io5stdio6_print17h657677ef6fd356f5E(ptr align 8 %_13)
          to label %bb7 unwind label %cleanup

bb7:                                              ; preds = %bb6
; call core::ptr::drop_in_place<alloc::string::String>
  call void @"_ZN4core3ptr42drop_in_place$LT$alloc..string..String$GT$17h699f9b831a8e228fE"(ptr align 8 %str2)
  ret void

terminate:                                        ; preds = %bb9
  %7 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_ZN4core9panicking16panic_in_cleanup17hf9a3242f02e22468E() #14
  unreachable

bb10:                                             ; preds = %bb9
  %8 = load ptr, ptr %0, align 8
  %9 = getelementptr inbounds i8, ptr %0, i64 8
  %10 = load i32, ptr %9, align 8
  %11 = insertvalue { ptr, i32 } poison, ptr %8, 0
  %12 = insertvalue { ptr, i32 } %11, i32 %10, 1
  resume { ptr, i32 } %12
}

; <str as alloc::string::SpecToString>::spec_to_string
; Function Attrs: inlinehint uwtable
define internal void @"_ZN51_$LT$str$u20$as$u20$alloc..string..SpecToString$GT$14spec_to_string17h40fc4fdca5c69852E"(ptr sret([24 x i8]) align 8 %_0, ptr align 1 %self.0, i64 %self.1) unnamed_addr #1 {
start:
  %bytes = alloca [24 x i8], align 8
; call <T as alloc::slice::<impl [T]>::to_vec_in::ConvertVec>::to_vec
  call void @"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hf1ad1b68653b8512E"(ptr sret([24 x i8]) align 8 %bytes, ptr align 1 %self.0, i64 %self.1)
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %bytes, i64 24, i1 false)
  ret void
}

; <() as std::process::Termination>::report
; Function Attrs: inlinehint uwtable
define internal i8 @"_ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17h6bef7039522cf155E"() unnamed_addr #1 {
start:
  ret i8 0
}

; alloc::vec::Vec<T,A>::set_len::precondition_check
; Function Attrs: inlinehint nounwind uwtable
define internal void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len18precondition_check17h15f00798a15ece34E"(i64 %new_len, i64 %capacity, ptr align 8 %0) unnamed_addr #4 {
start:
  %_7 = alloca [16 x i8], align 8
  %_5 = alloca [48 x i8], align 8
  %_3 = icmp ule i64 %new_len, %capacity
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw { ptr, i64 }, ptr %_7, i64 0
  store ptr @alloc_57d70e9d94c65ecfc15225d29a5ed72b, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %1, i64 8
  store i64 198, ptr %2, align 8
  store ptr %_7, ptr %_5, align 8
  %3 = getelementptr inbounds i8, ptr %_5, i64 8
  store i64 1, ptr %3, align 8
  %4 = load ptr, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, align 8
  %5 = load i64, ptr getelementptr inbounds (i8, ptr @anon.1a8ca7ad941f0113fec942780bd214ec.0, i64 8), align 8
  %6 = getelementptr inbounds i8, ptr %_5, i64 32
  store ptr %4, ptr %6, align 8
  %7 = getelementptr inbounds i8, ptr %6, i64 8
  store i64 %5, ptr %7, align 8
  %8 = getelementptr inbounds i8, ptr %_5, i64 16
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds i8, ptr %8, i64 8
  store i64 0, ptr %9, align 8
; call core::panicking::panic_nounwind_fmt
  call void @_ZN4core9panicking18panic_nounwind_fmt17he7b3b461c44adf19E(ptr align 8 %_5, i1 zeroext false, ptr align 8 %0) #15
  unreachable

bb1:                                              ; preds = %start
  ret void
}

; alloc::raw_vec::RawVecInner<A>::with_capacity_in
; Function Attrs: inlinehint uwtable
define internal { i64, ptr } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17he15a4a1129a704d5E"(i64 %capacity, i64 %elem_layout.0, i64 %elem_layout.1, ptr align 8 %0) unnamed_addr #1 {
start:
  %self = alloca [8 x i8], align 8
  %elem_layout = alloca [16 x i8], align 8
  %this = alloca [16 x i8], align 8
  %_4 = alloca [24 x i8], align 8
; call alloc::raw_vec::RawVecInner<A>::try_allocate_in
  call void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h5d28a70dbd7b7cc5E"(ptr sret([24 x i8]) align 8 %_4, i64 %capacity, i1 zeroext false, i64 %elem_layout.0, i64 %elem_layout.1)
  %_5 = load i64, ptr %_4, align 8
  %1 = trunc nuw i64 %_5 to i1
  br i1 %1, label %bb3, label %bb4

bb3:                                              ; preds = %start
  %2 = getelementptr inbounds i8, ptr %_4, i64 8
  %err.0 = load i64, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %2, i64 8
  %err.1 = load i64, ptr %3, align 8
; call alloc::raw_vec::handle_error
  call void @_ZN5alloc7raw_vec12handle_error17h9004b972ed53e1ccE(i64 %err.0, i64 %err.1, ptr align 8 %0) #13
  unreachable

bb4:                                              ; preds = %start
  %4 = getelementptr inbounds i8, ptr %_4, i64 8
  %5 = load i64, ptr %4, align 8
  %6 = getelementptr inbounds i8, ptr %4, i64 8
  %7 = load ptr, ptr %6, align 8
  store i64 %5, ptr %this, align 8
  %8 = getelementptr inbounds i8, ptr %this, i64 8
  store ptr %7, ptr %8, align 8
  store i64 %elem_layout.0, ptr %elem_layout, align 8
  %9 = getelementptr inbounds i8, ptr %elem_layout, i64 8
  store i64 %elem_layout.1, ptr %9, align 8
  %10 = icmp eq i64 %elem_layout.1, 0
  br i1 %10, label %bb6, label %bb7

bb6:                                              ; preds = %bb4
  store i64 -1, ptr %self, align 8
  br label %bb5

bb7:                                              ; preds = %bb4
  %self1 = load i64, ptr %this, align 8
  store i64 %self1, ptr %self, align 8
  br label %bb5

bb5:                                              ; preds = %bb7, %bb6
  %11 = load i64, ptr %self, align 8
  %_13 = sub i64 %11, 0
  %_8 = icmp ugt i64 %capacity, %_13
  %cond = xor i1 %_8, true
  br label %bb8

bb8:                                              ; preds = %bb5
; call core::hint::assert_unchecked::precondition_check
  call void @_ZN4core4hint16assert_unchecked18precondition_check17hf2423342d4ce0dc8E(i1 zeroext %cond, ptr align 8 %0) #17
  br label %bb9

bb9:                                              ; preds = %bb8
  %_0.0 = load i64, ptr %this, align 8
  %12 = getelementptr inbounds i8, ptr %this, i64 8
  %_0.1 = load ptr, ptr %12, align 8
  %13 = insertvalue { i64, ptr } poison, i64 %_0.0, 0
  %14 = insertvalue { i64, ptr } %13, ptr %_0.1, 1
  ret { i64, ptr } %14

bb2:                                              ; No predecessors!
  unreachable
}

; <alloc::string::String as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal zeroext i1 @"_ZN60_$LT$alloc..string..String$u20$as$u20$core..fmt..Display$GT$3fmt17h69b885877aab5ca5E"(ptr align 8 %self, ptr align 8 %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds i8, ptr %self, i64 8
  %_8 = load ptr, ptr %0, align 8
  %1 = getelementptr inbounds i8, ptr %self, i64 16
  %len = load i64, ptr %1, align 8
  br label %bb2

bb2:                                              ; preds = %start
; call core::slice::raw::from_raw_parts::precondition_check
  call void @_ZN4core5slice3raw14from_raw_parts18precondition_check17hf0736a9dec3dfd5fE(ptr %_8, i64 1, i64 1, i64 %len, ptr align 8 @alloc_45d9bde824eaf8c7c137c6cac06cfaa6) #17
  br label %bb4

bb4:                                              ; preds = %bb2
; call <str as core::fmt::Display>::fmt
  %_0 = call zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h77df5c2970cb7cfcE"(ptr align 1 %_8, i64 %len, ptr align 8 %f)
  ret i1 %_0
}

; <T as alloc::slice::<impl [T]>::to_vec_in::ConvertVec>::to_vec
; Function Attrs: inlinehint uwtable
define internal void @"_ZN87_$LT$T$u20$as$u20$alloc..slice..$LT$impl$u20$$u5b$T$u5d$$GT$..to_vec_in..ConvertVec$GT$6to_vec17hf1ad1b68653b8512E"(ptr sret([24 x i8]) align 8 %_0, ptr align 1 %s.0, i64 %s.1) unnamed_addr #1 {
start:
  %_21 = alloca [8 x i8], align 8
  %v = alloca [24 x i8], align 8
; call alloc::raw_vec::RawVecInner<A>::with_capacity_in
  %0 = call { i64, ptr } @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$16with_capacity_in17he15a4a1129a704d5E"(i64 %s.1, i64 1, i64 1, ptr align 8 @alloc_05441d7184f3a3d1288dcc4d94055447)
  %_11.0 = extractvalue { i64, ptr } %0, 0
  %_11.1 = extractvalue { i64, ptr } %0, 1
  store i64 %_11.0, ptr %v, align 8
  %1 = getelementptr inbounds i8, ptr %v, i64 8
  store ptr %_11.1, ptr %1, align 8
  %2 = getelementptr inbounds i8, ptr %v, i64 16
  store i64 0, ptr %2, align 8
  %3 = getelementptr inbounds i8, ptr %v, i64 8
  %_13 = load ptr, ptr %3, align 8
  br label %bb2

bb2:                                              ; preds = %start
; call core::ptr::copy_nonoverlapping::precondition_check
  call void @_ZN4core3ptr19copy_nonoverlapping18precondition_check17h39204a95ee8aee99E(ptr %s.0, ptr %_13, i64 1, i64 1, i64 %s.1, ptr align 8 @alloc_e4dc20d2cae5f82cf9816b90557d9a81) #17
  %4 = mul i64 %s.1, 1
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_13, ptr align 1 %s.0, i64 %4, i1 false)
  br label %bb5

bb5:                                              ; preds = %bb2
  br label %bb10

bb10:                                             ; preds = %bb5
  %self = load i64, ptr %v, align 8
  store i64 %self, ptr %_21, align 8
  br label %bb8

bb9:                                              ; No predecessors!
  store i64 -1, ptr %_21, align 8
  br label %bb8

bb8:                                              ; preds = %bb10, %bb9
  %5 = load i64, ptr %_21, align 8
; call alloc::vec::Vec<T,A>::set_len::precondition_check
  call void @"_ZN5alloc3vec16Vec$LT$T$C$A$GT$7set_len18precondition_check17h15f00798a15ece34E"(i64 %s.1, i64 %5, ptr align 8 @alloc_ffedbed2222a78852ecc406c34126416) #17
  br label %bb7

bb7:                                              ; preds = %bb8
  %6 = getelementptr inbounds i8, ptr %v, i64 16
  store i64 %s.1, ptr %6, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr align 8 %_0, ptr align 8 %v, i64 24, i1 false)
  ret void

bb4:                                              ; No predecessors!
  unreachable
}

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare i64 @_ZN3std2rt19lang_start_internal17h7e788da8c79e20dcE(ptr align 1, ptr align 8, i64, ptr, i8) unnamed_addr #0

; core::fmt::num::imp::<impl core::fmt::Display for i32>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$i32$GT$3fmt17hf71ca2f276637ae9E"(ptr align 4, ptr align 8) unnamed_addr #0

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #5

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #6

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctpop.i64(i64) #7

; core::panicking::panic_cannot_unwind
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_ZN4core9panicking19panic_cannot_unwind17h3811ef26462f5a57E() unnamed_addr #8

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17h9ec18d20141b7425E(ptr align 8, ptr align 8) unnamed_addr #9

; core::panicking::panic_nounwind_fmt
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking18panic_nounwind_fmt17he7b3b461c44adf19E(ptr align 8, i1 zeroext, ptr align 8) unnamed_addr #10

; <alloc::vec::Vec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @"_ZN70_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h715dd9a513976372E"(ptr align 8) unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_ZN4core9panicking16panic_in_cleanup17hf9a3242f02e22468E() unnamed_addr #8

; <alloc::raw_vec::RawVec<T,A> as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @"_ZN77_$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h83663997b631b7a1E"(ptr align 8) unnamed_addr #0

; core::panicking::panic_const::panic_const_div_by_zero
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking11panic_const23panic_const_div_by_zero17h1b0d02368eb84e20E(ptr align 8) unnamed_addr #9

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #7

; core::panicking::panic_nounwind
; Function Attrs: cold noinline noreturn nounwind uwtable
declare void @_ZN4core9panicking14panic_nounwind17h82f92e8e7c9f657cE(ptr align 1, i64) unnamed_addr #10

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h657677ef6fd356f5E(ptr align 8) unnamed_addr #0

; alloc::raw_vec::RawVecInner<A>::try_allocate_in
; Function Attrs: uwtable
declare void @"_ZN5alloc7raw_vec20RawVecInner$LT$A$GT$15try_allocate_in17h5d28a70dbd7b7cc5E"(ptr sret([24 x i8]) align 8, i64, i1 zeroext, i64, i64) unnamed_addr #0

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_ZN5alloc7raw_vec12handle_error17h9004b972ed53e1ccE(i64, i64, ptr align 8) unnamed_addr #11

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h77df5c2970cb7cfcE"(ptr align 1, i64, ptr align 8) unnamed_addr #0

define i32 @main(i32 %0, ptr %1) unnamed_addr #12 {
top:
  %2 = sext i32 %0 to i64
; call std::rt::lang_start
  %3 = call i64 @_ZN3std2rt10lang_start17h8f3f1d2dc6734b7fE(ptr @_ZN4main4main17hfa203070a8b0b79bE, i64 %2, ptr %1, i8 0)
  %4 = trunc i64 %3 to i32
  ret i32 %4
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #8 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { cold noinline noreturn nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { "frame-pointer"="non-leaf" "target-cpu"="apple-m1" }
attributes #13 = { noreturn }
attributes #14 = { cold noreturn nounwind }
attributes #15 = { noreturn nounwind }
attributes #16 = { cold }
attributes #17 = { nounwind }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.91.0 (f8297e351 2025-10-28)"}
!3 = !{i64 12258124428446864}

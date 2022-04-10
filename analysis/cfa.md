---
date: 2022-04-10
status: draft
---

# Control-flow analysis

CFA analyses the entire `.text` section with linear complexity.

It has the following goals:
- Detect function boundaries
- Create the control-flow graph of each function

## Pass 1: Find control-flow intrinsics

Input:
- `.text` section

Output:
- Locations of compiler intrinsics

Rules

*savegpr/restgpr detection* (CodeWarrior only)

Find the following instructions in `.text`

```asm
_save_gpr:
stw r14, -0x48(r11)
stw r15, -0x44(r11)
stw r16, -0x40(r11)
stw r17, -0x3c(r11)
stw r18, -0x38(r11)
stw r19, -0x34(r11)
stw r20, -0x30(r11)
stw r21, -0x2c(r11)
stw r22, -0x28(r11)
stw r23, -0x24(r11)
stw r24, -0x20(r11)
stw r25, -0x1c(r11)
stw r26, -0x18(r11)
stw r27, -0x14(r11)
stw r28, -0x10(r11)
stw r29, -0xc(r11)
stw r30, -0x8(r11)
stw r31, -0x4(r11)
blr

_load_gpr:
lwz r14, -0x48(r11)
lwz r15, -0x44(r11)
lwz r16, -0x40(r11)
lwz r17, -0x3c(r11)
lwz r18, -0x38(r11)
lwz r19, -0x34(r11)
lwz r20, -0x30(r11)
lwz r21, -0x2c(r11)
lwz r22, -0x28(r11)
lwz r23, -0x24(r11)
lwz r24, -0x20(r11)
lwz r25, -0x1c(r11)
lwz r26, -0x18(r11)
lwz r27, -0x14(r11)
lwz r28, -0x10(r11)
lwz r29, -0xc(r11)
lwz r30, -0x8(r11)
lwz r31, -0x4(r11)
blr
```

## Pass 2: Branch analysis

Input:
- `.text` section

Output:
- Slices (cuts) from which basic blocks can be derived
- Forward edges between basic blocks
- Initial set of function boundaries

#### Branch instruction hints

- Iterate over all branch opcodes (b, bc, bcctr, bclr)
- Assume that branches with link register save …
  - point to the start of another function;
  - eventually return back to the call site.
- Assume that branches without link register save are …
  - tail calls if their target precedes the function start of the call site;
  - probably jumps to basic blocks within the function otherwise (might be wrong)
- Skip indirect local branches (bcctr, bclr) for now.

#### Stack frame detection (CodeWarrior only)

Detect patterns matching stack frames. They might be reordered due to instruction scheduling.

Since CodeWarrior will only emit one function epilogue and prologue per function,
we can use this hint (if present) to reliably detect function bounds.

- On function entry point:
  - Execute instructions up to next branch
  - Derive stack frame from changes in machine state
  - If the stack changed, we found an epilog
- When a `blr` (function return) is encountered:
  - Execute instructions in basic block of `blr`
  - If stack-related machine state got reverted, assume this BB to contain the prolog

Example 0x80045de0 from RMCP01 (savegpr/restgpr sled):

```asm
# Stack
# +0x04..+0x08: ret addr to caller
#  0x00..+0x04: caller backchain   <- caller r1
# -0x30.. 0x00: callee saved gprs  <- callee r11
# -0x68..-0x30: callee stack
# -0x6c..-0x68: ret addr to callee
# -0x70..-0x6c: callee backchain   <- callee r1

_alloc_stack_frame:
stwu r1, -0x70(r1)  # store callee backchain & alloc stack frame
mflr r0
…
stw r0, 0x74(r1)    # store ret addr to caller
…
la r11, 0x40(r1)    # load ptr to callee saved gprs
bl _savegpr_22      # save gprs
mr r31, r1

…

_free_stack_frame:
mr r10, r31
la r11, 0x40(r10)   # load ptr to callee saved gprs
bl _restgpr_22      # restore gprs
lwz r10, 0x0(r1)    # load ptr to caller stack frame
lwz r0, 0x4(r10)    # load ret addr to caller
mr r1, r10          # free stack frame
mtlr r0
blr                 # return to caller
```

Example 0x80228490 from RMCP01 (stmw/lmw):

```asm
# Stack
# +0x04..+0x08: ret addr to caller
#  0x00..+0x04: caller backchain   <- caller r1
# -0x30.. 0x00: callee saved gprs  <- callee r11
# -0x38..-0x30: callee stack
# -0x3c..-0x38: ret addr to callee
# -0x40..-0x3c: callee backchain   <- callee r1

_alloc_stack_frame:
stwu r1, -0x40(r1)  # store callee backchain & alloc stack frame
mflr r0
stw r0, 0x44(r1)    # store ret addr to caller
stmw r20, -0x30(r1) # save gprs

…

_free_stack_frame:
lmw r20, -0x30(r1)  # restore gprs
lwz r0, 0x44(r1)    # load ret addr to caller
mtlr r0
la r1, 0x40(r1)     # free stack frame
blr                 # return to caller
```

## TODO

Add the following rules:
- follow indirect local branches
- destructor detection
- vtable detection

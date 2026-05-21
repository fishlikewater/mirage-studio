# Canvas Interaction Feedback Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make image generation clicks show immediate feedback and make node connection handles easier to hit.

**Architecture:** Keep the fix local to the canvas frontend. `ImageEditNode` owns the transient submit guard because the state only matters while that node is preparing a request. Global handle usability is handled in the shared React Flow CSS so all existing node handles benefit without editing each node.

**Tech Stack:** React 18, TypeScript, Vitest, Testing Library, React Flow, Tailwind CSS.

---

## Current Execution Status

- Status: handoff
- Current step: complete pending user review
- Notes: Targeted Vitest suite and production build passed. Browser verification was attempted but the local dev server could not be started from this sandboxed PowerShell environment.

## Task 1: ImageEditNode submit guard

**Files:**
- Modify: `src/features/canvas/nodes/ImageEditNode.test.tsx`
- Modify: `src/features/canvas/nodes/ImageEditNode.tsx`

- [x] **Step 1: Write failing test for duplicate Generate clicks**

Add a test that renders `ImageEditNode` with a valid prompt, makes `canvasAiGateway.submitGenerateImageJob` stay pending, clicks Generate twice, and expects `addNode` and `submitGenerateImageJob` to be called once.

- [x] **Step 2: Run RED verification**

Run: `rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx`

Expected: FAIL because the current component allows a second click before visible generation feedback disables the controls.

- [x] **Step 3: Implement minimal local pending guard**

In `ImageEditNode.tsx`, add a local `isSubmittingGeneration` state plus a ref guard. Set both immediately after validation passes and before `await getRuntimeDiagnostics()`. Disable Edit and Generate while pending. Clear both when the submission setup throws.

- [x] **Step 4: Run GREEN verification**

Run: `rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx`

Expected: PASS.

## Task 2: Connection handle target size

**Files:**
- Modify: `src/index.css`
- Test: `src/features/canvas/nodes/ImageEditNode.test.tsx`

- [x] **Step 1: Write failing assertion for handle size class**

Update the React Flow `Handle` mock to expose `className`, then assert the rendered source and target handles use the shared larger target class instead of the old `!h-2 !w-2`.

- [x] **Step 2: Run RED verification**

Run: `rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx`

Expected: FAIL because the node currently renders the old 8px handle classes.

- [x] **Step 3: Apply shared handle sizing**

Increase the global `.react-flow__handle` target size in `src/index.css`, and remove the per-node `!h-2 !w-2` override from `ImageEditNode` handles so the global target size applies there.

- [x] **Step 4: Run GREEN verification**

Run: `rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx`

Expected: PASS.

## Task 3: Final verification

**Files:**
- Check: `src/features/canvas/nodes/ImageEditNode.tsx`
- Check: `src/index.css`

- [x] **Step 1: Run targeted tests**

Run: `rtk npm exec vitest run src/features/canvas/nodes/ImageEditNode.test.tsx`

- [x] **Step 2: Run build**

Run: `rtk npm run build`

- [x] **Step 3: Sync cowork-flow status**

Update this plan status, validate the change, and keep task/change metadata pointing to this task and plan.

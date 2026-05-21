# Canvas Interaction Feedback Behavior Spec

## Image generation button feedback

When a user clicks Edit or Generate in an image generation node and the request is valid:

- The clicked action must enter a local pending state before any asynchronous diagnostic or gateway work can complete.
- Edit and Generate controls must be disabled while the local pending state is active.
- A second click during the pending state must not create another generated node or submit another generation job.
- If validation fails before submission, such as empty prompt, the controls must not enter pending state.
- If the submission setup fails, the pending state must clear so the user can retry.

## Connection handle usability

Canvas node connection handles must be easier to acquire with a pointer:

- The handle target should be larger than the previous 8px handle target.
- Existing left/right placement and accent styling must remain recognizable.
- The change should apply consistently to existing nodes that use React Flow handles.

## Acceptance

- A regression test proves repeated Generate clicks in `ImageEditNode` produce one generation submission.
- A regression test proves the local pending state disables both generation controls immediately after submission starts.
- A style or component assertion proves connection handles no longer use the old 8px target size.


# Canvas interaction feedback

## Goal

Make canvas generation and node connection interactions feel responsive and reliable.

## Requirements

- Image generation nodes immediately show a submitting state after a valid Edit or Generate click.
- Repeated clicks during submit setup cannot enqueue duplicate generation work.
- Node connection handles are easier to target with the mouse.
- Keep changes scoped to existing canvas frontend code and style conventions.

## Acceptance Criteria

- [ ] Rapidly clicking Generate in `ImageEditNode` creates/submits only one generation request.
- [ ] Generate/Edit buttons are disabled while the local submit state is active.
- [ ] Connection handles have a larger target than the current 8px size.
- [ ] Targeted tests pass.

## Technical Notes

- This is an L1 frontend behavior fix.
- Use local component state/ref for the submit-in-progress guard; do not promote it to global store.
- Prefer existing UI primitives and React Flow handle styling.


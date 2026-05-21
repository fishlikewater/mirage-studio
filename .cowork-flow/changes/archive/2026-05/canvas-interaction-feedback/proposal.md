# Canvas interaction feedback

## Why

Two canvas interactions currently feel unreliable:

- In an image generation node, clicking Edit or Generate can have a short delay before visible feedback appears. During that gap the user may click again and submit duplicate generation work.
- Node connection handles are visually small and hard to hit precisely, which makes connecting nodes feel fussy.

## What Changes

- Image generation actions should show an immediate local pending state after a valid click starts submitting.
- While that pending state is active, Edit and Generate should be disabled so repeated clicks cannot submit duplicate jobs.
- Canvas connection handles should have a larger, easier target while keeping the existing visual language.

## Scope

- Frontend canvas UI only.
- No backend protocol or generation payload contract changes.
- No redesign of node layout or edge routing.


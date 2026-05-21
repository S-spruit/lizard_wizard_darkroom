# MVP roadmap
in this document you can find the mvp and the roadmap for this software afterwards. It will be explained in phases

## phase 0: basic viewer
you should be able to do the following things:
- Browse thumbnails 
- Open RAW files
- Render previews
- Zoom in and out
- Move around

## phase 1: basic editing
You should be able to do the following things:
- non destructive editing; original RAW unharmed
- ajust exposure
- ajust contrast
- ajust white balance
- ajust tint/temp
- ajust saturation
- crop images

## phase 2: better basic editing
You should be able to do the following things:
- ajust hightlights
- ajust shadows
- view histogram
- ajust curves
- export to jpeg, png and tiff

## phase 3: much needed performance improvements
In this phase the focus will be on performance improvements. This phase might change over time or return on later points when new test reports and features release.
In this phase:
- tiled image rendering (instead of rendering the entire raw at once)
- multithreading (only if async wants to work)
- GPU accelleration
- caching

## phase 4: masking
You should be able to do:
- apply masks
- do local ajustments
- Save your changes as presets

## phase 5:
You should be able to do:
AI denoise

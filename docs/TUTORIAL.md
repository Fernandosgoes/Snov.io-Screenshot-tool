# Tutorial: Snov.io Screenshot tool

A guide to each tool: what it does, how to use it, and a keyboard shortcut when there is one.

> **Pending images:** the spots marked `[image: ...]` need a real capture of the app. The suggested file name for each one is in parentheses and should go in `docs/images/`.

## How to start a capture

1. Press **Shift + Backspace** (or the hotkey chosen in Settings). The screen dims and freezes.
2. Click and drag to select an area. When you release, the vertical tool bar appears to the right of the area and the horizontal action bar appears below it.
3. The label above the area shows its size in pixels.

`[image: selection with the size label and both bars (01-selection.png)]`

To start over, press **Esc** or click the **X** in the horizontal bar. Clicking outside the area does not cancel it, so you don't lose your selection by mistake.

## Vertical bar: tools

From top to bottom:

### 1. Eraser (Clear all)

Removes **all** annotations from the capture at once. It does not remove the selection or the screenshot itself. You can undo it with **Ctrl + Z**.

`[image: before and after using the eraser (02-eraser.png)]`

### 2. Move (cross)

Drags the **selected area** across the screen, without touching the annotations. Use it to reposition the crop.

- With the cross active, dragging inside the area moves the area.
- With any other tool, dragging inside the area draws.
- To resize, drag the small squares on the edges and corners.

`[image: area being moved with the cross (03-move.png)]`

### 3. Freehand (pen)

Free-hand drawing, like a marker. Use it to scribble, circle something or underline it.

- Shortcut: **P**
- Stroke width: set in Settings, or with the mouse wheel.

`[image: freehand stroke around a button (04-freehand.png)]`

### 4. Line

A straight line between two points. Hold **Shift** to snap to 45° angles.

- Shortcut: **L**

`[image: line pointing at a field (05-line.png)]`

### 5. Arrow

An arrow with a head, to point at something. Hold **Shift** to snap to 45°.

- Shortcut: **A**

`[image: arrow pointing at a UI element (06-arrow.png)]`

### 6. Rectangle

A rectangular outline to highlight an area. Hold **Shift** to draw a square.

- Shortcut: **R**

`[image: rectangle around a block of text (07-rectangle.png)]`

### 7. Ellipse

An oval outline. Hold **Shift** to draw a circle.

- Shortcut: **E**

`[image: ellipse around an icon (08-ellipse.png)]`

### 8. Highlighter

A translucent band, like a paper highlighter. The text underneath stays readable.

- Shortcut: **M**

`[image: part of a text highlighted in yellow (09-highlighter.png)]`

### 9. Text

A box for typing. Click where you want the text and start typing.

- Shortcut: **T**
- **Enter** confirms. **Shift + Enter** starts a new line. Clicking outside the box also confirms.
- The text sits on a white bubble with a purple border and square corners.
- Size: the mouse wheel changes the text size while you type.
- Font: Inter, embedded in the app.

`[image: text bubble on the capture (10-text.png)]`

#### Moving text you already placed

After you confirm a text, you can move it without retyping:

1. Hover over the **T** in the vertical bar. A move (cross) icon appears outside the bar, to its right.
2. Click the cross. The text tool stays highlighted.
3. Drag the text. It always stays inside the selected area.
4. Click the **T** again to go back to typing.

`[image: move-text icon showing next to the T (11-move-text.png)]`

### 10. Pixelate

Blurs the chosen region with a block effect, to hide sensitive information (names, emails, numbers).

- Shortcut: **B**

`[image: pixelated email address (12-pixelate.png)]`

### 11. Number (counter)

Places numbered markers (1, 2, 3…) to explain steps. Numbering adjusts by itself when you delete one.

- Shortcut: **N**
- Available colors: red, green, blue and purple.

`[image: steps 1, 2 and 3 numbered (13-number.png)]`

### 12. Color

The colored dot opens the color palette, to the left of the bar. Colors: yellow, blue, green, red, black, white and purple (Snov.io).

- Keys **1 to 6** pick the first six colors.
- The selected color applies to the next annotations.

`[image: palette open next to the bar (14-color.png)]`

### 13. Undo

Undoes the last action. Shortcut: **Ctrl + Z**. To redo, use **Ctrl + Y**.

`[image: undo arrow (15-undo.png)]`

## Horizontal bar: actions

### Copy

Copies the capture, with its annotations, to the clipboard. Then paste (**Ctrl + V**) anywhere. The capture closes right after.

- Shortcut: **Ctrl + C**

### Save

Opens the "Save as" dialog with the default format from Settings (JPEG or PNG). The extension you type in the file name sets the final format. The capture closes after saving.

- Shortcut: **Ctrl + S**

`[image: Save as dialog (16-save.png)]`

### Close (X)

Discards the capture without copying or saving it.

- Shortcut: **Esc**

## Deleting

- **A text:** use the move-text icon (next to the T), click the text and press **Delete**.
- **Everything at once:** use the eraser (first button in the vertical bar).
- **Last step:** **Ctrl + Z** undoes the last action.

## Tips

- **Move the selection without touching the drawings:** use the cross.
- **Draw precisely:** hold **Shift** with the line, arrow, rectangle and ellipse tools.
- **Don't lose a crop:** clicking outside the area does not cancel the selection.
- **Tool shortcuts** only work when you are not typing a text.

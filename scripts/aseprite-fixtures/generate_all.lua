-- PixelForge parity fixtures — generates aseprite-lua manifest entries.
-- Run: aseprite -b --script scripts/aseprite-fixtures/generate_all.lua

local function outDir()
  local d = os.getenv("PIXELFORGE_FIXTURES_DIR")
  if d and #d > 0 then return d end
  return "tests/parity/fixtures"
end

local function save(sprite, filename)
  local path = app.fs.joinPath(outDir(), filename)
  sprite:saveAs(path)
  print("wrote " .. path)
end

local function newIndexed(w, h, frames)
  local s = Sprite(w, h, ColorMode.INDEXED)
  app.activeSprite = s
  while #s.frames < frames do
    s:newFrame()
  end
  return s
end

local function newRgb(w, h, frames)
  local s = Sprite(w, h, ColorMode.RGB)
  app.activeSprite = s
  while #s.frames < frames do
    s:newFrame()
  end
  return s
end

local function newGray(w, h, frames)
  local s = Sprite(w, h, ColorMode.GRAYSCALE)
  app.activeSprite = s
  while #s.frames < frames do
    s:newFrame()
  end
  return s
end

local function drawPixel(layer, x, y, color)
  local cel = layer:cel(1)
  if cel then
    local img = cel.image
    img:drawPixel(x, y, color)
  end
end

local function drawLine(layer, x1, y1, x2, y2, color)
  local cel = layer:cel(1)
  if not cel then return end
  local img = cel.image
  app.useTool{
    tool="line",
    color=color,
    points={ Point(x1, y1), Point(x2, y2) },
    layer=layer,
    frameNumber=1
  }
end

-- CM: color modes
do
  local s = newRgb(32, 32, 1)
  local layer = s.layers[1]
  drawPixel(layer, 4, 4, Color{ r=255, g=0, b=0, a=255 })
  save(s, "mode_rgb_32.aseprite")
  s:close()
end

do
  local s = newGray(32, 32, 1)
  local layer = s.layers[1]
  drawPixel(layer, 8, 8, Color{ r=128, g=128, b=128, a=255 })
  save(s, "mode_gray_32.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "mode_indexed_16.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  save(s, "mode_indexed_256.aseprite")
  s:close()
end

do
  local s = newRgb(64, 64, 1)
  save(s, "mode_rgb_64.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  local pal = s.palettes[1]
  if pal then
    pal:setColor(1, Color{ r=0, g=0, b=255, a=255 })
  end
  save(s, "mode_indexed_32_palette.aseprite")
  s:close()
end

-- TL: timeline and tags
do
  local s = newIndexed(16, 16, 6)
  local tag = s:newTag(1, 4)
  tag.name = "walk"
  tag.aniDir = AniDir.FORWARD
  save(s, "tags_walk_cycle.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 4)
  local tag = s:newTag(1, 4)
  tag.name = "idle"
  tag.aniDir = AniDir.PING_PONG
  save(s, "tags_pingpong.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 4)
  local tag = s:newTag(1, 4)
  tag.name = "back"
  tag.aniDir = AniDir.REVERSE
  save(s, "tags_reverse.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 4)
  local tag = s:newTag(1, 3)
  tag.name = "attack"
  tag.aniDir = AniDir.FORWARD
  tag.repeats = 3
  save(s, "tags_repeat_3.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 4)
  s.frames[1].duration = 50
  s.frames[2].duration = 100
  s.frames[3].duration = 150
  s.frames[4].duration = 200
  save(s, "anim_4frame_durations.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 8)
  save(s, "anim_8frame.aseprite")
  s:close()
end

do
  local s = newRgb(16, 16, 1)
  local layer = s.layers[1]
  local cel = layer:cel(1)
  if cel then cel.opacity = 128 end
  save(s, "cel_opacity_rgb.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "cel_zindex.aseprite")
  s:close()
end

-- RT: linked cels, groups, slices, reference, user data
do
  local s = newIndexed(16, 16, 4)
  local layer = s.layers[1]
  local cel1 = layer:cel(1)
  if cel1 then
    for f = 2, 3 do
      s:newCel(layer, f, cel1.image)
    end
  end
  save(s, "linked_cels.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  s:newGroup()
  save(s, "layer_groups.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  local slice = s:newSlice(Rectangle(4, 4, 20, 20))
  slice.name = "panel"
  save(s, "slices_ninepatch.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  save(s, "reference_layer.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  s.data = "pixelforge-test-user-data"
  save(s, "user_data_sprite.aseprite")
  s:close()
end

-- MAP: tilemaps (basic indexed placeholders; full tilemap when API available)
do
  local s = newIndexed(32, 32, 1)
  save(s, "tilemap_manual.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "tilemap_16x16.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  s:newLayer()
  save(s, "tilemap_two_layers.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  save(s, "tilemap_auto_mode.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  save(s, "tilemap_stack_mode.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  save(s, "tilemap_mixed_pixel.aseprite")
  s:close()
end

-- EX: export-oriented animations
do
  local s = newIndexed(16, 16, 4)
  save(s, "anim_4frame.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 4)
  save(s, "sheet_strip_4.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 4)
  save(s, "sheet_grid_2x2.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 2)
  save(s, "export_indexed_anim.aseprite")
  s:close()
end

do
  local s = newRgb(16, 16, 1)
  save(s, "export_rgb_static.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 4)
  local tag = s:newTag(1, 4)
  tag.name = "run"
  save(s, "export_with_tags.aseprite")
  s:close()
end

-- OP: minimal tool strokes
do
  local s = newIndexed(16, 16, 1)
  local layer = s.layers[1]
  app.useTool{
    tool="pencil",
    color=Color{ r=0, g=0, b=0, a=255 },
    points={ Point(2, 2), Point(10, 10) },
    layer=layer,
    frameNumber=1
  }
  save(s, "pencil_stroke.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  local layer = s.layers[1]
  drawPixel(layer, 4, 4, Color{ r=255, g=0, b=0, a=255 })
  app.useTool{
    tool="paint_bucket",
    color=Color{ r=0, g=255, b=0, a=255 },
    points={ Point(4, 4) },
    layer=layer,
    frameNumber=1
  }
  save(s, "bucket_fill.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  local layer = s.layers[1]
  drawPixel(layer, 8, 8, Color{ r=255, g=255, b=0, a=255 })
  save(s, "rot_sprite.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  app.useTool{
    tool="line",
    color=Color{ r=0, g=0, b=0, a=255 },
    points={ Point(1, 1), Point(14, 14) },
    layer=s.layers[1],
    frameNumber=1
  }
  save(s, "line_tool.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  app.useTool{
    tool="rectangle",
    color=Color{ r=255, g=0, b=0, a=255 },
    points={ Point(2, 2), Point(12, 12) },
    layer=s.layers[1],
    frameNumber=1
  }
  save(s, "rect_filled.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  app.useTool{
    tool="ellipse",
    color=Color{ r=0, g=0, b=255, a=255 },
    points={ Point(2, 2), Point(12, 12) },
    layer=s.layers[1],
    frameNumber=1
  }
  save(s, "ellipse_tool.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  local layer = s.layers[1]
  drawPixel(layer, 5, 5, Color{ r=0, g=0, b=0, a=255 })
  app.useTool{
    tool="eraser",
    color=Color{ r=0, g=0, b=0, a=255 },
    points={ Point(5, 5) },
    layer=layer,
    frameNumber=1
  }
  save(s, "eraser_stroke.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "gradient_fill.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "spray_tool.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  local layer = s.layers[1]
  drawPixel(layer, 4, 4, Color{ r=255, g=0, b=0, a=255 })
  save(s, "magic_wand_select.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "marquee_rect.aseprite")
  s:close()
end

do
  local s = newIndexed(32, 32, 1)
  save(s, "text_tool.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "blur_tool.aseprite")
  s:close()
end

do
  local s = newIndexed(16, 16, 1)
  save(s, "contour_tool.aseprite")
  s:close()
end

print("generate_all.lua complete")

-- Virtual Resolution Library
push = require 'push'

WIDTH = 1280
HEIGHT = 720

VIRTUAL_WIDTH = 512
VIRTUAL_HEIGHT = 288

GROUND_IMG_HEIGHT = 16

local backgroundImg = love.graphics.newImage('background.png');
local groundImg = love.graphics.newImage('ground.png');

-- Initialize
function love.load()
  love.graphics.setDefaultFilter('nearest', 'nearest')
  love.window.setTitle('Flingo Bird ;P')
  push:setupScreen(
    VIRTUAL_WIDTH,
    VIRTUAL_HEIGHT,
    WIDTH,
    HEIGHT,
    {
      vsync = true,
      fullscreen = false,
      resizable = true,
    }
  )
end

function love.resize(w, h)
  push:resize(w, h)
end

-- User Input
function love.keypressed(key)
  if key == 'escape' then
    love.event.quit()
  end
end

-- Render everything
function love.draw()
  push:start()
  love.graphics.draw(backgroundImg, 0, 0)
  love.graphics.draw(groundImg, 0, VIRTUAL_HEIGHT - GROUND_IMG_HEIGHT)
  push:finish()
end

push = require('push')

WIDTH = 1280
HEIGHT = 720

VIRTUAL_WIDTH = 432
VIRTUAL_HEIGHT = 243

function love.load()
  love.graphics.setDefaultFilter('nearest', 'nearest')

  push:setupScreen(
    VIRTUAL_WIDTH,
    VIRTUAL_HEIGHT,
    WIDTH,
    HEIGHT,
    {
      fullscreen = false,
      resizable = false,
      vsync = true
    }
  )
end

function love.keypressed(key)
    -- keys can be accessed by string name
    if key == 'escape' then
        -- function LÖVE gives us to terminate application
        love.event.quit()
    end
end

function love.draw()
  -- begin rendering at virtual resolution
  push:apply('start')

  love.graphics.printf(
    'Hello Pong!',
    0,
    VIRTUAL_HEIGHT / 2 - 6, -- -6 is half height of default Text Height
    VIRTUAL_WIDTH,
    'center'
  )

  push:apply('end')
end
